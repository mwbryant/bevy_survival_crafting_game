use bevy::prelude::{EventWriter, Res};
use kayak_ui::{
    core::{
        constructor, rsx,
        styles::{Edge, Style, StyleProp, Units},
        use_state, widget, Binding, Bound, Color, EventType, OnEvent, VecTracker, WidgetProps,
    },
    widgets::{Button, Element, Text},
};

use crate::{
    game_ui::{UIItems, UIProps},
    prelude::{UIEvent, UIEventType},
};

#[derive(Default, Debug, WidgetProps, Clone, PartialEq)]
pub struct ItemProps {
    pub name: String,
    pub event_type: UIEventType,
    #[prop_field(Styles)]
    pub styles: Option<Style>,
    pub disabled: bool,
}

#[widget]
pub fn InventoryItem(props: ItemProps) {
    let button_style = Style {
        width: StyleProp::Value(Units::Pixels(70.0)),
        height: StyleProp::Value(Units::Pixels(50.0)),
        color: StyleProp::Value(Color::new(1., 0., 0., 1.)),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        ..props.styles.clone().unwrap_or_default()
    };

    let (clicked, set_clicked, ..) = use_state!(false);

    if clicked {
        context.query_world::<EventWriter<UIEvent>, _, _>(|mut ev| {
            ev.send(UIEvent(props.event_type.clone()));
            set_clicked(false);
        });
    }

    let on_click_event = OnEvent::new(move |_, event| {
        if let EventType::Click(..) = event.event_type {
            set_clicked(true);
        }
    });

    let item_name = props.name.clone();
    rsx! {
        <>
            <Button on_event={Some(on_click_event)} styles={Some(button_style)} disabled={props.disabled}>
                <Text content={item_name} />
            </Button>
        </>
    }
}

#[widget]
pub fn InventoryUI(ui_props: UIProps) {
    let ui_items =
        context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());

    context.bind(&ui_items);

    let ii = ui_items.get().inventory_items;
    rsx! {
        <Element styles={ui_props.styles.clone()}>
        {VecTracker::from(ii.iter().map(|item| {
            constructor! {
                <InventoryItem name={item.name.clone().to_string()}/>
            }
        }))}
        </Element>
    }
}

#[widget]
pub fn HandUI(ui_props: UIProps) {
    let ui_items =
        context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());

    context.bind(&ui_items);

    let hand_item = ui_items.get().hand_item.unwrap_or(ItemProps {
        name: "Empty".to_string(),
        event_type: UIEventType::ToolEvent("Empty".to_string()),
        styles: None,
        disabled: false,
    });

    rsx! {
        <Element styles={ui_props.styles.clone()} >
            <InventoryItem name={hand_item.name.clone()} event_type={hand_item.event_type.clone()}/>
        </Element>
    }
}

#[widget]
pub fn RecipeUI(ui_props: UIProps) {
    let ui_items =
        context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
    context.bind(&ui_items);
    let ii = ui_items.get().slot_items;

    rsx! {
        <Element styles={ui_props.styles.clone()}>
        {VecTracker::from(ii.iter().map(|item| {
            constructor! {
                <InventoryItem name={item.name.clone().to_string()} event_type={item.event_type.clone()}/>
            }
        }))}
        </ Element>
    }
}
