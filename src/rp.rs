use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoteConfig {
    pub name: String,
    pub fetch: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepositoryConfig {
    pub name: String,
    pub remote: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpConfig {
    pub remotes: Option<Vec<RemoteConfig>>,
    pub repositories: Vec<RepositoryConfig>,
}

pub fn get_remote<'a>(config: &'a RpConfig, remote_name: &'a str) -> Option<&'a str> {
    config.remotes.as_ref()?
        .iter()
        .find(|remote| remote.name == remote_name)
        .map(|remote| remote.fetch.as_str())
}
