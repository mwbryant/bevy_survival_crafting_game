#![allow(clippy::redundant_field_names)]
#![allow(clippy::expect_fun_call)]
#![allow(clippy::type_complexity)]
use bevy::{prelude::*, window::PresentMode};
use bevy_inspector_egui::WorldInspectorPlugin;

mod assets;
mod crafting;
mod error;
mod game_camera;
mod inventory;
mod item;
mod mouse;
mod player;
mod prelude;

use mouse::MousePlugin;
// todo implement `PluginGroup`
use prelude::{
    CraftingPlugin, GameAssetsPlugin, GameCameraPlugin, InventoryPlugin, ItemsPlugin, PlayerPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.3, 0.5, 0.3)))
        .insert_resource(WindowDescriptor {
            width: 1600.0,
            height: 900.0,
            title: "DST clone".to_string(),
            present_mode: PresentMode::Immediate,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GameAssetsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MousePlugin)
        .add_plugin(ItemsPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(CraftingPlugin)
        .add_plugin(GameCameraPlugin)
        .run();
}
