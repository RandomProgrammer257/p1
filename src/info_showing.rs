use crate::{
    Camera2d, CameraWorldAngle, Campos, Children, FrameComponent, PI, Planet, PlanetInfo,
    PlanetInfoText, PlanetInfoZone, Transform,
};

use bevy::prelude::*;

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
        (&Transform, &Children),
        (
            With<Planet>,
            Without<Camera2d>,
            Without<PlanetInfo>,
            Without<PlanetInfoText>,
        ),
    >,

    mut info_query: Query<&mut Transform, (With<PlanetInfo>, Without<Camera2d>, Without<Planet>)>,
    // info_text_query: Query<
    //     &mut Transform,
    //     (
    //         With<PlanetInfoText>,
    //         Without<Planet>,
    //         Without<Rec>,
    //         Without<Camera2d>,
    //         Without<PlanetInfo>,
    //     ),
    // >,
) {
    for (cam_pos, camera_angle) in &camera_query {
        for (planet_transform, planet_children) in &planet_query {
            for &child_entity in planet_children.iter() {
                if let Ok(mut info_transform) = info_query.get_mut(child_entity) {
                    let planet_angle = planet_transform.rotation.to_euler(EulerRot::XYZ).2;

                    info_transform.rotation =
                        Quat::from_rotation_z(camera_angle.0 - planet_angle - PI / 2.0);
                }
            }
        }

        for mut info_transform in info_query.iter_mut() {
            info_transform.scale = Vec3::new(cam_pos.0, cam_pos.0, cam_pos.0);
        }
    }
}

//Добавь условие обработки информации ^ : только при отображении (наведении мышки)
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
