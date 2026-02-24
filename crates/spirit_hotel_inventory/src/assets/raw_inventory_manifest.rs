use std::collections::HashMap;
use std::path::PathBuf;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Asset, TypePath, Serialize, Deserialize, PartialEq)]
pub struct RawInventoryManifest {
    pub items: Vec<RawInventoryItem>,
    pub combinations: HashMap<(String, String), String>,
    pub texture: PathBuf,
    pub layout: InventoryItemTextureAtlasLayout,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RawInventoryItem {
    pub id: String,
    pub position: UVec2,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InventoryItemTextureAtlasLayout {
    pub tile_size: UVec2,
    pub rows: u32,
    pub columns: u32,
    pub padding: Option<UVec2>,
    pub offset: Option<UVec2>,
}
