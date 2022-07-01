use crate::{
    fire_graphics::{spawn_fire, ActiveFires},
    prelude::*,
    GameState,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct PlaceableGhost {
    pub to_place: Option<WorldObject>,
}

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Main).with_system(build_ghost));
    }
}

//TODO it would be nice to not need active fires here
fn build_ghost(
    mut commands: Commands,
    graphics: Res<Graphics>,
    mut player: Query<&mut PlaceableGhost>,
    mouse_input: Res<Input<MouseButton>>,
    mut active_fires: ResMut<ActiveFires>,
    mouse_position: Res<MousePosition>,
) {
    let mut ghost = player.single_mut();

    if mouse_input.just_pressed(MouseButton::Left) && ghost.to_place.is_some() {
        match ghost.to_place.unwrap() {
            WorldObject::CampFire => {
                spawn_fire(
                    &mut commands,
                    &mut active_fires,
                    &graphics,
                    **mouse_position,
                );
                ghost.to_place = None;
            }
            _ => {
                unreachable!("Other placement not implmented!");
            }
        }
    }
}
