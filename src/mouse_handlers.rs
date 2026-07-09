use crate::{Camera2d, Campos, MORESIZE, Rec, Transform, Window};
use bevy::prelude::*;

#[derive(Component)]
pub struct Mouse;

#[derive(Resource, Default)]
pub struct MouseStates {
    pub window_position: Vec2,
    pub world_position: Vec2,
    pub is_in_window: bool,
    pub size: f32,
    pub planet_spawn_mode: bool,
}

#[derive(Bundle)]
pub struct MouseBundle {
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub mouse_mark: Mouse,
}

pub fn mouse_spawn_hander(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    camera_query: Query<Entity, With<Camera2d>>,
) {
    let mouse = commands
        .spawn(MouseBundle {
            mesh: Mesh2d(meshes.add(Circle::new(1.0 * MORESIZE))),
            material: MeshMaterial2d(materials.add(Color::srgba(0.69, 0.35, 0.17, 1.0))),
            transform: Transform::from_xyz(0.0, 0.0, 30.0),
            mouse_mark: Mouse,
        })
        .id();

    if let Ok(parent_entity) = camera_query.get_single() {
        commands.entity(parent_entity).add_child(mouse);
    }
}

pub fn mouse_position_handler(
    mut mouse_state: ResMut<MouseStates>,
    window_query: Query<&Window>,
    player_query: Query<&Transform, With<Rec>>,
    camera_query: Query<&Campos, (With<Camera2d>, Without<Rec>)>,

    mut mouse_query: Query<&mut Transform, (With<Mouse>, Without<Camera2d>, Without<Rec>)>,
) {
    let window = window_query.single();

    if let Some(cursor_position) = window.cursor_position() {
        mouse_state.is_in_window = true;

        mouse_state.window_position.x = cursor_position.x;
        mouse_state.window_position.y = cursor_position.y;

        let physical_width = window.resolution.physical_width();
        let physical_height = window.resolution.physical_height();

        if let Ok(player_transform) = player_query.get_single() {
            if let Ok(cam_pos) = camera_query.get_single() {
                if let Ok(mut mouse_transform) = mouse_query.get_single_mut() {
                    mouse_transform.translation.y =
                        -(mouse_state.window_position.y - (physical_height / 2) as f32) * cam_pos.0;
                    mouse_transform.translation.x =
                        (mouse_state.window_position.x - (physical_width / 2) as f32) * cam_pos.0;

                    mouse_state.world_position.x =
                        mouse_transform.translation.x + player_transform.translation.x;
                    mouse_state.world_position.y =
                        mouse_transform.translation.y + player_transform.translation.y;

                    mouse_transform.scale = Vec3::new(cam_pos.0, cam_pos.0, 1.0);
                    mouse_state.size = MORESIZE * cam_pos.0;
                }
            }
        }
    } else {
        mouse_state.is_in_window = false;
    }
}
