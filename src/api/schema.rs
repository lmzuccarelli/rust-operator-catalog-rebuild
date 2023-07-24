// module api
use clap::Parser;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestSchema {
    pub tag: String,
    pub name: String,
    pub architecture: String,
    pub schema_version: i64,
    pub history: Vec<History>,
    pub fs_layers: Vec<FsLayer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    #[serde(rename = "v1Compatibility")]
    pub v1compatibility: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FsLayer {
    pub blob_sum: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub token: String,
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
    #[serde(rename = "issued_at")]
    pub issued_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub auths: Auths,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Auths {
    #[serde(rename = "cloud.openshift.com")]
    pub cloud_openshift_com: CloudOpenshiftCom,
    #[serde(rename = "quay.io")]
    pub quay_io: QuayIo,
    #[serde(rename = "registry.connect.redhat.com")]
    pub registry_connect_redhat_com: RegistryConnectRedhatCom,
    #[serde(rename = "registry.redhat.io")]
    pub registry_redhat_io: RegistryRedhatIo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudOpenshiftCom {
    pub auth: String,
    pub email: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuayIo {
    pub auth: String,
    pub email: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryConnectRedhatCom {
    pub auth: String,
    pub email: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryRedhatIo {
    pub auth: String,
    pub email: Option<String>,
}

/// rust-container-tool cli struct
#[derive(Parser, Debug)]
#[command(name = "rust-operator-catalog-rebuild")]
#[command(author = "Luigi Mario Zuccarelli <luzuccar@redhat.com>")]
#[command(version = "0.0.1")]
#[command(about = "Used to rebuild a catalog for a filtered set of operators", long_about = None)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// catalog to rebuild
    #[arg(short, long, value_name = "config", default_value = "")]
    pub config: String,
}

/// config schema
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterConfig {
    #[serde(rename = "kind")]
    pub kind: String,

    #[serde(rename = "apiVersion")]
    pub api_version: String,

    #[serde(rename = "catalog")]
    pub catalog: String,

    #[serde(rename = "packages")]
    pub operators: Vec<Operator>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Operator {
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedImage {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "image")]
    pub image: String,
}

// ImageReference
#[derive(Debug, Clone)]
pub struct ImageReference {
    pub registry: String,
    pub namespace: String,
    pub name: String,
    pub version: String,
}
