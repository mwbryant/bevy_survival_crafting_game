use crate::prelude::PlaceHolderGraphics;
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use serde::Deserialize;

#[derive(Component, Inspectable)]
pub struct Pickupable {
    pub(crate) item: ItemType,
    pub(crate) drops: Option<WorldObject>,
}

#[derive(Debug, Inspectable, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum WorldObject {
    Item(ItemType),
    Tree,
    Stump,
    Sapling,
    DeadSapling,
}

#[derive(Inspectable, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize)]
pub enum ItemType {
    None,
    Flint,
    Axe,
    Twig,
    Grass,
    Wood,
    //FIXME Is actually not an item, is a world object!
    Fire,
}

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::spawn_flint)
            .add_startup_system(Self::spawn_sapling)
            .add_system(Self::update_graphics)
            .add_system(Self::sapling_regrowth);
        //FIXME I don't think this is working...
        if cfg!(debug_assertions) {
            app.register_type::<RegrowthTimer>()
                .register_inspectable::<WorldObject>()
                .register_inspectable::<ItemAndCount>()
                .register_inspectable::<Pickupable>();
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct RegrowthTimer {
    timer: Timer,
}

impl ItemsPlugin {
    //XXX is it ok to be adding/removing components (for performance i guess) or should anything that can regrow already have the timer
    fn sapling_regrowth(
        mut commands: Commands,
        mut sapling_query: Query<(Entity, &mut WorldObject, Option<&mut RegrowthTimer>)>,
        time: Res<Time>,
    ) {
        for (ent, mut sapling, timer) in sapling_query.iter_mut() {
            if *sapling != WorldObject::DeadSapling {
                continue;
            }
            if let Some(mut timer) = timer {
                timer.timer.tick(time.delta());
                if !timer.timer.finished() {
                    continue;
                }
                commands.entity(ent).remove::<RegrowthTimer>();
                *sapling = WorldObject::Sapling;
                //FIXME dont re add pickupable, track if can be picked up?
                //Idk how this work for renewables
                commands.entity(ent).insert(Pickupable {
                    item: ItemType::Twig,
                    drops: Some(WorldObject::DeadSapling),
                });
            } else {
                commands.entity(ent).insert(RegrowthTimer {
                    timer: Timer::from_seconds(1.0, false),
                });
            }
        }
    }

    fn update_graphics(
        mut to_update_query: Query<(&mut TextureAtlasSprite, &WorldObject), Changed<WorldObject>>,
        graphics: Res<PlaceHolderGraphics>,
    ) {
        for (mut sprite, world_object) in to_update_query.iter_mut() {
            sprite.index = *graphics
                .item_map
                .get(world_object)
                .expect(&format!("No graphic for object {:?}", world_object));
        }
    }

    fn spawn_sapling(mut commands: Commands, graphics: Res<PlaceHolderGraphics>) {
        spawn_world_object(
            &mut commands,
            &graphics,
            WorldObject::Sapling,
            Some(ItemType::Twig),
            Some(WorldObject::DeadSapling),
            Vec2::new(-0.3, 0.3),
        );
    }

    fn spawn_flint(mut commands: Commands, graphics: Res<PlaceHolderGraphics>) {
        spawn_world_object(
            &mut commands,
            &graphics,
            WorldObject::Item(ItemType::Flint),
            Some(ItemType::Flint),
            None,
            Vec2::new(0.40, 0.3),
        );
        spawn_world_object(
            &mut commands,
            &graphics,
            WorldObject::Item(ItemType::Flint),
            Some(ItemType::Flint),
            None,
            Vec2::new(0.3, 0.40),
        );
        spawn_world_object(
            &mut commands,
            &graphics,
            WorldObject::Item(ItemType::Flint),
            Some(ItemType::Flint),
            None,
            Vec2::new(0.3, 0.3),
        );
    }
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::None
    }
}

#[derive(Clone, Copy, Default, Inspectable)]
pub struct ItemAndCount {
    pub item: ItemType,
    pub count: usize,
}

impl std::fmt::Display for ItemAndCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x {:?}", self.count, self.item)
    }
}

fn spawn_world_object(
    commands: &mut Commands,
    graphics: &PlaceHolderGraphics,
    object_type: WorldObject,
    pickup: Option<ItemType>,
    drops: Option<WorldObject>,
    position: Vec2,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(
        *graphics
            .item_map
            .get(&object_type)
            .expect(&format!("No graphic for object {:?}", object_type)),
    );
    sprite.custom_size = Some(Vec2::splat(0.1));
    let item = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: graphics.texture_atlas.clone(),
            transform: Transform {
                translation: position.extend(0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("GroundItem"))
        .insert(object_type)
        .id();

    if let Some(pickup) = pickup {
        commands.entity(item).insert(Pickupable {
            item: pickup,
            drops,
        });
    }
    item
}
