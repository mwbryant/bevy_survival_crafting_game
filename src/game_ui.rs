use bevy::prelude::*;
use kayak_ui::{
    bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle},
    core::{render, rsx, widget, Index},
    widgets::{App, Window},
};

pub struct GameUIPlugin;

#[widget]
fn GameUI() {
    rsx! {
        <>
            <Window position={(0., 0.)} size={(100., 500.)} title={"Inventory Window".to_string()}>
            </Window>
            <Window position={(1600. / 2. - 200., 900. - 100.)} size={(400., 100.)} title={"Item Slots Window".to_string()}>
            </Window>
            <Window position={(1600. - 200., 900. - 100.)} size={(200., 100.)} title={"Hand Slot Window".to_string()}>
            </Window>
        </>
    }
}

fn setup_game_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    let context = BevyContext::new(|context| {
        render! {
            <App>
                <GameUI />
            </App>
        }
    });

    commands.insert_resource(context);
    info!("game ui initialized");
}

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(BevyKayakUIPlugin)
            .add_startup_system_to_stage(
                StartupStage::PreStartup,
                setup_game_ui.label("kayak game ui"),
            );
    }
}
