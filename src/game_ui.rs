use crate::prelude::{HandUI, InventoryUI, ItemProps};
use bevy::prelude::*;
use kayak_ui::{
    bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle},
    core::{bind, render, rsx, widget, Index},
    widgets::{App, Window},
};

pub struct GameUIPlugin;

pub struct UIEvent(UIEventType);

pub enum UIEventType {
    CraftEvent(String),
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UIItems {
    pub inventory_items: Vec<ItemProps>,
    pub slot_items: Vec<ItemProps>,
    pub hand_item: Option<ItemProps>,
}

#[widget]
fn GameUI() {
    rsx! {
        <>
            <Window position={(0., 0.)} size={(100., 500.)} title={"Inventory Window".to_string()}>
                <InventoryUI />
            </Window>
            <Window position={(1600. / 2. - 200., 900. - 100.)} size={(400., 100.)} title={"Item Slots Window".to_string()}>
            </Window>
            <Window position={(1600. - 200., 900. - 100.)} size={(200., 100.)} title={"Hand Slot Window".to_string()}>
                <HandUI />
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

    commands.insert_resource(bind(UIItems::default()));

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
            .add_startup_system(setup_game_ui.label("kayak game ui"))
            .add_event::<UIEvent>();
    }
}
