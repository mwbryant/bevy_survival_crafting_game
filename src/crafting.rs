use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::{item::WorldObject, prelude::*};

#[derive(Component, Inspectable)]
pub struct CraftingBox {
    //TODO grey out currently impossible recipes
    active: bool,
    //This is always a valid index, enforce this
    recipe_index: usize,
}

pub struct CraftingBook {
    pub(crate) recipes: Vec<CraftingRecipe>,
}

#[derive(Clone)]
pub struct CraftingRecipe {
    pub(crate) needed: Vec<ItemAndCount>,
    pub(crate) produces: ItemType,
}

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CraftingBook {
            recipes: vec![
                CraftingRecipe {
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
                    produces: ItemType::Tool(Tool::Axe),
                },
                CraftingRecipe {
                    needed: vec![
                        ItemAndCount {
                            item: ItemType::Wood,
                            count: 1,
                        },
                        ItemAndCount {
                            item: ItemType::Grass,
                            count: 2,
                        },
                    ],
                    produces: ItemType::Fire,
                },
            ],
        })
        .add_startup_system(Self::spawn_crafting_ui)
        .add_system(Self::crafting_ui_graying)
        .add_system(Self::test_crafting)
        .add_system(Self::crafting_ui_active)
        .register_inspectable::<CraftingBox>();
    }
}

impl CraftingPlugin {
    fn test_crafting(
        mut inventory_query: Query<&mut Inventory>,
        box_query: Query<(&GlobalTransform, &CraftingBox)>,
        crafting_book: Res<CraftingBook>,
        mouse_pos: Res<MousePosition>,
        mouse: Res<Input<MouseButton>>,
    ) {
        for (transform, crafting_box) in box_query.iter() {
            assert!(transform.scale == Vec3::splat(1.0));
            let translation = transform.translation;
            //TODO use bevy aabb collide method
            // 80% of the dimensions of the box, that's 0.4 in either direction
            if !(mouse_pos.0.x > translation.x - 0.4
                && mouse_pos.0.x < translation.x + 0.4
                && mouse_pos.0.y > translation.y - 0.4
                && mouse_pos.0.y < translation.y + 0.4)
            {
                continue;
            }
            //We are over a box
            let mut inventory = inventory_query.single_mut();
            if !mouse.just_pressed(MouseButton::Left) {
                continue;
            }
            //TODO just check if the button is active,
            // button should if can craft
            if !inventory.ingredients_available(&crafting_book.recipes[crafting_box.recipe_index]) {
                info!("neccessary ingredients for crafting not available");
                continue;
            }

            crafting_book.recipes[crafting_box.recipe_index]
                .needed
                .iter()
                .for_each(|item_and_count| {
                    if let Err(error) = inventory.remove(item_and_count) {
                        warn!("{:?}", error);
                    };
                });

            let product_item = crafting_book.recipes[crafting_box.recipe_index].produces;
            if let Some(overflow) = inventory.add(&ItemAndCount {
                item: product_item,
                count: 1,
            }) {
                warn!(
                    "couldnt add item to inventory: {}x{:?}",
                    overflow.0, product_item
                );
            };
        }
    }

    fn crafting_ui_active(
        mut crafting_ui: Query<&mut CraftingBox>,
        crafting_book: Res<CraftingBook>,
        inventory_query: Query<&Inventory, With<Player>>,
    ) {
        let inventory = inventory_query.single();
        for mut ui_box in crafting_ui.iter_mut() {
            ui_box.active = can_craft(inventory, &crafting_book.recipes[ui_box.recipe_index]);
        }
    }

    fn crafting_ui_graying(
        mut crafting_ui: Query<
            (&Children, &mut TextureAtlasSprite, &CraftingBox),
            Changed<CraftingBox>,
        >,
        mut box_children: Query<&mut TextureAtlasSprite, Without<CraftingBox>>,
    ) {
        for (children, mut sprite, crafting_box) in crafting_ui.iter_mut() {
            if crafting_box.active {
                sprite.color = Color::WHITE;
            } else {
                sprite.color = Color::GRAY;
            }
            for child in children.iter() {
                if let Ok(mut sprite) = box_children.get_mut(*child) {
                    if crafting_box.active {
                        sprite.color = Color::WHITE;
                    } else {
                        sprite.color = Color::GRAY;
                    }
                }
            }
        }
    }

    fn spawn_crafting_ui(
        mut commands: Commands,
        graphics: Res<Graphics>,
        camera_query: Query<Entity, With<GameCamera>>,
        book: Res<CraftingBook>,
    ) {
        let camera_ent = camera_query.single();

        let starting_y = book.recipes.len() as f32 / 2.0 - 0.5;

        let mut sprite = TextureAtlasSprite::new(graphics.box_index);
        sprite.custom_size = Some(Vec2::splat(1.));

        //could enumerate book
        let boxes: Vec<Entity> = book
            .recipes
            .iter()
            .enumerate()
            .map(|(i, recipe)| {
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: sprite.clone(),
                        texture_atlas: graphics.texture_atlas.clone(),
                        transform: Transform::from_xyz(-7.5, starting_y - i as f32, -1.0),
                        ..Default::default()
                    })
                    .insert(CraftingBox {
                        active: false,
                        recipe_index: i,
                    })
                    .insert(Name::new("CraftingBox"))
                    .with_children(|parent| {
                        let sprite = graphics
                            .item_map
                            .get(&WorldObject::Item(recipe.produces))
                            .expect(&format!("No graphic for item {:?}", recipe.produces))
                            .clone();

                        parent
                            .spawn_bundle(SpriteSheetBundle {
                                sprite,
                                texture_atlas: graphics.texture_atlas.clone(),
                                ..Default::default()
                            })
                            .insert(Name::new("ItemGraphic"));
                    })
                    .id()
            })
            .collect();
        commands.entity(camera_ent).push_children(&boxes);
    }
}
impl Inventory {
    pub fn ingredients_available(&mut self, recipe: &CraftingRecipe) -> bool {
        for ingredient in recipe.needed.clone() {
            if !self.can_remove(&ingredient) {
                return false;
            }
        }
        true
    }
}

fn can_craft(inventory: &Inventory, recipe: &CraftingRecipe) -> bool {
    recipe.needed.iter().all(|item_and_count| {
        inventory.items.iter().any(|item_slot| {
            item_slot.item == item_and_count.item && item_slot.count >= item_and_count.count
        })
    })
}
