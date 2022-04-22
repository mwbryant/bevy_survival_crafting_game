use crate::prelude::{PlaceHolderGraphics};
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
    Grass,
    PluckedGrass,
    GrowingTree,
}

impl WorldObject {
    pub fn spawn(
        self,
        commands: &mut Commands,
        graphics: &PlaceHolderGraphics,
        position: Vec2,
    ) -> Entity {
        let sprite = graphics
        .item_map
        .get(&self)
        .expect(&format!("No graphic for object {:?}", self))
        .clone();

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
            .insert(self)
            .id();

        if let Some(pickup) = self.as_pickup() {
            commands.entity(item).insert(pickup);
        }

        if self.grows_into().is_some() {
            commands.entity(item).insert(GrowthTimer {
                timer: Timer::from_seconds(3.0, false),
            });
        }

        item
    }

    pub fn grow(
        self,
        commands: &mut Commands,
        graphics: &PlaceHolderGraphics,
        ent: Entity,
        transform: &Transform
    ) -> Entity {
        if let Some(new_object) = self.grows_into() {
            commands.entity(ent).despawn_recursive();
            new_object.spawn(commands, graphics, transform.translation.truncate())
            //println!("{:?} grew into a beautiful {:?}", self, self.grows_into());
        } else {
            ent
        }
    }

    pub fn grows_into(&self) -> Option<WorldObject> {
        match self {
            WorldObject::DeadSapling => Some(WorldObject::Sapling),
            WorldObject::PluckedGrass => Some(WorldObject::Grass),
            WorldObject::GrowingTree => Some(WorldObject::Tree),
            _ => None
        }
    }

    pub fn as_pickup(&self) -> Option<Pickupable> {
        match self {
            WorldObject::Sapling => Some(Pickupable { item: ItemType::Twig, drops: Some(WorldObject::DeadSapling) }),
            WorldObject::Grass => Some(Pickupable { item: ItemType::Grass, drops: Some(WorldObject::PluckedGrass) }),
            WorldObject::Tree => Some(Pickupable { item: ItemType::Wood, drops: Some(WorldObject::Stump) }),
            WorldObject::Item(item) => Some(Pickupable { item: *item, drops: None }),
            _ => None
        }
    }
}

impl Default for WorldObject {
    fn default() -> Self {
        WorldObject::Item(ItemType::None)
    }
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
        app.add_startup_system(Self::spawn_test_objects)
            .add_system(Self::update_graphics)
            .add_system(Self::world_object_growth);
        //FIXME I don't think this is working...
        if cfg!(debug_assertions) {
            app.register_type::<GrowthTimer>()
                .register_inspectable::<WorldObject>()
                .register_inspectable::<ItemAndCount>()
                .register_inspectable::<Pickupable>();
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct GrowthTimer {
    timer: Timer,
}

impl ItemsPlugin {
    fn world_object_growth(
        mut commands: Commands,
        time: Res<Time>,
        graphics: Res<PlaceHolderGraphics>,
        mut growable_query: Query<(Entity, &Transform, &WorldObject, Option<&mut GrowthTimer>)>,
    ) {
        for (ent, transform, world_object, regrowth_timer) in growable_query.iter_mut() {
            if let Some(mut timer) = regrowth_timer {
                timer.timer.tick(time.delta());
                if !timer.timer.finished() {
                    continue;
                }

                world_object.grow(&mut commands, &graphics, ent, transform);
            }
        }
    }

    fn update_graphics(
        mut to_update_query: Query<(&mut TextureAtlasSprite, &WorldObject), Changed<WorldObject>>,
        graphics: Res<PlaceHolderGraphics>,
    ) {
        for (mut sprite, world_object) in to_update_query.iter_mut() {
            sprite.clone_from(
                graphics
                .item_map
                .get(world_object)
                .expect(&format!("No graphic for object {:?}", world_object))
            );
        }
    }

    fn spawn_test_objects(mut commands: Commands, graphics: Res<PlaceHolderGraphics>) {
        WorldObject::Sapling.spawn(
            &mut commands,
            &graphics,
            Vec2::new(-0.6, 0.6),
        );
        WorldObject::Sapling.spawn(
            &mut commands,
            &graphics,
            Vec2::new(-0.6, 0.3),
        );
        WorldObject::Sapling.spawn(
            &mut commands,
            &graphics,
            Vec2::new(-0.3, 0.6),
        );
        WorldObject::Sapling.spawn(
            &mut commands,
            &graphics,
            Vec2::new(-0.3, 0.3),
        );

        WorldObject::Grass.spawn(
            &mut commands,
            &graphics,
            Vec2::new(0.6, -0.6),
        );
        WorldObject::Grass.spawn(
            &mut commands,
            &graphics,
            Vec2::new(0.6, -0.3),
        );
        WorldObject::Grass.spawn(
            &mut commands,
            &graphics,
            Vec2::new(0.3, -0.6),
        );
        WorldObject::Grass.spawn(
            &mut commands,
            &graphics,
            Vec2::new(0.3, -0.3),
        );

        WorldObject::Tree.spawn(
            &mut commands,
            &graphics,
            Vec2::new(-0.6, -0.6),
        );
        WorldObject::Tree.spawn(
            &mut commands,
            &graphics,
            Vec2::new(-0.6, -0.3),
        );
        WorldObject::Tree.spawn(
            &mut commands,
            &graphics,
            Vec2::new(-0.3, -0.6),
        );
        WorldObject::Tree.spawn(
            &mut commands,
            &graphics,
            Vec2::new(-0.3, -0.3),
        );

        WorldObject::Item(ItemType::Flint).spawn(
            &mut commands,
            &graphics,
            Vec2::new(0.4, 0.4),
        );
        WorldObject::Item(ItemType::Flint).spawn(
            &mut commands,
            &graphics,
            Vec2::new(0.4, 0.3),
        );
        WorldObject::Item(ItemType::Flint).spawn(
            &mut commands,
            &graphics,
            Vec2::new(0.3, 0.4),
        );
        WorldObject::Item(ItemType::Flint).spawn(
            &mut commands,
            &graphics,
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
