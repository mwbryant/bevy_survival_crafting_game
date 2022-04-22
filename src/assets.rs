use std::fs;

use bevy::prelude::*;
use bevy::reflect::erased_serde::private::serde::Deserialize;
use bevy::sprite::Anchor;
use bevy::utils::HashMap;

use crate::item::WorldObject;
use ron::de::from_str;

pub struct GameAssetsPlugin;

#[derive(Default, Clone, Copy, Debug, Reflect, Deserialize)]
pub struct MyRect {
    pub pos: (f32, f32),
    pub size: (f32, f32),
    pub anchor: Option<(f32, f32)>,
}

impl MyRect {
    pub fn new(pos: (f32, f32), size: (f32, f32)) -> Self {
        Self {
            pos,
            size,
            anchor: None,
        }
    }

    pub fn to_atlas_rect(self) -> bevy::sprite::Rect {
        bevy::sprite::Rect {
            //A tiny amount is clipped off the sides of the rectangle
            //to stop contents of other sprites from bleeding through
            min: Vec2::new(self.pos.0 + 0.15, self.pos.1 + 0.15),
            max: Vec2::new(
                self.pos.0 + self.size.0 - 0.15,
                self.pos.1 + self.size.1 - 0.15,
            ),
        }
    }
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
//Three pixels divided by half of the screen height
pub const PIXEL_SIZE: f32 = 3. / 450.;

pub struct Graphics {
    pub texture_atlas: Handle<TextureAtlas>,
    pub player_index: usize,
    pub box_index: usize,
    pub item_map: HashMap<WorldObject, TextureAtlasSprite>,
}

impl GameAssetsPlugin {
    fn load_graphics(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut texture_assets: ResMut<Assets<TextureAtlas>>,
    ) {
        let image_handle = assets.load("bevy_survival_sprites.png");
        let sprite_desc = fs::read_to_string("assets/sprites_desc.ron").unwrap();

        let sprite_desc: GraphicsDesc = from_str(&sprite_desc).unwrap_or_else(|e| {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        });

        let mut atlas = TextureAtlas::new_empty(image_handle, Vec2::splat(256.0));

        let player_index = atlas.add_texture(MyRect::new((0., 0.), (32., 32.)).to_atlas_rect());

        let mut item_map = HashMap::default();
        for (item, rect) in sprite_desc.map.iter() {
            println!("Found graphic {:?}", item);
            let mut sprite = TextureAtlasSprite::new(atlas.add_texture(rect.to_atlas_rect()));

            //Set the size to be proportional to the source rectangle
            sprite.custom_size = Some(Vec2::new(
                rect.size.0 * PIXEL_SIZE,
                rect.size.1 * PIXEL_SIZE,
            ));

            //Position the sprite anchor if one is defined
            if let Some(anchor) = rect.anchor {
                sprite.anchor = Anchor::Custom(Vec2::new(
                    anchor.0 / rect.size.0 - 0.5,
                    0.5 - anchor.1 / rect.size.1,
                ));
            };

            item_map.insert(*item, sprite);
        }

        let box_index = atlas.add_texture(MyRect::new((0., 32.), (32., 32.)).to_atlas_rect());

        let atlas_handle = texture_assets.add(atlas);

        commands.insert_resource(Graphics {
            texture_atlas: atlas_handle,
            player_index,
            box_index,
            item_map,
        });
    }
}
