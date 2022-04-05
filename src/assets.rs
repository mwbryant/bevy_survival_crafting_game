use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::prelude::ItemType;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup,
                                         Self::load_graphics.label("graphics"));
    }
}

pub const RESOLUTION: f32 = 16.0 / 9.0;

pub struct PlaceHolderGraphics {
    pub texture_atlas: Handle<TextureAtlas>,
    pub player_index: usize,
    pub box_index: usize,
    pub item_map: HashMap<ItemType, usize>,
}

impl GameAssetsPlugin {
    fn load_graphics(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut texture_assets: ResMut<Assets<TextureAtlas>>,
    ) {
        let image_handle = assets.load("placeholder.png");
        let mut atlas = TextureAtlas::new_empty(image_handle, Vec2::splat(256.0));
        let player_index = atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::splat(0.0),
            max: Vec2::splat(32.0),
        });

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

        let mut item_map = HashMap::default();
        item_map.insert(ItemType::Flint, flint_index);
        item_map.insert(ItemType::Grass, grass_index);
        item_map.insert(ItemType::Twig, twigs_index);
        item_map.insert(ItemType::Axe, axe_index);
        item_map.insert(ItemType::Wood, wood_index);

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
