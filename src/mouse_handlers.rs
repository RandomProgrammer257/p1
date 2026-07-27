//! Модуль обработки мыши
//!
//! Этот модуль содержит компоненты, ресурсы и системы для отслеживания
//! позиции мыши и обработки ввода в игре.

use crate::{Camera2d, Campos, MORESIZE, Rec, Transform, Window};
use bevy::prelude::*;

/// Компонент-маркер для визуального отображения курсора мыши
#[derive(Component)]
pub struct Mouse;

/// Ресурс с состоянием мыши
///
/// Хранит текущую позицию мыши в координатах окна и мира,
/// а также флаги состояния и режимы спавна.
#[derive(Resource, Default)]
pub struct MouseStates {
    /// Позиция мыши в координатах окна
    pub window_position: Vec2,
    /// Позиция мыши в мировых координатах
    pub world_position: Vec2,
    /// Флаг: находится ли мышь внутри окна
    pub is_in_window: bool,
    /// Текущий размер отображения мыши
    pub size: f32,
    /// Режим спавна планет по клику мыши
    pub planet_spawn_mode: bool,
}

/// Bundle для создания визуального маркера мыши
#[derive(Bundle)]
pub struct MouseBundle {
    /// 2D меш для отображения
    pub mesh: Mesh2d,
    /// Материал меша
    pub material: MeshMaterial2d<ColorMaterial>,
    /// Трансформация маркера
    pub transform: Transform,
    /// Компонент-маркер мыши
    pub mouse_mark: Mouse,
}

/// Система создания маркера мыши на старте
///
/// Создает визуальный маркер для отображения текущей позиции курсора
/// и прикрепляет его к камере.
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

/// Система обновления позиции мыши
///
/// Вычисляет текущую позицию курсора в координатах окна и мира,
/// обновляет ресурс `MouseStates` и позиционирует визуальный маркер.
///
/// # Особенности
/// * Позиция в мире вычисляется с учетом позиции игрока и масштаба камеры
/// * Масштаб маркера изменяется в зависимости от зума камеры
/// * Если мышь за пределами окна, флаг `is_in_window` устанавливается в `false`
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
