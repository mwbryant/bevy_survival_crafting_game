use std::fs;

use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use serde::Deserialize;

use crate::{build::PlaceableGhost, prelude::*};

/// Currently Unused but needs to be reincorperated with UI to render the outline
/// of the box and grey out contents (or deleted)
#[derive(Component, Inspectable)]
pub struct CraftingBox {
    //TODO grey out currently impossible recipes
    active: bool,
    //This is always a valid index, enforce this
    recipe_index: usize,
}

/// Resource loaded from recipes.ron
/// Contains all valid recipes currently in the game
#[derive(Clone, Deserialize)]
pub struct CraftingBook {
    pub(crate) recipes: Vec<CraftingRecipe>,
}

// The description of a single recipe
#[derive(Clone, Deserialize)]
pub struct CraftingRecipe {
    pub(crate) needed: Vec<ItemAndCount>,
    pub(crate) produces: WorldObject,
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
    /// A system to respond to crafting UI Events
    /// Checks if the player can craft something, manages inventory juggling, and sets the ghost if
    /// the item is placeable
    fn craft_item(
        mut event_reader: EventReader<UIEvent>,
        mut inventory_query: Query<(&mut Inventory, &mut PlaceableGhost), With<Player>>,
        crafting_book: Res<CraftingBook>,
    ) {
        for ev in event_reader.iter() {
            if let UIEventType::CraftEvent(item) = ev.0 {
                // get player inventory
                let (mut inventory, mut ghost) = inventory_query.single_mut();

                // find recipe to craft
                let recipe_to_craft = crafting_book
                    .recipes
                    .iter()
                    .filter(|recipe| recipe.produces == item)
                    .collect::<Vec<&CraftingRecipe>>()[0];

                match recipe_to_craft.produces {
                    WorldObject::Item(item) => {
                        // make sure inventory has ingredients and space to store new item
                        if inventory.ingredients_available(recipe_to_craft)
                            && inventory.can_add(&ItemAndCount {
                                item: item,
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
                                item: item,
                                count: 1,
                            });
                        } else {
                            info!("either not enough ingredients or not enough space in inventory");
                        }
                    }
                    //Making something placeable
                    _ => {
                        if inventory.ingredients_available(recipe_to_craft) {
                            // remove ingredients
                            recipe_to_craft.needed.iter().for_each(|ingredient| {
                                inventory
                                    .remove(ingredient)
                                    .expect("removing ingredients failed")
                            });
                            ghost.to_place = Some(recipe_to_craft.produces);
                        } else {
                            info!("Not enough items!");
                        }
                    }
                }
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
