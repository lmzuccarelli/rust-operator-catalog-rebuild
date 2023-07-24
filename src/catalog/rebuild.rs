use crate::api::schema::*;
use crate::auth::credentials::*;
use crate::config::read::*;
use crate::log::logging::*;
use sha256::digest;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::{fs, io};
use tar::Builder;

// rebuild catalog
pub async fn rebuild_catalog(
    log: &Logging,
    working_dir: String,
    configs_dir: String,
    opm: String,
    filter: FilterConfig,
) {
    // create a temp dir
    let tmp_dir = working_dir.to_string() + "/tmp-catalog/";
    let res = fs::create_dir_all(tmp_dir.clone() + &"configs");
    // iterate through each filtered operator and copy them to the temp catalog
    if res.is_ok() {
        for pkg in filter.operators {
            let src = configs_dir.to_string() + &"/" + &pkg.name;
            let dst = tmp_dir.to_string() + &"configs/" + &pkg.name;
            if copy_dir_all(src.clone(), dst.clone()).is_ok() {
                log.mid(&format!("  from {}", src));
                log.lo(&format!("  to {}", dst));
            } else {
                log.error(&format!("error copying {} {}", src, dst));
            }
        }
        // we can now execute the opm binary
        let out = Command::new(opm.clone() + "/opm")
            .arg("serve")
            .arg(tmp_dir.clone() + &"configs")
            .arg("--cache-dir")
            .arg(tmp_dir.clone() + "tmp/")
            .arg("--cache-only")
            .output()
            .expect("failed to execute process");
        let error = String::from_utf8(out.stderr).unwrap();
        log.trace(&format!("results {:#?}", error));

        // use tar crate
        let tarfile = File::create(working_dir.clone() + "/tmp-catalog.tar").unwrap();
        let mut tar = Builder::new(tarfile);
        let res = tar.append_dir_all(".", tmp_dir.clone()).unwrap();
        log.trace(&format!("results {:#?} ", res));

        // rename the tmp catalog
        let bytes = std::fs::read(working_dir.clone() + &"/tmp-catalog.tar").unwrap();
        let hash = digest(&bytes);
        log.ex(&format!("hash {}", hash));
        let base_dir = working_dir.split("/cache").nth(0).unwrap();
        fs::rename(
            working_dir.clone() + &"/tmp-catalog.tar",
            base_dir.clone().to_owned() + "/blobs/sha256/" + &hash,
        )
        .unwrap();
        fs::remove_dir_all(working_dir.clone() + &"/tmp-catalog").unwrap();
        log.hi("temp-catalog directory removed");

        // update the manifest
        let manifest_content = load_file(base_dir.clone().to_string() + "/manifest.json").unwrap();
        let mut manifest = parse_json_manifest(manifest_content).unwrap();
        // get the 6 diget sha from the configs dir
        let sha = configs_dir
            .split("cache/")
            .nth(1)
            .unwrap()
            .split("/")
            .nth(0)
            .unwrap();

        let mut val = 0;
        let layers = manifest.clone();
        for (index, layer) in layers.fs_layers.iter().enumerate() {
            log.trace(&format!("blog layer sha256 {}", layer.blob_sum));
            if layer.blob_sum.contains(sha.clone()) {
                log.hi(&format!("found layer {}", layer.blob_sum));
                val = index;
            }
        }
        // move manifest.json -> manifest-old.json
        fs::rename(
            base_dir.clone().to_string() + "/manifest.json",
            base_dir.clone().to_string() + "/manifest-old.json",
        )
        .unwrap();
        log.debug("manifest backup created");

        manifest.fs_layers[val].blob_sum = "sha256:".to_string() + &hash;
        let mut file =
            std::fs::File::create(base_dir.clone().to_string() + "/manifest.json").unwrap();
        serde_json::to_writer_pretty(&mut file, &manifest).unwrap();
        log.debug(&format!("manifest created and updated with hash {}", hash));

        // This step is not necessary, we have the blobs on disk
        // the containers mirror function uses the folder and manifest.json or index.json
        // looks in the blobs/sha256 directory and then mirrors the image

        // finally create a new catalog-index
        let current_index = filter
            .catalog
            .to_string()
            .split("/")
            .nth(2)
            .unwrap()
            .to_string();
        log.debug(&format!("current index {}", current_index.clone()));
        let new_index =
            File::create(base_dir.clone().to_string() + "/" + &current_index.clone() + "-rebuild")
                .unwrap();
        let mut tar = Builder::new(new_index);
        tar.append_dir_all(".", base_dir.clone().to_string() + "/blobs")
            .unwrap();
        let mut f = File::open(base_dir.clone().to_string() + "/manifest.json").unwrap();
        tar.append_file("manifest.json", &mut f).unwrap();
        log.debug(&format!(
            "new catalog created : {} ",
            current_index.clone() + "-rebuild"
        ));
    } else {
        log.error(&format!("error creating dir {:#?}", res));
    }
}

// copy all contents
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
