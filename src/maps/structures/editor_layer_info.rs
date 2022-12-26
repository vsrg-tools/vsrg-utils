use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct EditorLayerInfo {
    pub name: String,
    pub hidden: bool,
    pub color_rgb: String,
}
