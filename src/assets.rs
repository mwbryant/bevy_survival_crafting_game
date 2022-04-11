use std::fs;

use bevy::prelude::*;
use bevy::reflect::erased_serde::private::serde::Deserialize;
use bevy::utils::HashMap;

use crate::item::WorldObject;
use crate::prelude::ItemType;
use ron::de::from_str;

pub struct GameAssetsPlugin;

#[derive(Default, Clone, Copy, Debug, Reflect, Deserialize)]
pub struct MyRect {
    pub min: Vec2,
    pub max: Vec2,
}

#[derive(Deserialize)]
pub struct GraphicsDesc {
    map: HashMap<WorldObject, MyRect>,
}

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            Self::load_graphics.label("graphics"),
        );
    }
}

pub const RESOLUTION: f32 = 16.0 / 9.0;

pub struct PlaceHolderGraphics {
    pub texture_atlas: Handle<TextureAtlas>,
    pub player_index: usize,
    pub box_index: usize,
    pub item_map: HashMap<WorldObject, usize>,
}

impl GameAssetsPlugin {
    fn load_graphics(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut texture_assets: ResMut<Assets<TextureAtlas>>,
    ) {
        let image_handle = assets.load("placeholder.png");
        let sprite_desc = fs::read_to_string("assets/placeholder_desc.ron").unwrap();

        let sprite_desc: GraphicsDesc = match from_str(&sprite_desc) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);
                std::process::exit(1);
            }
        };
        let mut atlas = TextureAtlas::new_empty(image_handle, Vec2::splat(256.0));

        let player_index = atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::splat(0.0),
            max: Vec2::splat(32.0),
        });

        let mut item_map = HashMap::default();
        for (item, rect) in sprite_desc.map.iter() {
            println!("Found graphic {:?}", item);
            let index = atlas.add_texture(bevy::sprite::Rect {
                min: rect.min,
                max: rect.max,
            });
            item_map.insert(*item, index);
        }

        let flint_index = atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::new(34.0, 0.0),
            max: Vec2::new(49.0, 16.0),
        });
        let grass_index = atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::new(50.0, 0.0),
            max: Vec2::new(65.0, 16.0),
        });

        let axe_index = atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::new(34.0, 18.0),
            max: Vec2::new(49.0, 33.0),
        });

        let twigs_index = atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::new(50.0, 18.0),
            max: Vec2::new(65.0, 33.0),
        });

        let wood_index = atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::new(34.0, 34.0),
            max: Vec2::new(49.0, 49.0),
        });

        item_map.insert(WorldObject::Item(ItemType::Flint), flint_index);
        item_map.insert(WorldObject::Item(ItemType::Grass), grass_index);
        item_map.insert(WorldObject::Item(ItemType::Axe), axe_index);
        item_map.insert(WorldObject::Item(ItemType::Wood), wood_index);

        let box_index = atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::new(0.0, 34.0),
            max: Vec2::new(32.0, 64.0),
        });

        let atlas_handle = texture_assets.add(atlas);

        commands.insert_resource(PlaceHolderGraphics {
            texture_atlas: atlas_handle,
            player_index: player_index,
            box_index: box_index,
            item_map: item_map,
        });
    }
}
