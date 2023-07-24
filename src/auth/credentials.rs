use crate::api::schema::*;
use crate::Path;
use base64::{engine::general_purpose, Engine as _};
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

// read the credentials from set path (see podman credential reference)
pub fn get_credentials() -> Result<String, Box<dyn std::error::Error>> {
    // Create a path to the desired file
    // using $XDG_RUNTIME_DIR envar
    let u = match env::var_os("XDG_RUNTIME_DIR") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$XDG_RUNTIME_DIR is not set"),
    };
    let binding = &(u.to_owned() + "/containers/auth.json");
    let path = Path::new(binding);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&binding) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

// parse the json credentials to a struct
pub fn parse_json_creds(data: String) -> Result<String, Box<dyn std::error::Error>> {
    // Parse the string of data into serde_json::Root.
    let creds: Root = serde_json::from_str(&data)?;
    Ok(creds.auths.registry_redhat_io.auth)
}

// parse the json from the api call
pub fn parse_json_token(data: String) -> Result<String, Box<dyn std::error::Error>> {
    // Parse the string of data into serde_json::Token.
    let root: Token = serde_json::from_str(&data)?;
    Ok(root.access_token)
}

// parse the manifest json
pub fn parse_json_manifest(data: String) -> Result<ManifestSchema, Box<dyn std::error::Error>> {
    // Parse the string of data into serde_json::ManifestSchema.
    let root: ManifestSchema = serde_json::from_str(&data)?;
    Ok(root)
}

// async api call with basic auth
pub async fn get_auth_json(
    url: String,
    user: String,
    password: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let pwd: Option<String> = Some(password);
    let body = client
        .get(url)
        .basic_auth(user, pwd)
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}

// process all relative functions in this module to actaully get the token
pub async fn get_token(name: String) -> String {
    let token_url = match name.as_str() {
        "registry.redhat.io" => "https://sso.redhat.com/auth/realms/rhcc/protocol/redhat-docker-v2/auth?service=docker-registry&client_id=curl&scope=repository:rhel:pull".to_string(),
        &_ => "none".to_string(),
    };
    // get creds from $XDG_RUNTIME_DIR
    let creds = get_credentials().unwrap();
    // parse the json data
    let rhauth = parse_json_creds(creds).unwrap();
    // decode to base64
    let bytes = general_purpose::STANDARD.decode(rhauth).unwrap();

    let s = match str::from_utf8(&bytes) {
        Ok(v) => v,
        Err(e) => panic!("ERROR: invalid UTF-8 sequence: {}", e),
    };
    // get user and password form json
    let user = s.split(":").nth(0).unwrap();
    let pwd = s.split(":").nth(1).unwrap();
    // call the realm url to get a token with the creds
    let res = get_auth_json(token_url, user.to_string(), pwd.to_string())
        .await
        .unwrap();
    // if all goes well we should have a valid token
    let token = parse_json_token(res).unwrap();
    token
}
