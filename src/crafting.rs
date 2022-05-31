use std::fs;

use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use serde::Deserialize;

use crate::prelude::*;

#[derive(Component, Inspectable)]
pub struct CraftingBox {
    //TODO grey out currently impossible recipes
    active: bool,
    //This is always a valid index, enforce this
    recipe_index: usize,
}

#[derive(Clone, Deserialize)]
pub struct CraftingBook {
    pub(crate) recipes: Vec<CraftingRecipe>,
}

#[derive(Clone, Deserialize)]
pub struct CraftingRecipe {
    pub(crate) needed: Vec<ItemAndCount>,
    pub(crate) produces: ItemType,
}

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        let crafting_desc = fs::read_to_string("assets/recipes.ron").unwrap();

        let crafting_book: CraftingBook = ron::de::from_str(&crafting_desc).unwrap_or_else(|e| {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        });

        app.insert_resource(crafting_book)
            .add_system(Self::craft_item)
            .register_inspectable::<CraftingBox>();
    }
}

impl CraftingPlugin {
    fn craft_item(
        mut event_reader: EventReader<UIEvent>,
        mut inventory_query: Query<&mut Inventory, With<Player>>,
        crafting_book: Res<CraftingBook>,
    ) {
        for ev in event_reader.iter() {
            match ev.0.clone() {
                UIEventType::CraftEvent(item) => {
                    // get player inventory
                    let mut inventory = inventory_query.single_mut();

                    // find recipe to craft
                    let recipe_to_craft = crafting_book
                        .recipes
                        .iter()
                        .filter(|recipe| recipe.produces.name() == item)
                        .collect::<Vec<&CraftingRecipe>>()[0];

                    // make sure inventory has ingredients and space to store new item
                    if inventory.ingredients_available(recipe_to_craft)
                        && inventory.can_add(&ItemAndCount {
                            item: recipe_to_craft.produces,
                            count: 1,
                        })
                    {
                        // remove ingredients
                        recipe_to_craft.needed.iter().for_each(|ingredient| {
                            inventory
                                .remove(ingredient)
                                .expect("removing ingredients failed")
                        });
                        // add newly crafted item
                        inventory.add(&ItemAndCount {
                            item: recipe_to_craft.produces,
                            count: 1,
                        });
                    } else {
                        info!("either not enough ingredients or not enough space in inventory");
                    }
                }
                _ => {}
            }
        }
    }
}

impl Inventory {
    pub fn ingredients_available(&self, recipe: &CraftingRecipe) -> bool {
        for ingredient in recipe.needed.clone() {
            if !self.can_remove(&ingredient) {
                return false;
            }
        }
        true
    }
}
