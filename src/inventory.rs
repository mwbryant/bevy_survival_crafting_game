use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::{
    item::{ItemAndCount, WorldObject},
    prelude::{GameCamera, ItemType, PlaceHolderGraphics, RESOLUTION},
};

pub const INVENTORY_SIZE: usize = 5;

pub struct InventoryPlugin;

#[derive(Component, Default, Inspectable)]
pub struct Inventory {
    pub items: [ItemAndCount; INVENTORY_SIZE],
}

#[derive(Component)]
pub struct UiCountText {
    slot: usize,
}

#[derive(Component)]
pub struct InventoryBox {
    slot: usize,
}

#[derive(Component)]
pub struct InventoryBoxContents;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_inventory_ui)
            .add_startup_system(spawn_inventory_ui)
            .add_system(inventory_consistency_forcer)
            .register_inspectable::<Inventory>();
    }
}

//XXX does this silently hide bugs or does it fix bugs....
fn inventory_consistency_forcer(mut inventory_query: Query<&mut Inventory>) {
    let mut inventory = inventory_query.single_mut();
    for mut slot in inventory.items.iter_mut() {
        if slot.count == 0 {
            slot.item = ItemType::None;
        }
        //if slot.item == ItemType::None {
        //slot.count = 0;
        //}
    }
}

//TODO result in future if doesn't have item
pub fn take_inventory_item(inventory: &mut Inventory, to_take: ItemType, amount: usize) -> bool {
    for mut slot in inventory.items.iter_mut() {
        if slot.item == to_take {
            if slot.count < amount {
                return false;
            }
            slot.count -= amount;
        }
    }
    true
}

//TODO result in future if inventory full
pub fn give_inventory_item(inventory: &mut Inventory, to_give: ItemType) -> bool {
    //Add it if you have it
    for mut slot in inventory.items.iter_mut() {
        if slot.item == to_give {
            slot.count += 1;
            return true;
        }
    }
    //Pick it up if you dont
    for mut slot in inventory.items.iter_mut() {
        if slot.item == ItemType::None {
            slot.item = to_give;
            slot.count = 1;
            return true;
        }
    }
    false
}

pub fn can_pickup(inventory: &Inventory, to_give: ItemType) -> bool {
    for slot in inventory.items.iter() {
        if slot.item == to_give {
            return true;
        }
    }
    for slot in inventory.items.iter() {
        if slot.item == ItemType::None {
            return true;
        }
    }
    false
}

//XXX probably buggy
fn update_inventory_ui(
    mut commands: Commands,
    inventory_query: Query<&Inventory>,
    graphics: Res<PlaceHolderGraphics>,
    box_query: Query<(Entity, Option<&Children>, &InventoryBox)>,
    mut box_contents_query: Query<&mut TextureAtlasSprite, With<InventoryBoxContents>>,
    mut text_query: Query<(&UiCountText, &mut Text)>,
) {
    let inventory = inventory_query.single();
    for (i, slot) in inventory.items.iter().enumerate() {
        for (text_count, mut text) in text_query.iter_mut() {
            if text_count.slot == i {
                if slot.count == 0 {
                    text.sections[0].value = "".to_string();
                } else {
                    text.sections[0].value = format!("{}", slot.count);
                }
            }
        }
        for (box_ent, children, ui_box) in box_query.iter() {
            if ui_box.slot == i {
                if slot.count != 0 {
                    match children {
                        //Change the graphic
                        Some(children) => {
                            for child in children.iter() {
                                let mut sprite = box_contents_query
                                    .get_mut(*child)
                                    .expect("Nonsprite child of box");
                                sprite.index = *graphics
                                    .item_map
                                    .get(&WorldObject::Item(slot.item))
                                    .expect("No graphic for item");
                            }
                        }
                        //Create graphic
                        None => {
                            let mut sprite = TextureAtlasSprite::new(
                                *graphics
                                    .item_map
                                    .get(&WorldObject::Item(slot.item))
                                    .expect("No graphic for item"),
                            );
                            sprite.custom_size = Some(Vec2::splat(0.10));
                            let graphic = commands
                                .spawn_bundle(SpriteSheetBundle {
                                    sprite: sprite,
                                    texture_atlas: graphics.texture_atlas.clone(),
                                    ..Default::default()
                                })
                                .insert(Name::new("ItemGraphic"))
                                .insert(InventoryBoxContents)
                                .id();
                            commands.entity(box_ent).add_child(graphic);
                        }
                    }
                } else if let Some(children) = children {
                    //Slot empty despawn children
                    for child in children.iter() {
                        if box_contents_query.get(*child).is_ok() {
                            commands.entity(*child).despawn_recursive();
                        }
                    }
                    commands.entity(box_ent).remove::<Children>();
                }
            }
        }
    }
}

fn spawn_inventory_ui(
    mut commands: Commands,
    graphics: Res<PlaceHolderGraphics>,
    camera_query: Query<Entity, With<GameCamera>>,
    assets: Res<AssetServer>,
) {
    let camera_ent = camera_query.single();

    let mut boxes = Vec::new();
    let mut ui_texts = Vec::new();

    let spacing = 0.20;
    let spacing_percent = spacing / 2.0 / RESOLUTION * 100.0;

    let starting_x = (-(INVENTORY_SIZE as f32) / 2.0 + 0.5) * spacing;
    let starting_percent = (0.5 + starting_x / 2.0 / RESOLUTION) * 100.0 + 0.9;

    let mut sprite = TextureAtlasSprite::new(graphics.box_index);
    sprite.custom_size = Some(Vec2::splat(0.15));

    for i in 0..INVENTORY_SIZE {
        ui_texts.push(
            commands
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Auto,
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: Val::Percent(7.0),
                            left: Val::Percent(starting_percent + spacing_percent * i as f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        format!("{}", 0),
                        TextStyle {
                            //TODO don't load the font everytime please
                            //Or is the assetserver smart enough to not reload?
                            font: assets.load("QuattrocentoSans-Regular.ttf"),
                            font_size: 22.0,
                            color: Color::BLACK,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Right,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(UiCountText { slot: i })
                .insert(Name::new("Inventory Count"))
                .id(),
        );
        boxes.push(
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: sprite.clone(),
                    texture_atlas: graphics.texture_atlas.clone(),
                    transform: Transform {
                        translation: Vec3::new(starting_x + spacing * i as f32, -0.8, -1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(InventoryBox { slot: i })
                .insert(Name::new("InventoryBox"))
                .id(),
        );
    }
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .push_children(&ui_texts)
        .insert(Name::new("InventoryText"));
    commands.entity(camera_ent).push_children(&boxes);
}
