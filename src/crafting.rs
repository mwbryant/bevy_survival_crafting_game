use bevy::prelude::*;

use crate::prelude::{
    ItemAndCount,
    ItemType,
    Inventory,
    give_inventory_item,
    take_inventory_item
};

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CraftingBook {
                recipes: vec![CraftingRecipe {
                    needed: vec![
                        ItemAndCount {
                            item: ItemType::Twig,
                            count: 1,
                        },
                        ItemAndCount {
                            item: ItemType::Flint,
                            count: 1,
                        },
                    ],
                    produces: ItemType::Axe,
                }],
            })
            .add_system(Self::test_crafting);
    }
}

impl CraftingPlugin {
    fn test_crafting(
        mut inventory_query: Query<&mut Inventory>,
        crafting_book: Res<CraftingBook>,
        keyboard: Res<Input<KeyCode>>,
    ) {
        let mut inventory = inventory_query.single_mut();
        if keyboard.just_pressed(KeyCode::E) && can_craft(&inventory, &crafting_book.recipes[0]) {
            println!("Can craft axe!");
            for item_and_count in crafting_book.recipes[0].needed.iter() {
                take_inventory_item(&mut inventory, item_and_count.item, item_and_count.count);
            }

            give_inventory_item(&mut inventory, crafting_book.recipes[0].produces);
        }
    }
}

pub struct CraftingBook {
    pub(crate) recipes: Vec<CraftingRecipe>,
}

#[derive(Clone)]
pub struct CraftingRecipe {
    pub(crate) needed: Vec<ItemAndCount>,
    pub(crate) produces: ItemType,
}

fn can_craft(inventory: &Inventory, recipe: &CraftingRecipe) -> bool {
    for item_and_count in recipe.needed.iter() {
        let mut found_item = false;
        for item_slot in inventory.items.iter() {
            if item_slot.item == item_and_count.item && item_slot.count >= item_and_count.count {
                found_item = true;
            }
        }
        if !found_item {
            return false;
        }
    }
    true
}
