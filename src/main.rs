// use modules
use clap::Parser;
use std::fs;
use std::path::Path;
// use std::process;
use tokio;

// define local modules
mod api;
mod auth;
mod catalog;
mod config;
mod image;
mod log;
mod manifests;

// use local modules
use api::schema::*;
use auth::credentials::*;
use catalog::rebuild::*;
use config::read::*;
use image::copy::*;
use log::logging::*;
use manifests::files::*;

// main entry point (use async)
#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let cfg = args.config.as_str().to_string();

    let log = &Logging {
        log_level: Level::DEBUG,
    };

    log.info(&format!("rust-operator-catalog-rebuild {} ", cfg));

    // Parse the config serde_yaml::FilterConfiguration.
    let config = load_file(cfg).unwrap();
    let filter_config = parse_yaml_config(config).unwrap();
    log.debug(&format!("{:#?}", filter_config.operators));

    // parse the config - iterate through each catalog
    let ir = parse_image_index(log, filter_config.catalog.clone());

    let manifest_json = get_manifest_json_file(ir.name.clone(), ir.version.clone());
    let working_dir_blobs = get_blobs_dir(ir.name.clone(), ir.version.clone());
    let working_dir_cache = get_cache_dir(ir.name.clone(), ir.version.clone());

    // check if the directory exists
    if !Path::new(&working_dir_blobs).exists() {
        let token = get_token(ir.registry.clone()).await;
        // use token to get manifest
        let manifest_url = get_image_manifest_url(ir.clone());
        let manifest = get_manifest(manifest_url.clone(), token.clone())
            .await
            .unwrap();

        // create the full path
        fs::create_dir_all(working_dir_blobs.clone()).expect("unable to create directory");
        fs::write(manifest_json, manifest.clone()).expect("unable to write file");
        let res = parse_json_manifest(manifest).unwrap();
        let blobs_url = get_blobs_url(ir.clone());
        // use a concurrent process to get related blobs
        get_blobs(
            log,
            blobs_url,
            token,
            res.fs_layers,
            working_dir_blobs.clone(),
        )
        .await;
        log.info("completed image index download");
    } else {
        log.info("catalog index exists - no further processing required");
    }
    // check if the cache directory exists
    if !Path::new(&working_dir_cache).exists() {
        // create the cache directory
        fs::create_dir_all(&working_dir_cache).expect("unable to create directory");
        untar_layers(log, working_dir_blobs.clone()).await;
        log.info("completed untar of layers");
    } else {
        log.info("cache exists - no further processing required");
    }

    // find the directory 'configs'
    let configs_dir = find_dir(log, working_dir_cache.clone(), "configs".to_string()).await;
    log.info(&format!(
        "full path for directory 'configs' {} ",
        &configs_dir
    ));

    // find the opm binary
    let opm = find_parent_dir(
        log,
        working_dir_cache.clone(),
        "/usr/bin/registry/opm".to_string(),
    )
    .await;

    log.info(&format!("full path for opm binary directory {} ", &opm));
    if configs_dir != "" && opm != "" {
        // rebuild the catalog
        rebuild_catalog(
            log,
            working_dir_cache.clone(),
            configs_dir,
            opm,
            filter_config.clone(),
        )
        .await;
    } else {
        log.error("configs directory and/or opm binary not found");
    }
}

// utility functions - get_manifest_json
fn get_manifest_json_file(name: String, version: String) -> String {
    let mut file = String::from("working-dir/");
    file.push_str(&name);
    file.push_str(&"/");
    file.push_str(&version);
    file.push_str(&"/");
    file.push_str(&"manifest.json");
    file
}

// get_blobs_dir
fn get_blobs_dir(name: String, version: String) -> String {
    let mut file = String::from("working-dir/");
    file.push_str(&name);
    file.push_str(&"/");
    file.push_str(&version);
    file.push_str(&"/");
    file.push_str(&"blobs/sha256/");
    file
}

// get_cache_dir
fn get_cache_dir(name: String, version: String) -> String {
    let mut file = String::from("working-dir/");
    file.push_str(&name);
    file.push_str(&"/");
    file.push_str(&version);
    file.push_str(&"/");
    file.push_str(&"cache");
    file
}
