use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::{
    inventory::can_pickup,
    prelude::{give_inventory_item, Inventory, Pickupable, PlaceHolderGraphics},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::spawn_player)
            .add_system(Self::player_movement)
            .add_system(Self::player_pickup)
            .register_inspectable::<Player>();
    }
}

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    arm_length: f32,
}

impl PlayerPlugin {
    fn player_pickup(
        mut commands: Commands,
        keyboard: Res<Input<KeyCode>>,
        mut player_query: Query<(&Transform, &Player, &mut Inventory)>,
        pickupable_query: Query<(Entity, &Transform, &Pickupable), Without<Player>>,
    ) {
        let (player_transform, player, mut inventory) = player_query.single_mut();
        //Press space to pickup items
        //TODO if held walk to nearest
        if keyboard.just_pressed(KeyCode::Space) {
            if let Some((ent, pickup)) = pickupable_query
                .iter()
                .filter_map(|(ent, transform, pickup)| {
                    let distance = transform
                        .translation
                        .truncate()
                        .distance(player_transform.translation.truncate());
                    if player.arm_length > distance {
                        Some((ent, distance, pickup))
                    } else {
                        None
                    }
                })
                .filter(|(_, _, pickup)| can_pickup(&inventory, pickup.item))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Greater))
                .map(|(ent, _, pickup)| (ent, pickup))
            {
                give_inventory_item(&mut inventory, pickup.item);
                //TODO not always despawn i guess
                commands.entity(ent).despawn_recursive();
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

    fn spawn_player(mut commands: Commands, graphics: Res<PlaceHolderGraphics>) {
        let mut sprite = TextureAtlasSprite::new(graphics.player_index);
        sprite.custom_size = Some(Vec2::splat(0.3));
        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: sprite,
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
            .insert(Name::new("Player"));
    }
}
