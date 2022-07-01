use crate::{
    item::{ItemAndCount, ItemType, WorldObject},
    prelude::{HandUI, InventoryUI, RecipeUI},
    GameState, HEIGHT, RESOLUTION,
};
use bevy::prelude::*;
use kayak_ui::{
    bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle},
    core::{
        bind, render, rsx,
        styles::{Edge, LayoutType, PositionType, Style as KayakStyle, StyleProp, Units},
        widget, Color, WidgetProps,
    },
    widgets::{App, Background, Clip},
};

pub struct GameUIPlugin;

#[derive(Debug)]
pub struct UIEvent(pub UIEventType);

#[derive(Debug, Clone, PartialEq)]
pub enum UIEventType {
    None,
    CraftEvent(WorldObject),
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
            UIEventType::CraftEvent(_i) => ItemAndCount {
                item: ItemType::None,
                count: 1,
            },
            UIEventType::ToolEvent(i) | UIEventType::InventoryEvent(i) => i,
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
    pub crafting_items: Vec<WorldObject>,
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
        //padding: StyleProp::Value(Edge::axis(Units::Pixels(10.), Units::Stretch(1.0))),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        row_between: StyleProp::Value(Units::Pixels(10.)),
        ..Default::default()
    };

    let clip_styles = KayakStyle {
        padding: StyleProp::Value(Edge::all(Units::Pixels(5.0))),
        width: StyleProp::Value(Units::Stretch(1.0)),
        height: StyleProp::Value(Units::Stretch(1.0)),
        ..Default::default()
    };

    let width = 100.;
    let inventory_pos = (HEIGHT * RESOLUTION / 2., HEIGHT - width);
    let recipe_pos = (0., HEIGHT / 2.0);
    let hand_pos = (HEIGHT * RESOLUTION - width, HEIGHT - width);

    let recipe_style = KayakStyle {
        position_type: StyleProp::Value(PositionType::SelfDirected),
        left: StyleProp::Value(Units::Pixels(recipe_pos.0)),
        top: StyleProp::Value(Units::Pixels(recipe_pos.1)),
        //XXX these sizes should not matter ...
        width: StyleProp::Value(Units::Pixels(width)),
        height: StyleProp::Value(Units::Pixels(width)),
        max_width: StyleProp::Value(Units::Pixels(width)),
        max_height: StyleProp::Value(Units::Pixels(width)),
        background_color: StyleProp::Value(Color::new(0.7, 0.4, 0.4, 0.0)),
        ..Default::default()
    };

    let inventory_style = KayakStyle {
        position_type: StyleProp::Value(PositionType::SelfDirected),
        left: StyleProp::Value(Units::Pixels(inventory_pos.0)),
        top: StyleProp::Value(Units::Pixels(inventory_pos.1)),
        width: StyleProp::Value(Units::Pixels(width)),
        height: StyleProp::Value(Units::Pixels(width)),
        max_width: StyleProp::Value(Units::Pixels(width)),
        max_height: StyleProp::Value(Units::Pixels(width)),
        background_color: StyleProp::Value(Color::new(0.4, 0.7, 0.4, 0.0)),
        ..Default::default()
    };

    let hand_style = KayakStyle {
        position_type: StyleProp::Value(PositionType::SelfDirected),
        left: StyleProp::Value(Units::Pixels(hand_pos.0)),
        top: StyleProp::Value(Units::Pixels(hand_pos.1)),
        width: StyleProp::Value(Units::Pixels(width)),
        height: StyleProp::Value(Units::Pixels(width)),
        max_width: StyleProp::Value(Units::Pixels(width)),
        max_height: StyleProp::Value(Units::Pixels(width)),
        background_color: StyleProp::Value(Color::new(0.4, 0.4, 0.7, 0.0)),
        ..Default::default()
    };

    rsx! {
        <Clip styles= {Some(clip_styles)}>
            <Background styles={Some(inventory_style)}>
                <InventoryUI styles={Some(row_style)} />
            </Background>
            <Background styles={Some(recipe_style)}>
                <RecipeUI styles={Some(column_style)} />
            </Background>
            <Background styles={Some(hand_style)}>
                <HandUI styles={Some(row_style)} />
            </Background>
        </Clip>
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
}

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(BevyKayakUIPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Main).with_system(setup_game_ui))
            .add_event::<UIEvent>();
    }
}
