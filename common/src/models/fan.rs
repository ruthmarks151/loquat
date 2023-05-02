use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Fan {
    pub id: String,
    pub name: String,
}
