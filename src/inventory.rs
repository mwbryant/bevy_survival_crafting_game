use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::{game_ui::UIItems, item::ItemAndCount, prelude::*};

pub const INVENTORY_SIZE: usize = 3;
pub const INVENTORY_ITEM_SIZE: usize = 5;

pub struct InventoryPlugin;

#[derive(Component, Default, Inspectable, Clone)]
pub struct Inventory {
    pub items: [ItemAndCount; INVENTORY_SIZE],
}

#[derive(Component)]
pub struct InventoryBox {
    pub slot: usize,
}

#[derive(Component)]
pub struct InventoryBoxContents;

pub struct InventoryOverflow(pub usize);

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<Inventory>();
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

    pub fn can_add(&self, item_and_count: &ItemAndCount) -> bool {
        let mut inventory_clone = self.clone();
        inventory_clone.add(&item_and_count).is_none()
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

    pub fn can_remove(&self, item_and_count: &ItemAndCount) -> bool {
        let mut inventory_clone = self.clone();
        matches!(inventory_clone.remove(item_and_count), Ok(()))
    }

    pub fn get_ui_representation(&self) -> UIItems {
        let item_props = self
            .items
            .iter()
            .filter(|item| item.item != ItemType::None)
            .map(|item| ItemProps {
                name: format!("{:?} x{}", item.item, item.count),
                event_type: UIEventType::None,
                styles: None,
                disabled: false,
            })
            .collect();
        UIItems {
            inventory_items: item_props,
            ..Default::default()
        }
    }
}
