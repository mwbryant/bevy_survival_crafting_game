//Gross as BevyImage
use bevy::prelude::{default, EventWriter, Handle, Image as BevyImage, Res, ResMut};
use kayak_ui::{
    bevy::ImageManager,
    core::{
        constructor, rsx,
        styles::{Edge, Style, StyleProp, Units},
        widget, Binding, Bound, Color, EventType, OnEvent, VecTracker, WidgetProps,
    },
    widgets::{Button, Element, Image, Text},
};

use crate::{
    game_ui::{UIItems, UIProps},
    item::WorldObject,
    prelude::{Graphics, UIEvent, UIEventType},
};

#[derive(Default, Debug, WidgetProps, Clone, PartialEq)]
pub struct ItemProps {
    pub event_type: UIEventType,
    //Option to sastify Default
    pub handle: Option<Handle<BevyImage>>,
    #[prop_field(Styles)]
    pub styles: Option<Style>,
    pub disabled: bool,
}

#[widget]
pub fn Item(props: ItemProps) {
    let button_style = Style {
        width: StyleProp::Value(Units::Pixels(50.0)),
        height: StyleProp::Value(Units::Pixels(50.0)),
        background_color: StyleProp::Value(Color::TRANSPARENT),
        //background_color: StyleProp::Value(Color::new(0.4, 0.4, 0.4, 1.0)),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        ..props.styles.clone().unwrap_or_default()
    };

    let image_style = Style {
        width: StyleProp::Value(Units::Pixels(45.0)),
        height: StyleProp::Value(Units::Pixels(45.0)),
        ..default()
    };

    let ui_event = props.event_type;

    let on_click_event = OnEvent::new(move |context, event| {
        if let EventType::Click(..) = event.event_type {
            context.query_world::<EventWriter<UIEvent>, _, _>(move |mut ev| {
                ev.send(UIEvent(ui_event));
            });
        }
    });

    let handle = context.query_world::<ResMut<ImageManager>, _, _>(|mut manager| {
        manager.get(&props.handle.clone().unwrap())
    });

    let item_count = format!("x{}", props.clone().event_type.item_and_count().count);

    let text_style = Style {
        right: StyleProp::Value(Units::Pixels(5.0)),
        ..default()
    };

    rsx! {
        <>
            <Button on_event={Some(on_click_event)} styles={Some(button_style)} disabled={props.disabled}>
                <Image handle={handle} styles={Some(image_style)} />
                <Text content={item_count} styles={Some(text_style)} />
            </Button>
        </>
    }
}

#[widget]
pub fn InventoryUI(ui_props: UIProps) {
    let ui_items =
        context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());

    let handles = context.query_world::<Res<Graphics>, _, _>(|graphics| graphics.image_map.clone());

    context.bind(&ui_items);

    let ii = ui_items.get().inventory_items;
    rsx! {
        <Element styles={ui_props.styles.clone()}>
        {VecTracker::from(ii.iter().map(|item| {
            constructor! {
                <Item event_type=
                {UIEventType::InventoryEvent(*item)}
                handle={Some(handles.get(&WorldObject::Item(item.item)).unwrap().clone())}/>
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

    let hand_item = ui_items.get().hand_item;
    let handles = context.query_world::<Res<Graphics>, _, _>(|graphics| graphics.image_map.clone());

    if let Some(item) = hand_item {
        rsx! {
            <Element styles={ui_props.styles.clone()} >
                <Item event_type={UIEventType::ToolEvent(hand_item.unwrap())}
                handle={Some(handles.get(&WorldObject::Item(item.item)).unwrap().clone())}/>
            </Element>
        }
    } else {
        rsx! {
            <Element styles={ui_props.styles.clone()} >
            </Element>
        }
    }
}

#[widget]
pub fn RecipeUI(ui_props: UIProps) {
    let ui_items =
        context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
    context.bind(&ui_items);

    let handles = context.query_world::<Res<Graphics>, _, _>(|graphics| graphics.image_map.clone());
    let crafting_items = ui_items.get().crafting_items;

    rsx! {
        <Element styles={ui_props.styles.clone()}>
        {VecTracker::from(crafting_items.iter().map(|item| {
            constructor! {
                <Item event_type={UIEventType::CraftEvent(*item)}
                handle={println!("{:?}", item); Some(handles.get(item).unwrap().clone())}/>
            }
        }))}
        </ Element>
    }
}
