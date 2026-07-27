//! Модуль отображения информации о планетах
//!
//! Содержит системы для отображения информации о планетах (текст, рамки)
//! с учетом масштаба камеры и поворота.

use crate::{
    AdditionalMassProperties, Camera2d, CameraWorldAngle, Campos, Children, FrameComponent, G,
    MORESIZE, MouseStates, NearestObject, PI, Planet, PlanetDensity, PlanetExtraGravZone,
    PlanetInfo, PlanetInfoText, PlanetInfoZone, PlanetPreGravity, PlanetRadius, PlanetVolume, Rec,
    Star, Text2d, TextLayoutInfo, Transform, Velocity, Visibility,
};

use bevy::input::mouse::MouseButton;
use bevy::prelude::*;

/// Система обработки размеров и поворота информационных текстов планет
///
/// Обновляет трансформации текстовой информации о планетах:
/// * Поворачивает текст в зависимости от угла камеры
/// * Масштабирует текст в зависимости от зума камеры
/// * Позиционирует текст на границе радиуса планеты
///
/// # Условия работы
/// * Информация отображается только при наведении мыши (управляется через `visibility_control_handler`)
pub fn planet_info_handler(
    camera_query: Query<
        (&Campos, &CameraWorldAngle),
        (
            With<Camera2d>,
            Without<Planet>,
            Without<PlanetInfo>,
            Without<PlanetInfoText>,
        ),
    >,
    planet_query: Query<
        (&Transform, &Children, &PlanetRadius),
        (
            With<Planet>,
            Without<Camera2d>,
            Without<PlanetInfo>,
            Without<PlanetInfoText>,
        ),
    >,

    mut info_query: Query<
        (&mut Transform, &Children),
        (With<PlanetInfo>, Without<Camera2d>, Without<Planet>),
    >,

    mut info_text_query: Query<
        (&mut Transform, &TextLayoutInfo),
        (
            With<PlanetInfoText>,
            Without<Planet>,
            Without<Rec>,
            Without<Camera2d>,
            Without<PlanetInfo>,
        ),
    >,
) {
    for (cam_pos, camera_angle) in &camera_query {
        for (planet_transform, planet_children, planet_radius) in &planet_query {
            for &child_entity in planet_children.iter() {
                if let Ok((mut info_transform, info_text)) = info_query.get_mut(child_entity) {
                    let planet_angle = planet_transform.rotation.to_euler(EulerRot::XYZ).2;

                    info_transform.rotation =
                        Quat::from_rotation_z(camera_angle.0 - planet_angle - PI / 2.0);
                    for &info_text in info_text.iter() {
                        if let Ok((mut text_transform, text_layout)) =
                            info_text_query.get_mut(info_text)
                        {
                            text_transform.scale.x = cam_pos.0 * 0.5;
                            text_transform.scale.y = cam_pos.0 * 0.5;

                            let size: Vec2 = text_layout.size;

                            if (planet_radius.0 + size.x * cam_pos.0 * 0.5).abs() >= planet_radius.0
                                && (planet_radius.0 + size.y * cam_pos.0 * 0.5).abs()
                                    >= planet_radius.0
                            {
                                text_transform.translation.x = (planet_radius.0
                                    + size.x * cam_pos.0 * 0.15)
                                    * (text_transform.translation.x
                                        / text_transform.translation.x.abs());
                                text_transform.translation.y = (planet_radius.0
                                    + size.y * cam_pos.0 * 0.15)
                                    * (text_transform.translation.y
                                        / text_transform.translation.y.abs());
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Система обработки размеров и поворота рамки планеты
///
/// Обновляет трансформации рамки вокруг планеты:
/// * Поворачивает рамку в соответствии с углом камеры
/// * Масштабирует компоненты рамки в зависимости от зума
pub fn planet_frame_handler(
    mut planet_frame_query: Query<(&mut Transform, &Children), With<PlanetInfoZone>>,
    mut frame_component_query: Query<
        &mut Transform,
        (Without<PlanetInfoZone>, With<FrameComponent>),
    >,

    planet_query: Query<
        (&Transform, &Children),
        (
            With<Planet>,
            Without<PlanetInfoZone>,
            Without<FrameComponent>,
        ),
    >,
    camera_query: Query<(&CameraWorldAngle, &Campos), With<Camera2d>>,
) {
    for (camera_angle, cam_pos) in camera_query.iter() {
        for (planet_transform, planet_children) in &planet_query {
            for &frame in planet_children.iter() {
                if let Ok((mut frame_transform, frame_children)) = planet_frame_query.get_mut(frame)
                {
                    let planet_angle = planet_transform.rotation.to_euler(EulerRot::XYZ).2;
                    frame_transform.rotation =
                        Quat::from_rotation_z(camera_angle.0 - planet_angle - PI / 2.0);

                    for &frame_children in frame_children.iter() {
                        if let Ok(mut frame_component_transform) =
                            frame_component_query.get_mut(frame_children)
                        {
                            frame_component_transform.scale = Vec3::new(1.0, cam_pos.0, 1.0);
                        }
                    }
                }
            }
        }
    }
}

/// Система передачи информации о планете в текстовое поле
///
/// Собирает все параметры планеты и формирует текстовую информацию
/// для отображения в интерфейсе:
/// * Орбитальная высота
/// * Радиус, объем, масса
/// * Скорость (линейная и угловая)
/// * Параметры орбиты (большая полуось, период)
/// * Информация о гравитационном воздействии
pub fn planet_get_info_handler(
    planet_query: Query<
        (
            &Transform,
            &PlanetVolume,
            &PlanetDensity,
            &Velocity,
            &PlanetRadius,
            &AdditionalMassProperties,
            &PlanetExtraGravZone,
            &PlanetPreGravity,
            &NearestObject,
            &Children,
        ),
        (
            With<Planet>,
            Without<Camera2d>,
            Without<PlanetInfo>,
            Without<PlanetInfoText>,
            Without<Star>,
        ),
    >,

    mut info_query: Query<
        &Children,
        (
            With<PlanetInfo>,
            Without<Camera2d>,
            Without<Planet>,
            Without<Star>,
        ),
    >,

    mut info_text_query: Query<
        &mut Text2d,
        (
            With<PlanetInfoText>,
            Without<Planet>,
            Without<Rec>,
            Without<Camera2d>,
            Without<PlanetInfo>,
            Without<Star>,
        ),
    >,
    nearest_by_gravity_query: Query<
        (
            Option<&Transform>,
            Option<&Velocity>,
            Option<&PlanetPreGravity>,
            Option<&PlanetExtraGravZone>,
            Option<&AdditionalMassProperties>,
            Option<&Star>,
            Option<&Planet>,
        ),
        Or<(With<Planet>, With<Star>)>,
    >,
) {
    for (
        planet_transform,
        planet_volume,
        planet_density,
        planet_velocity,
        planet_radius,
        planet_mass,
        extra_grav_zone,
        gravity_param,
        nearest_object,
        planet_children,
    ) in &planet_query
    {
        let planet_mass = match *planet_mass {
            AdditionalMassProperties::Mass(m) => m,
            _ => 0.0,
        };

        let mut object_transform = Transform::from_xyz(0.0, 0.0, 0.0);

        let mut object_velocity = Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        };
        let mut object_gravity_param = 0.0;

        let mut master = "".to_string();

        if let Some(nearest_entity) = nearest_object.0 {
            if let Ok((transform, velocity, gravity_param, zone, _mass, star, planet)) =
                nearest_by_gravity_query.get(nearest_entity)
            {
                if let Some(t) = transform {
                    object_transform = *t;
                }
                if let Some(v) = velocity {
                    object_velocity = *v;
                }
                if star.is_some() {
                    master = "star".to_string();

                    if let Some(g) = gravity_param {
                        object_gravity_param = g.0 * extra_grav_zone.0.powf(2.0);
                    }
                }
                if planet.is_some() {
                    if let Some(z) = zone
                        && let Some(g) = gravity_param
                    {
                        object_gravity_param = g.0 * z.0.powf(2.0);
                    }

                    master = "planet".to_string();
                }
            }
        }
        for &child_entity in planet_children.iter() {
            if let Ok(info_text) = info_query.get_mut(child_entity) {
                for &info_text in info_text.iter() {
                    if let Ok(mut text) = info_text_query.get_mut(info_text) {
                        let planet_position = planet_transform.translation;
                        let gravity_object_position = object_transform.translation;

                        let orbital_height = ((planet_position.x - gravity_object_position.x)
                            .powf(2.0)
                            + (planet_position.y - gravity_object_position.y).powf(2.0))
                        .sqrt();

                        let planet_linvel = ((planet_velocity.linvel.x - object_velocity.linvel.x)
                            .powf(2.0)
                            + (planet_velocity.linvel.y - object_velocity.linvel.y).powf(2.0))
                        .sqrt();

                        let planet_angvel = planet_velocity.angvel;

                        let orbital_axis_a = 1.0
                            / (2.0 / orbital_height
                                - planet_linvel.powf(2.0) / object_gravity_param);

                        let year_period =
                            2.0 * PI * (orbital_axis_a.powf(3.0) / object_gravity_param).sqrt();

                        let day_period = 2.0 * PI / planet_angvel;

                        let recomendated_horizontal_velocity =
                            ((G * planet_mass + object_gravity_param) / orbital_height).sqrt();

                        text.0 = (format!(
                            "
                            Orbital height: {:.3} m
                            Radius: {:.3} m
                            Volume: {:.3} m^3
                            Design density: {:.3} kg/m^3
                            Design mass: {:.3} kg
                            Master: {}
                            Orbital speed: {:.3} m/s
                            Recomendated speed: {:.3} m/s
                            Day lenght: {:.3} sec
                            Year length: {:.3} sec
                            Extra gravity zone size: {:.3} m
                            Gravity parameter: {:.3} m^3/c^2
                            Big half axis: {:.3} m
                            ",
                            orbital_height / MORESIZE,
                            planet_radius.0 / MORESIZE,
                            planet_volume.0 / MORESIZE.powf(3.0),
                            planet_density.0,
                            planet_mass,
                            master,
                            planet_linvel / MORESIZE,
                            recomendated_horizontal_velocity / MORESIZE,
                            day_period,
                            year_period,
                            extra_grav_zone.0 * planet_radius.0 / MORESIZE,
                            gravity_param.0 / MORESIZE.powf(3.0),
                            orbital_axis_a / MORESIZE
                        ))
                        .to_string();
                    }
                }
            }
        }
    }
}

/// Система управления видимостью информационных элементов
///
/// Управляет отображением информации о планетах:
/// * Клавиша `I` - переключение видимости рамок всех планет
/// * Клик левой кнопкой мыши по планете - переключение текстовой информации
///
/// # Поведение
/// * При нажатии `I` скрываются/показываются все рамки планет
/// * При клике по планете переключается отображение её текстовой информации
/// * Текстовая информация отображается только если рамка планеты видима
pub fn visibility_control_handler(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mouse_state: ResMut<MouseStates>,

    mut planet_info_query: Query<&mut Visibility, With<PlanetInfo>>,

    mut planet_frame_query: Query<&mut Visibility, (With<PlanetInfoZone>, Without<PlanetInfo>)>,

    planet_query: Query<(&Transform, &PlanetRadius, &Children), With<Planet>>,
) {
    if keys.just_pressed(KeyCode::KeyI) {
        for mut visibility in planet_frame_query.iter_mut() {
            if *visibility == Visibility::Visible {
                for mut frame_vis in planet_info_query.iter_mut() {
                    *frame_vis = Visibility::Hidden;
                }
                *visibility = Visibility::Hidden;
            } else if *visibility == Visibility::Hidden {
                *visibility = Visibility::Visible;
            }
        }
    }

    if mouse_input.just_pressed(MouseButton::Left) {
        for (planet_transform, planet_radius, children) in planet_query.iter() {
            let mut is_vis = false;

            for &child_entity in children.iter() {
                if let Ok(planet_frame_vis) = planet_frame_query.get_mut(child_entity) {
                    if *planet_frame_vis == Visibility::Visible {
                        is_vis = true;
                    }
                }
            }

            let range = ((planet_transform.translation.x - mouse_state.world_position.x).powf(2.0)
                + (planet_transform.translation.y - mouse_state.world_position.y).powf(2.0))
            .sqrt();

            if range < planet_radius.0 {
                for &child_entity in children.iter() {
                    if let Ok(mut planet_info_vis) = planet_info_query.get_mut(child_entity) {
                        if *planet_info_vis == Visibility::Visible {
                            *planet_info_vis = Visibility::Hidden;
                        } else if *planet_info_vis == Visibility::Hidden && is_vis {
                            *planet_info_vis = Visibility::Visible;
                        }
                    }
                }
            }
        }
    }
}
