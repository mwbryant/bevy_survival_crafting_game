#![allow(clippy::redundant_field_names)]
#![allow(clippy::expect_fun_call)]
#![allow(clippy::type_complexity)]
use bevy::{prelude::*, window::PresentMode};
use bevy_asset_loader::*;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

mod assets;
mod crafting;
mod error;
mod fire_graphics;
mod game_camera;
mod game_ui;
mod game_ui_widgets;
mod inventory;
mod item;
mod mouse;
mod player;
mod prelude;

use mouse::MousePlugin;
// todo implement `PluginGroup`
use prelude::{
    CraftingPlugin, FireGraphicsPlugin, GameAssetsPlugin, GameCameraPlugin, GameUIPlugin,
    InventoryPlugin, ItemsPlugin, PlayerPlugin,
};

pub const HEIGHT: f32 = 900.;
pub const RESOLUTION: f32 = 16.0 / 9.0;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    Main,
}

//TODO move to assets.rs
#[derive(AssetCollection)]
struct ImageAssets {
    #[asset(path = "bevy_survival_sprites.png")]
    pub sprite_sheet: Handle<Image>,
}

fn main() {
    let mut app = App::new();
    AssetLoader::new(GameState::Splash)
        .continue_to_state(GameState::Main)
        .with_collection::<ImageAssets>()
        .build(&mut app);
    app.add_state(GameState::Splash)
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
        .add_plugin(GameCameraPlugin)
        .add_plugin(GameAssetsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MousePlugin)
        .add_plugin(ItemsPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(CraftingPlugin)
        .add_plugin(FireGraphicsPlugin)
        .add_plugin(GameUIPlugin)
        .run();
}
