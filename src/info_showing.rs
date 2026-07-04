use crate::{
    Camera2d, CameraWorldAngle, Campos, Children, FrameComponent, PI, Planet, PlanetInfo,
    PlanetInfoText, PlanetInfoZone, PlanetRadius, Rec, Text2d, Transform, TextLayoutInfo, PlanetVolume, PlanetDensity, Velocity, AdditionalMassProperties, PlanetExtraGravZone
};

use bevy::prelude::*;

//Обработка размеров и поворота информации
//Добавь условие обработки информации ^ : только при отображении (наведении мышки)
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
                        && (planet_radius.0 + size.y * cam_pos.0 * 0.5).abs() >= planet_radius.0
                        {
                            text_transform.translation.x = (planet_radius.0 + size.x * cam_pos.0 * 0.15) * (text_transform.translation.x / text_transform.translation.x.abs());
                            text_transform.translation.y = (planet_radius.0 + size.y * cam_pos.0 * 0.15) * (text_transform.translation.y / text_transform.translation.y.abs());
                            
                        }
                        }
                    }
                }
            }
        }
    }
}

//Обработка размеров и поворота рамки
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

//Передача информации о планете в текстовое поле
pub fn planet_get_info_handler(
    planet_query: Query<
        (&Transform, &PlanetVolume, &PlanetDensity, &Velocity, &PlanetRadius, &AdditionalMassProperties, &PlanetExtraGravZone, &Children),
        (
            With<Planet>,
            Without<Camera2d>,
            Without<PlanetInfo>,
            Without<PlanetInfoText>,
        ),
    >,

    mut info_query: Query<
        &Children,
        (With<PlanetInfo>, Without<Camera2d>, Without<Planet>),
    >,

    mut info_text_query: Query<
        &mut Text2d,
        (
            With<PlanetInfoText>,
            Without<Planet>,
            Without<Rec>,
            Without<Camera2d>,
            Without<PlanetInfo>,
        ),
    >,
) {
        for (planet_transform, planet_volume, planet_density, planet_velocity, planet_radius, planet_mass, extra_grav_zone, planet_children) in &planet_query {
             
            let planet_mass = match *planet_mass {
            AdditionalMassProperties::Mass(m) => m,
            _ => 0.0,

        };
            for &child_entity in planet_children.iter() {
                if let Ok(info_text) = info_query.get_mut(child_entity) {
                    for &info_text in info_text.iter() {
                        if let Ok( mut text) =
                            info_text_query.get_mut(info_text)
                        {
                            let planet_position = planet_transform.translation;

                            let orbital_height = (planet_position.x.powf(2.0) + planet_position.y.powf(2.0)).sqrt();

                            let planet_linvel = (planet_velocity.linvel.x.powf(2.0) + planet_velocity.linvel.y.powf(2.0)).sqrt();
                            let planet_angvel = planet_velocity.angvel;

                            let year_period = 2.0 * PI * orbital_height / planet_linvel;

                            let day_period = 2.0 * PI * planet_radius.0 / planet_angvel;

                            text.0 = (format!("
                            Orbital height (Sun): {:.3}\n
                            Radius: {:.3}\n
                            Volume: {:.3}\n
                            Design density: {:.3}\n
                            Design mass: {:.3}\n
                            Orbital speed: {:.3} m/s\n
                            Day lenght: {:.3} sec\n
                            Year length: {:.3} sec\n
                            Extra gravity zone size: {:.3}\n
                            ", orbital_height, planet_radius.0, planet_volume.0, planet_density.0, planet_mass, planet_linvel, day_period, year_period, extra_grav_zone.0 * planet_radius.0)).to_string();
                           
                        }
                    }
                }
            }
        }
    
}