use crate::{
    item::{ItemAndCount, ItemType},
    prelude::{HandUI, InventoryUI, RecipeUI},
    GameState,
};
use bevy::prelude::*;
use kayak_ui::{
    bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle},
    core::{
        bind, render, rsx,
        styles::{Edge, LayoutType, Style as KayakStyle, StyleProp, Units},
        widget, WidgetProps,
    },
    widgets::{App, Window},
};

pub struct GameUIPlugin;

#[derive(Debug)]
pub struct UIEvent(pub UIEventType);

#[derive(Debug, Clone, PartialEq)]
pub enum UIEventType {
    None,
    CraftEvent(ItemAndCount),
    ToolEvent(ItemAndCount),
    InventoryEvent(ItemAndCount),
}

impl UIEventType {
    pub fn item_and_count(self) -> ItemAndCount {
        match self {
            UIEventType::None => ItemAndCount {
                item: ItemType::None,
                count: 0,
            },
            UIEventType::CraftEvent(i)
            | UIEventType::ToolEvent(i)
            | UIEventType::InventoryEvent(i) => i,
        }
    }
}

impl Default for UIEventType {
    fn default() -> Self {
        UIEventType::None
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UIItems {
    pub inventory_items: Vec<ItemAndCount>,
    pub crafting_items: Vec<ItemAndCount>,
    pub hand_item: Option<ItemAndCount>,
}

#[derive(Default, Debug, WidgetProps, Clone, PartialEq)]
pub struct UIProps {
    #[prop_field(Styles)]
    pub styles: Option<KayakStyle>,
}

#[widget]
fn GameUI() {
    let row_style = KayakStyle {
        layout_type: StyleProp::Value(LayoutType::Row),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        col_between: StyleProp::Value(Units::Pixels(10.)),
        ..Default::default()
    };

    let column_style = KayakStyle {
        padding: StyleProp::Value(Edge::axis(Units::Pixels(10.), Units::Stretch(1.0))),
        row_between: StyleProp::Value(Units::Pixels(10.)),
        ..Default::default()
    };

    rsx! {
        <>
            <Window position={(0., 0.)} size={(100., 500.)} title={"Inventory".to_string()}>
                <InventoryUI styles={Some(column_style)} />
            </Window>
            <Window position={(1600. / 2. - 200., 900. - 100.)} size={(400., 100.)} title={"Recipes".to_string()}>
                <RecipeUI styles={Some(row_style)} />
            </Window>
            <Window position={(1600. - 200., 900. - 100.)} size={(200., 100.)} title={"Hand Slot".to_string()} >
                <HandUI styles={Some(row_style)} />
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

fn read_game_ui_events(mut event_reader: EventReader<UIEvent>) {
    for ev in event_reader.iter() {
        info!("{:?}", ev);
    }
}

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(BevyKayakUIPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Main).with_system(setup_game_ui))
            //.add_startup_system(setup_game_ui.label("kayak game ui"))
            .add_event::<UIEvent>()
            .add_system(read_game_ui_events);
    }
}
