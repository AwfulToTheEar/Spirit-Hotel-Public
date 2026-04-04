use crate::assets::prelude::{RawInventoryItem, RawInventoryManifest};
use crate::components::prelude::InventoryItem;
use bevy::prelude::*;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::{Manifest, ManifestFormat};
use std::collections::HashMap;
use std::convert::Infallible;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct InventoryManifest {
    pub items: HashMap<Id<InventoryItem>, InventoryItem>,
    pub combinations: HashMap<(Id<InventoryItem>, Id<InventoryItem>), Id<InventoryItem>>,
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl Manifest for InventoryManifest {
    type RawManifest = RawInventoryManifest;
    type RawItem = RawInventoryItem;
    type Item = InventoryItem;
    type ConversionError = Infallible;

    const FORMAT: ManifestFormat = ManifestFormat::Ron;
    fn from_raw_manifest(
        raw_manifest: Self::RawManifest,
        world: &mut World,
    ) -> std::result::Result<Self, Self::ConversionError> {
        let asset_server = world.resource::<AssetServer>().clone();
        let mut texture_atlas_layout_assets = world.resource_mut::<Assets<TextureAtlasLayout>>();

        let texture_atlas_layout = TextureAtlasLayout::from_grid(
            raw_manifest.layout.tile_size,
            raw_manifest.layout.columns,
            raw_manifest.layout.rows,
            raw_manifest.layout.padding,
            raw_manifest.layout.offset,
        );

        let layout = texture_atlas_layout_assets.add(texture_atlas_layout);

        let texture = asset_server.load(raw_manifest.texture);

        let items: HashMap<_, _> = raw_manifest
            .items
            .into_iter()
            .map(|raw_item| {
                let id = Id::from_name(&raw_item.id);

                let item = InventoryItem {
                    name: raw_item.name,
                    description: raw_item.description,
                    index: (raw_item.position.x * raw_manifest.layout.rows + raw_item.position.y)
                        as usize,
                };

                (id, item)
            })
            .collect();

        let combinations = raw_manifest
            .combinations
            .into_iter()
            .map(|((input_a, input_b), result)| {
                (
                    (Id::from_name(&input_a), Id::from_name(&input_b)),
                    Id::from_name(&result),
                )
            })
            .collect::<HashMap<_, _>>();

        Ok(InventoryManifest {
            items,
            combinations,
            texture,
            layout,
        })
    }

    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.items.get(&id)
    }
}
