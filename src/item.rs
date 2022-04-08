use crate::prelude::PlaceHolderGraphics;
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::spawn_items)
            .register_inspectable::<Pickupable>();
    }
}

impl ItemsPlugin {
    fn spawn_items(mut commands: Commands, graphics: Res<PlaceHolderGraphics>) {
        spawn_item(
            &mut commands,
            &graphics,
            ItemType::Flint,
            Vec2::new(0.3, 0.3),
        );
        spawn_item(
            &mut commands,
            &graphics,
            ItemType::Flint,
            Vec2::new(0.8, 0.4),
        );
        spawn_item(
            &mut commands,
            &graphics,
            ItemType::Flint,
            Vec2::new(-0.6, 0.35),
        );
        spawn_item(
            &mut commands,
            &graphics,
            ItemType::Flint,
            Vec2::new(0.1, 0.55),
        );
        spawn_item(
            &mut commands,
            &graphics,
            ItemType::Twig,
            Vec2::new(-0.3, -0.3),
        );
        spawn_item(
            &mut commands,
            &graphics,
            ItemType::Twig,
            Vec2::new(-0.1, -0.4),
        );
        spawn_item(
            &mut commands,
            &graphics,
            ItemType::Twig,
            Vec2::new(-0.8, -0.1),
        );
    }
}

#[derive(Component, Inspectable)]
pub struct Pickupable {
    pub(crate) item: ItemType,
}

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ItemType {
    None,
    Flint,
    Axe,
    Twig,
    Grass,
    Wood,
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::None
    }
}

#[derive(Clone, Copy)]
pub struct ItemAndCount {
    pub item: ItemType,
    pub count: usize,
}

fn spawn_item(
    commands: &mut Commands,
    graphics: &PlaceHolderGraphics,
    to_spawn: ItemType,
    position: Vec2,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(
        *graphics
            .item_map
            .get(&to_spawn)
            .expect("No graphic for item"),
    );
    sprite.custom_size = Some(Vec2::splat(0.1));
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: graphics.texture_atlas.clone(),
            transform: Transform {
                translation: position.extend(0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Pickupable { item: to_spawn })
        .insert(Name::new("GroundItem"))
        .id()
}
