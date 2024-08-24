use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DbConnectionInfo {
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

fn default_port() -> u16 {
    5432
}

impl DbConnectionInfo {
    pub fn into_url(self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(default)]
pub struct DbConnectionPoolConfig {
    pub max_connections: u32,
}

impl Default for DbConnectionPoolConfig {
    fn default() -> Self {
        Self { max_connections: 8 }
    }
}
