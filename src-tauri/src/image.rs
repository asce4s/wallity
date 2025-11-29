use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    pub name: String,
    pub img_path: String,
    pub thumbnail_path: String,
}
