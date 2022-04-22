use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::{
    item::{ItemAndCount, WorldObject},
    prelude::{
        GameCamera, GameError, GameErrorType, ItemType, PlaceHolderGraphics, PIXEL_SIZE, RESOLUTION,
    },
};

pub const INVENTORY_SIZE: usize = 5;
pub const INVENTORY_ITEM_SIZE: usize = 5;

pub struct InventoryPlugin;

#[derive(Component, Default, Inspectable, Clone)]
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

pub struct InventoryOverflow(pub usize);

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_inventory_ui)
            .add_startup_system(spawn_inventory_ui)
            .register_inspectable::<Inventory>();
    }
}

impl Inventory {
    pub fn add(&mut self, item_and_count: &ItemAndCount) -> Option<InventoryOverflow> {
        let mut remaining_amount = item_and_count.count;

        for item in self
            .items
            .iter_mut()
            .filter(|item| item.item != ItemType::None)
        {
            if item.item == item_and_count.item {
                let addable_item_count =
                    std::cmp::min(remaining_amount, INVENTORY_ITEM_SIZE - item_and_count.count);
                item.count += addable_item_count;
                remaining_amount -= addable_item_count;
                if remaining_amount == 0 {
                    return None;
                }
            }
        }

        for item in self
            .items
            .iter_mut()
            .filter(|item| item.item == ItemType::None)
        {
            item.item = item_and_count.item;
            let addable_item_count =
                std::cmp::min(remaining_amount, INVENTORY_ITEM_SIZE - item_and_count.count);
            item.count = addable_item_count;
            remaining_amount -= item.count;
            if remaining_amount == 0 {
                return None;
            }
        }
        Some(InventoryOverflow(remaining_amount))
    }

    pub fn can_add(&mut self, item_and_count: ItemAndCount) -> bool {
        let mut inventory_clone = self.clone();
        match inventory_clone.add(&item_and_count) {
            Some(_) => false,
            None => true,
        }
    }

    pub fn remove(&mut self, item_and_count: &ItemAndCount) -> Result<(), GameError> {
        let mut existing = false;
        for inventory_item in self.items.iter_mut() {
            if inventory_item.item == item_and_count.item {
                existing = true;
                if inventory_item.count > item_and_count.count {
                    inventory_item.count -= item_and_count.count;
                    return Ok(());
                }
                if inventory_item.count == item_and_count.count {
                    inventory_item.count = 0;
                    inventory_item.item = ItemType::None;
                    return Ok(());
                }
            }
        }
        if existing {
            return Err(GameError::new(
                GameErrorType::ItemMissing,
                format!("Not enough items in inventory: {:?}", item_and_count.item),
            ));
        }
        Err(GameError::new(
            GameErrorType::ItemMissing,
            format!("Item not in inventory: {:?}", item_and_count.item),
        ))
    }

    pub fn can_remove(&mut self, item_and_count: &ItemAndCount) -> bool {
        let mut inventory_clone = self.clone();
        match inventory_clone.remove(item_and_count) {
            Ok(()) => true,
            _ => false,
        }
    }
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
        text_query
            .iter_mut()
            .filter(|(text, _)| text.slot == i)
            .for_each(|(_, mut text)| {
                if slot.count == 0 {
                    text.sections[0].value = "".to_string();
                } else {
                    text.sections[0].value = format!("{}", slot.count);
                }
            });
        for (box_ent, children, ui_box) in box_query.iter() {
            if ui_box.slot != i {
                continue;
            }
            if slot.count == 0 {
                // NOTE: with feature(let_chains) activated we can merge if's with `if let Some`,
                if let Some(children) = children {
                    //Slot empty despawn children
                    for child in children.iter() {
                        if box_contents_query.get(*child).is_ok() {
                            commands.entity(*child).despawn_recursive();
                        }
                    }
                    commands.entity(box_ent).remove::<Children>();
                }
                continue;
            }
            if let Some(children) = children {
                for child in children.iter() {
                    let mut sprite = box_contents_query
                        .get_mut(*child)
                        .expect("Nonsprite child of box");
                    sprite.index = graphics
                        .item_map
                        .get(&WorldObject::Item(slot.item))
                        .expect("No graphic for item")
                        .index;
                }
                continue;
            }

            let sprite = graphics
                .item_map
                .get(&WorldObject::Item(slot.item))
                .expect("No graphic for item")
                .clone();

            //sprite.custom_size = Some(Vec2::splat(PIXEL_SIZE * 16.));
            let graphic = commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite,
                    texture_atlas: graphics.texture_atlas.clone(),
                    ..Default::default()
                })
                .insert(Name::new("ItemGraphic"))
                .insert(InventoryBoxContents)
                .id();
            commands.entity(box_ent).add_child(graphic);
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
    sprite.custom_size = Some(Vec2::splat(PIXEL_SIZE * 32.));

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
                            color: Color::WHITE,
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
