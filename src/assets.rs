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

fn create_rect(a: (f32, f32), b: (f32, f32)) -> bevy::sprite::Rect {
    bevy::sprite::Rect {
        min: Vec2::new(a.0, a.1),
        max: Vec2::new(b.0, b.1),
    }
}

impl GameAssetsPlugin {
    fn load_graphics(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut texture_assets: ResMut<Assets<TextureAtlas>>,
    ) {
        let image_handle = assets.load("placeholder.png");
        let sprite_desc = fs::read_to_string("assets/placeholder_desc.ron").unwrap();

        let sprite_desc: GraphicsDesc = from_str(&sprite_desc).unwrap_or_else(|e| {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        });

        let mut atlas = TextureAtlas::new_empty(image_handle, Vec2::splat(256.0));

        let player_index = atlas.add_texture(create_rect((0., 0.), (32., 32.)));

        let mut item_map = HashMap::default();
        for (item, rect) in sprite_desc.map.iter() {
            println!("Found graphic {:?}", item);
            let index = atlas.add_texture(bevy::sprite::Rect {
                min: rect.min,
                max: rect.max,
            });
            item_map.insert(*item, index);
        }

        let flint_index = atlas.add_texture(create_rect((34., 0.), (49., 16.)));
        let grass_index = atlas.add_texture(create_rect((50., 0.), (65., 16.)));
        let axe_index = atlas.add_texture(create_rect((34., 18.), (49., 33.)));
        let wood_index = atlas.add_texture(create_rect((34., 34.), (49., 49.)));

        item_map.insert(WorldObject::Item(ItemType::Flint), flint_index);
        item_map.insert(WorldObject::Item(ItemType::Grass), grass_index);
        item_map.insert(WorldObject::Item(ItemType::Axe), axe_index);
        item_map.insert(WorldObject::Item(ItemType::Wood), wood_index);

        let box_index = atlas.add_texture(create_rect((0., 34.), (32., 64.)));

        let atlas_handle = texture_assets.add(atlas);

        commands.insert_resource(PlaceHolderGraphics {
            texture_atlas: atlas_handle,
            player_index,
            box_index,
            item_map,
        });
    }
}
