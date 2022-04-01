#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
use bevy::{prelude::*, render::camera::ScalingMode, utils::HashMap};
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};

pub const RESOLUTION: f32 = 16.0 / 9.0;

mod inventory;
use inventory::{Inventory, InventoryPlugin};

use crate::inventory::{give_inventory_item, take_inventory_item};

#[derive(Component)]
pub struct GameCamera;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    arm_length: f32,
}

#[derive(Component, Inspectable)]
pub struct Pickupable {
    item: ItemType,
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

pub struct CraftingBook {
    recipes: Vec<CraftingRecipe>,
}

#[derive(Clone)]
pub struct CraftingRecipe {
    needed: Vec<ItemAndCount>,
    produces: ItemType,
}

#[derive(Clone, Copy)]
pub struct ItemAndCount {
    item: ItemType,
    count: usize,
}

pub struct PlaceHolderGraphics {
    texture_atlas: Handle<TextureAtlas>,
    player_index: usize,
    box_index: usize,
    item_map: HashMap<ItemType, usize>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.3, 0.5, 0.3)))
        .insert_resource(WindowDescriptor {
            width: 1600.0,
            height: 900.0,
            title: "DST clone".to_string(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InventoryPlugin)
        .insert_resource(CraftingBook {
            recipes: vec![CraftingRecipe {
                needed: vec![
                    ItemAndCount {
                        item: ItemType::Twig,
                        count: 1,
                    },
                    ItemAndCount {
                        item: ItemType::Flint,
                        count: 1,
                    },
                ],
                produces: ItemType::Axe,
            }],
        })
        .add_system(test_crafting)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_graphics)
        .add_startup_system(spawn_player)
        .add_startup_system_to_stage(StartupStage::PreStartup, spawn_camera)
        .add_startup_system(spawn_items)
        .add_system(player_movement)
        .add_system(player_pickup)
        .add_system(camera_follow)
        .register_inspectable::<Player>()
        .register_inspectable::<Pickupable>()
        .run();
}

fn can_craft(inventory: &Inventory, recipe: &CraftingRecipe) -> bool {
    for item_and_count in recipe.needed.iter() {
        let mut found_item = false;
        for item_slot in inventory.items.iter() {
            if item_slot.item == item_and_count.item && item_slot.count >= item_and_count.count {
                found_item = true;
            }
        }
        if !found_item {
            return false;
        }
    }
    true
}

fn test_crafting(
    mut inventory_query: Query<&mut Inventory>,
    crafting_book: Res<CraftingBook>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut inventory = inventory_query.single_mut();
    if keyboard.just_pressed(KeyCode::E) && can_craft(&inventory, &crafting_book.recipes[0]) {
        println!("Can craft axe!");
        for item_and_count in crafting_book.recipes[0].needed.iter() {
            take_inventory_item(&mut inventory, item_and_count.item, item_and_count.count);
        }

        give_inventory_item(&mut inventory, crafting_book.recipes[0].produces);
    }
}
fn player_pickup(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&Transform, &Player, &mut Inventory)>,
    pickupable_query: Query<(Entity, &Transform, &Pickupable), Without<Player>>,
) {
    let (player_transform, player, mut inventory) = player_query.single_mut();
    //Press space to pickup items
    //TODO if held walk to nearest
    if keyboard.just_pressed(KeyCode::Space) {
        //TODO get closest not just first
        for (ent, transform, pickup) in pickupable_query.iter() {
            if player.arm_length
                > Vec2::distance(
                    transform.translation.truncate(),
                    player_transform.translation.truncate(),
                )
                && give_inventory_item(&mut inventory, pickup.item)
            {
                //TODO not always despawn i guess
                commands.entity(ent).despawn_recursive();
            }
        }
    }
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

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
) {
    let player_transform = player_query.single().translation;
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.x;
    camera_transform.translation.y = player_transform.y;
}

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Player)>,
) {
    let (mut player_transform, player) = player_query.single_mut();

    if keyboard.pressed(KeyCode::A) {
        player_transform.translation.x -= player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        player_transform.translation.x += player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::W) {
        player_transform.translation.y += player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        player_transform.translation.y -= player.speed * time.delta_seconds();
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.left = -1.0 * RESOLUTION;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera).insert(GameCamera);
}

fn spawn_player(mut commands: Commands, graphics: Res<PlaceHolderGraphics>) {
    let mut sprite = TextureAtlasSprite::new(graphics.player_index);
    sprite.custom_size = Some(Vec2::splat(0.3));
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: graphics.texture_atlas.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 700.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            speed: 0.3,
            arm_length: 0.1,
        })
        .insert(Inventory::default())
        .insert(Name::new("Player"));
}

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
