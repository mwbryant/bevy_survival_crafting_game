use crate::{crafting::CRAFTING_BOX_SIZE, inventory::InventoryBox, prelude::PIXEL_SIZE};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::spawn_player)
            .add_system(Self::player_movement)
            .add_system(Self::player_pickup)
            .add_system(Self::player_equip)
            .register_inspectable::<Hands>()
            .register_inspectable::<Player>();
    }
}

#[derive(Component, Inspectable, Default)]
pub struct Hands {
    left: Option<Tool>,
}

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    arm_length: f32,
}

impl PlayerPlugin {
    fn player_equip(
        mut player_query: Query<(&Player, &mut Inventory, &mut Hands)>,
        inventory_boxes: Query<(&GlobalTransform, &InventoryBox)>,
        mouse: Res<Input<MouseButton>>,
        mouse_pos: Res<MousePosition>,
    ) {
        let (_, mut inventory, mut hands) = player_query.single_mut();
        if mouse.just_pressed(MouseButton::Left) {
            for (transform, inv_box) in inventory_boxes.iter() {
                //FIXME this uses a magic number, is there a better way
                if !(mouse_pos.0.x > transform.translation.x - 0.5 * CRAFTING_BOX_SIZE
                    && mouse_pos.0.x < transform.translation.x + 0.5 * CRAFTING_BOX_SIZE
                    && mouse_pos.0.y > transform.translation.y - 0.5 * CRAFTING_BOX_SIZE
                    && mouse_pos.0.y < transform.translation.y + 0.5 * CRAFTING_BOX_SIZE)
                {
                    continue;
                }
                if let ItemType::Tool(tool) = inventory.items[inv_box.slot].item {
                    if let Err(error) = inventory.remove(&ItemAndCount {
                        item: ItemType::Tool(tool),
                        count: 1,
                    }) {
                        warn!("{:?}", error);
                    };
                    if let Some(tool) = hands.left {
                        if inventory.add(&ItemAndCount {
                            item: ItemType::Tool(tool),
                            count: 1,
                        }).is_some() {
                            //FIXME removing what was in hand might not be able to go back into inventory
                            warn!("Item was lost! on unequip");
                        }
                    }
                    hands.left = Some(tool);
                }
            }
        }
    }

    //XXX is this better to be 2 systems... its a bit much
    fn player_pickup(
        mut commands: Commands,
        keyboard: Res<Input<KeyCode>>,
        mut player_query: Query<(&Transform, &Player, &mut Inventory, &Hands)>,
        pickupable_query: Query<
            (
                Entity,
                &Transform,
                Option<&Pickupable>,
                Option<&Harvestable>,
            ),
            //XXX assuming anything with pickup is never harvestable
            //How to enforce these components are mutually exclusive
            (Or<(With<Pickupable>, With<Harvestable>)>, Without<Player>),
        >,
        graphics: Res<Graphics>,
    ) {
        let (player_transform, player, mut inventory, hands) = player_query.single_mut();
        //Press space to pickup items
        //TODO if held walk to nearest
        if !keyboard.just_pressed(KeyCode::Space) {
            return;
        }
        if let Some((ent, transform, pickup, harvest)) = pickupable_query
            .iter()
            .filter_map(|(ent, transform, pickup, harvest)| {
                let distance = transform
                    .translation
                    .truncate()
                    .distance(player_transform.translation.truncate());
                if player.arm_length > distance {
                    Some((ent, transform, distance, pickup, harvest))
                } else {
                    None
                }
            })
            //.filter(|(_, _, pickup)| can_pickup(&inventory, pickup.item))
            .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Greater))
            .map(|(ent, transform, _, pickup, harvest)| (ent, transform, pickup, harvest))
        {
            if let Some(pickup) = pickup {
                let pickup_and_count = ItemAndCount {
                    item: pickup.item,
                    count: 1,
                };
                if inventory.can_add(pickup_and_count) {
                    inventory.add(&pickup_and_count);
                    commands.entity(ent).despawn_recursive();
                } else {
                    info!("no available slot for item: {}", pickup_and_count);
                }
            }
            if let Some(harvest) = harvest {
                let harvest_and_count = ItemAndCount {
                    item: harvest.item,
                    count: 1,
                };
                if inventory.can_add(harvest_and_count) {
                    if hands.left == harvest.tool_required || harvest.tool_required.is_none() {
                        inventory.add(&harvest_and_count);
                        commands.entity(ent).despawn_recursive();
                        if let Some(new_object) = harvest.drops {
                            //Become what you always were meant to be
                            //println!("Pickupable found its new life as a {:?}", new_object);
                            new_object.spawn(
                                &mut commands,
                                &graphics,
                                transform.translation.truncate(),
                            );
                        }
                    }
                } else {
                    info!("no available slot for item: {}", harvest_and_count);
                }
            }
        }
    }

    fn player_movement(
        keyboard: Res<Input<KeyCode>>,
        time: Res<Time>,
        mut player_query: Query<(&mut Transform, &Player)>,
    ) {
        let (mut player_transform, player) = player_query.single_mut();

        if keyboard.pressed(KeyCode::A) {
            player_transform.translation.x -= player.speed * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::D) {
            player_transform.translation.x += player.speed * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::W) {
            player_transform.translation.y += player.speed * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::S) {
            player_transform.translation.y -= player.speed * time.delta_seconds();
        }
    }

    fn spawn_player(mut commands: Commands, graphics: Res<Graphics>) {
        let mut sprite = TextureAtlasSprite::new(graphics.player_index);
        sprite.custom_size = Some(Vec2::splat(PIXEL_SIZE * 32.0));
        sprite.anchor = Anchor::Custom(Vec2::new(0.0, 0.5 - 30.0 / 32.0));
        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite,
                texture_atlas: graphics.texture_atlas.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 700.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player {
                speed: 0.3,
                arm_length: 0.1,
            })
            .insert(Inventory::default())
            .insert(Hands::default())
            .insert(Name::new("Player"));
    }
}
