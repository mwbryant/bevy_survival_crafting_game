#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
use bevy::{prelude::*, render::camera::ScalingMode, utils::HashMap};
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const INVENTORY_SIZE: usize = 20;

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

#[derive(Component, Default, Inspectable)]
pub struct Inventory {
    items: [InventoryEntry; INVENTORY_SIZE],
}

#[derive(Default, Inspectable)]
pub struct InventoryEntry {
    item: ItemType,
    count: usize,
}

#[derive(Component)]
pub struct UiCountText {
    slot: usize,
}

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ItemType {
    None,
    Flint,
    Axe,
    Twig,
    Grass,
    Wood,
    PineCone,
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::None
    }
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
        .add_startup_system_to_stage(StartupStage::PreStartup, load_graphics)
        .add_startup_system(spawn_player)
        .add_startup_system_to_stage(StartupStage::PreStartup, spawn_camera)
        .add_startup_system(spawn_inventory_ui)
        .add_startup_system(spawn_flint)
        .add_system(player_movement)
        .add_system(player_pickup)
        .add_system(camera_follow)
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<Inventory>()
        .register_inspectable::<Player>()
        .register_inspectable::<Pickupable>()
        .run();
}

fn spawn_inventory_ui(
    mut commands: Commands,
    graphics: Res<PlaceHolderGraphics>,
    camera_query: Query<Entity, With<GameCamera>>,
    assets: Res<AssetServer>,
) {
    let camera_ent = camera_query.single();

    let mut boxes = Vec::new();
    let mut ui_texts = Vec::new();

    let spacing = 0.15;
    let spacing_percent = spacing / 2.0 / RESOLUTION * 100.0;

    let starting_x = (-(INVENTORY_SIZE as f32) / 2.0 + 0.5) * spacing;
    let starting_percent = (0.5 + starting_x / 2.0 / RESOLUTION) * 100.0;

    let mut sprite = TextureAtlasSprite::new(graphics.box_index);
    sprite.custom_size = Some(Vec2::splat(0.1));

    for i in 0..INVENTORY_SIZE {
        ui_texts.push(
            commands
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Auto,
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: Val::Percent(8.0),
                            left: Val::Percent(starting_percent + spacing_percent * i as f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        format!("{}", 0),
                        TextStyle {
                            font: assets.load("QuattrocentoSans-Regular.ttf"),
                            font_size: 19.0,
                            color: Color::BLACK,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Right,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(UiCountText { slot: i })
                .insert(Name::new("Inventory Count"))
                .id(),
        );
        boxes.push(
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: sprite.clone(),
                    texture_atlas: graphics.texture_atlas.clone(),
                    transform: Transform {
                        translation: Vec3::new(starting_x + spacing * i as f32, -0.8, -1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .id(),
        );
    }
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .push_children(&ui_texts)
        .insert(Name::new("InventoryText"))
        ;
    commands.entity(camera_ent).push_children(&boxes);
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
                > Vec3::distance(transform.translation, player_transform.translation)
            {
                //Add it if you have it
                for mut slot in inventory.items.iter_mut() {
                    if slot.item == pickup.item {
                        slot.count += 1;
                        commands.entity(ent).despawn_recursive();
                        return;
                    }
                }
                //Pick it up if you dont
                for mut slot in inventory.items.iter_mut() {
                    if slot.item == ItemType::None {
                        slot.item = pickup.item;
                        slot.count = 1;
                        commands.entity(ent).despawn_recursive();
                        return;
                    }
                }
            }
        }
    }
}

fn spawn_flint(mut commands: Commands, graphics: Res<PlaceHolderGraphics>) {
    let mut sprite = TextureAtlasSprite::new(
        *graphics
            .item_map
            .get(&ItemType::Flint)
            .expect("No graphic for flint"),
    );
    sprite.custom_size = Some(Vec2::splat(0.1));
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: graphics.texture_atlas.clone(),
            ..Default::default()
        })
        .insert(Pickupable {
            item: ItemType::Flint,
        })
        .insert(Name::new("Flint"));
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
        min: Vec2::new(32.0, 0.0),
        max: Vec2::new(48.0, 16.0),
    });

    let mut item_map = HashMap::default();
    item_map.insert(ItemType::Flint, flint_index);

    let box_index = atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(0.0, 32.0),
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
