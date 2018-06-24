#[derive(Debug, Deserialize)]
pub struct RepoMD {
    pub revision: u64,
    pub data: Vec<RepoDataItem>,
}

#[derive(Debug, Deserialize)]
pub struct RepoDataLocation {
    pub href: String,
}

#[derive(Debug, Deserialize)]
pub struct RepoDataItem {
    #[serde(rename = "type")]
    pub repodatatype: String,
    pub checksum: String,
    #[serde(rename = "open-checksum")]
    pub open_checksum: Option<String>,
    pub location: RepoDataLocation,
    pub timestamp: u64,
    pub size: u64,
    #[serde(rename = "open-size")]
    pub open_size: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct PackageVersion {
    pub epoch: u32,
    pub ver: String,
    pub rel: String,
}

#[allow(dead_code)]
pub struct PackageId {
    pub name: String,
    pub epoch: u32,
    pub ver: String,
    pub rel: String,
    pub arch: String,
}
