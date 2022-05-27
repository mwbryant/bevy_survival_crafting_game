use kayak_ui::{
    core::{
        constructor, rsx,
        styles::{Style, StyleProp, Units},
        use_state, widget, Color, VecTracker, WidgetProps,
    },
    widgets::TextBox,
};

#[derive(Default, Debug, WidgetProps, Clone, PartialEq)]
pub struct ItemProps {
    pub name: String,
}

#[widget]
pub fn InventoryItem(props: ItemProps) {
    let (item_name, _, _) = use_state!(props.name.clone());

    let text_style = Style {
        top: StyleProp::Value(Units::Pixels(10.0)),
        left: StyleProp::Value(Units::Pixels(10.0)),
        width: StyleProp::Value(Units::Pixels(70.0)),
        height: StyleProp::Value(Units::Pixels(70.0)),
        color: StyleProp::Value(Color::new(1., 0., 0., 1.)),
        ..Default::default()
    };

    rsx! {
        <TextBox styles={Some(text_style)} value={item_name} />
    }
}

#[widget]
pub fn InventoryUI() {
    let items = vec!["Item 1", "Item 2", "Item 3"];
    rsx! {
        <>
        {VecTracker::from(items.iter().map(|item| {
            constructor! {
                <InventoryItem name={item.clone().to_string()}/>
            }
        }))}
        </>
    }
}
