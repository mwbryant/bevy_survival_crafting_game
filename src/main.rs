#![allow(clippy::redundant_field_names)]
#![allow(clippy::expect_fun_call)]
#![allow(clippy::type_complexity)]
use bevy::{prelude::*, window::PresentMode};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

mod assets;
mod crafting;
mod error;
mod fire_graphics;
mod game_camera;
mod inventory;
mod item;
mod mouse;
mod player;
mod prelude;

use mouse::MousePlugin;
// todo implement `PluginGroup`
use prelude::{
    CraftingPlugin, FireGraphicsPlugin, GameAssetsPlugin, GameCameraPlugin, InventoryPlugin,
    ItemsPlugin, PlayerPlugin,
};

pub const HEIGHT: f32 = 900.;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("b0c060").unwrap()))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "DST clone".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldInspectorParams {
            despawnable_entities: true,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GameAssetsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MousePlugin)
        .add_plugin(ItemsPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(CraftingPlugin)
        .add_plugin(GameCameraPlugin)
        .add_plugin(FireGraphicsPlugin)
        .run();
}
