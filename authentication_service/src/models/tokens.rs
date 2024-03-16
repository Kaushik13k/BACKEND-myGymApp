use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub email: String,
    pub username: String,
    pub exp: usize,
}
