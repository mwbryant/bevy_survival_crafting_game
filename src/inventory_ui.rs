use bevy::prelude::*;
use kayak_ui::{
    bevy::{BevyContext, FontMapping, UICameraBundle},
    core::{
        render, rsx,
        styles::{Style, StyleProp, Units},
        use_state, widget, Color, Index,
    },
    widgets::{App, TextBox, Window},
};

pub struct InventoryUIPlugin;

#[widget]
fn ShowTextbox() {
    let (value, _, _) = use_state!("Hello World!".to_string());

    let text_style = Style {
        top: StyleProp::Value(Units::Pixels(10.0)),
        color: StyleProp::Value(Color::new(1., 0., 0., 1.)),
        ..Default::default()
    };

    rsx! {
        <Window position={(0., 0.)} size={(100., 100.)} title={"Example".to_string()}>
            <TextBox styles={Some(text_style)} value={value} />
        </Window>
    }
}

fn setup_inventory_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());
    info!("Kayak: UI Camera Bundle spawned");
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    let context = BevyContext::new(|context| {
        render! {
            <App>
                <ShowTextbox />
            </App>
        }
    });

    commands.insert_resource(context);
    info!("Kayak: context resource inserted");
}

impl Plugin for InventoryUIPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            setup_inventory_ui.label("kayak inventory ui"),
        );
    }
}
