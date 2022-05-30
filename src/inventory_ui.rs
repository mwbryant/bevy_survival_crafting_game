use bevy::prelude::{info, Res};
use kayak_ui::{
    core::{
        constructor, rsx,
        styles::{Style, StyleProp, Units},
        use_state, widget, Binding, Bound, Color, EventType, OnEvent, VecTracker, WidgetProps,
    },
    widgets::{Button, Text, TextBox},
};

use crate::game_ui::UIItems;

#[derive(Default, Debug, WidgetProps, Clone, PartialEq)]
pub struct ItemProps {
    pub name: String,
}

#[widget]
pub fn InventoryItem(props: ItemProps) {
    let text_style = Style {
        top: StyleProp::Value(Units::Pixels(10.0)),
        left: StyleProp::Value(Units::Pixels(10.0)),
        width: StyleProp::Value(Units::Pixels(70.0)),
        height: StyleProp::Value(Units::Pixels(20.0)),
        color: StyleProp::Value(Color::new(1., 0., 0., 1.)),
        ..Default::default()
    };

    let on_click_event = OnEvent::new(move |_, event| match event.event_type {
        EventType::Click(..) => {
            println!("klick!");
        }
        _ => {}
    });
    let item_name = props.name.clone();
    rsx! {
        <>
            <TextBox styles={Some(text_style)} value={item_name} />
            <Button on_event={Some(on_click_event)}>
                <Text content={"Klick".to_string()} />
            </Button>
        </>
    }
}

#[widget]
pub fn InventoryUI() {
    let ui_items =
        context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());

    context.bind(&ui_items);

    let ii = ui_items.get().inventory_items;
    rsx! {
        <>
        {VecTracker::from(ii.iter().map(|item| {
            constructor! {
                <InventoryItem name={item.name.clone().to_string()}/>
            }
        }))}
        </>
    }
}

#[widget]
pub fn HandUI() {
    let ui_items =
        context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());

    context.bind(&ui_items);

    let hand_item = ui_items.get().hand_item.unwrap_or(ItemProps {
        name: "Empty".to_string(),
    });

    rsx! {
        <InventoryItem name={hand_item.name}/>
    }
}
