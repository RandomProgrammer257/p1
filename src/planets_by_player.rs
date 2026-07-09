use crate::{
    Camera2d, Campos, G, MORESIZE, MouseStates, PI, Planet, PlanetExtraGravZone, PlanetPreGravity,
    PlanetRadius, Star, Transform, spawn_planet,
};

use bevy::input::mouse::MouseButton;
use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

fn get_nearest(
    spawn_point: Vec2,
    spawn_radius: f32,
    spawn_density: f32,
    spawn_zone_size: f32,

    planets_query: Query<
        (
            Entity,
            &Transform,
            &PlanetExtraGravZone,
            &PlanetPreGravity,
            &PlanetRadius,
        ),
        With<Planet>,
    >,
    star_query: Query<
        (
            Entity,
            &Transform,
            &PlanetExtraGravZone,
            &PlanetPreGravity,
            &PlanetRadius,
        ),
        (With<Star>, Without<Planet>),
    >,
) -> Option<Entity> {
    let spawn_mass = 4.0 / 3.0 * PI * spawn_radius.powf(3.0) * spawn_density;
    let spawn_pre_gravity = spawn_mass * G;

    let mut nearest_entity_half_gravity = 0.0;
    let mut nearest_by_gravity_object_1: Option<Entity> = None;

    for (entity, transform_planet, zone, planet_pre_gravity, radius) in &planets_query {
        let dx = spawn_point.x - transform_planet.translation.x;
        let dy = spawn_point.y - transform_planet.translation.y;

        let range = (dx * dx + dy * dy).powf(0.5);

        let half_gravity_x: f32;
        let half_gravity_y: f32;

        if range > radius.0 * zone.0 + spawn_zone_size * spawn_radius {
            half_gravity_x = planet_pre_gravity.0 / G * spawn_pre_gravity
                / (range / spawn_zone_size).powf(2.0)
                * (dx / range);
            half_gravity_y = planet_pre_gravity.0 / G * spawn_pre_gravity
                / (range / spawn_zone_size).powf(2.0)
                * (dy / range);
        } else {
            half_gravity_x = -9.8 * MORESIZE * dx / range.powf(2.0);
            half_gravity_y = -9.8 * MORESIZE * dy / range.powf(2.0);
        }

        let half_gravity_vec = (half_gravity_x.powf(2.0) + half_gravity_y.powf(2.0)).sqrt();

        if half_gravity_vec > nearest_entity_half_gravity {
            nearest_by_gravity_object_1 = Some(entity);
            nearest_entity_half_gravity = half_gravity_vec;
        }
    }

    for (entity, transform_star, _zone, star_pre_gravity, _radius) in &star_query {
        let dx = spawn_point.x - transform_star.translation.x;
        let dy = spawn_point.y - transform_star.translation.y;

        let range = (dx * dx + dy * dy).powf(0.5);

        let half_gravity_x = star_pre_gravity.0 / G * spawn_pre_gravity
            / (range / spawn_zone_size).powf(2.0)
            * (dx / range);
        let half_gravity_y = star_pre_gravity.0 / G * spawn_pre_gravity
            / (range / spawn_zone_size).powf(2.0)
            * (dy / range);

        let half_gravity_vec = (half_gravity_x.powf(2.0) + half_gravity_y.powf(2.0)).sqrt();

        if half_gravity_vec > nearest_entity_half_gravity {
            nearest_by_gravity_object_1 = Some(entity);
            nearest_entity_half_gravity = half_gravity_vec;
        }
    }

    nearest_by_gravity_object_1
}

pub fn planet_spawn_ivent_handler(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mouse_input: Res<ButtonInput<MouseButton>>,
    mouse_state: ResMut<MouseStates>,

    camera_query: Query<&Campos, With<Camera2d>>,

    nearest_by_gravity_query: Query<
        (
            Option<&Transform>,
            Option<&Velocity>,
            Option<&PlanetPreGravity>,
            Option<&PlanetExtraGravZone>,
            Option<&Star>,
            Option<&Planet>,
        ),
        Or<(With<Planet>, With<Star>)>,
    >,

    planets_query: Query<
        (
            Entity,
            &Transform,
            &PlanetExtraGravZone,
            &PlanetPreGravity,
            &PlanetRadius,
        ),
        With<Planet>,
    >,
    star_query: Query<
        (
            Entity,
            &Transform,
            &PlanetExtraGravZone,
            &PlanetPreGravity,
            &PlanetRadius,
        ),
        (With<Star>, Without<Planet>),
    >,
) {
    if mouse_state.planet_spawn_mode && mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(cam_pos) = camera_query.get_single() {
            let radius = MORESIZE * cam_pos.0;
            let pos_x = mouse_state.world_position.x;
            let pos_y = mouse_state.world_position.y;

            let pos_z = 0.0;
            let density = 3.8e8;
            let zone = 10.0;

            let spawn_mass = 4.0 / 3.0 * PI * radius.powf(3.0) * density;

            let mut object_transform = Transform::from_xyz(0.0, 0.0, 0.0);

            let mut object_velocity = Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            };
            let mut object_gravity_param = 0.0;

            let nearest_entity = get_nearest(
                Vec2::new(pos_x, pos_y),
                radius,
                density,
                zone,
                planets_query,
                star_query,
            );

            if let Some(nearest_entity) = nearest_entity {
                if let Ok((transform, velocity, gravity_param, zone_2, star, planet)) =
                    nearest_by_gravity_query.get(nearest_entity)
                {
                    if let Some(t) = transform {
                        object_transform = *t;
                    }
                    if let Some(v) = velocity {
                        object_velocity = *v;
                    }
                    if star.is_some() {
                        if let Some(g) = gravity_param {
                            object_gravity_param = g.0 * zone.powf(2.0);
                        }
                    }
                    if planet.is_some() {
                        if let Some(z) = zone_2
                            && let Some(g) = gravity_param
                        {
                            object_gravity_param = g.0 * z.0.powf(2.0);
                        }
                    }
                }
            }

            let orbital_height = ((pos_x - object_transform.translation.x).powf(2.0)
                + (pos_y - object_transform.translation.y).powf(2.0))
            .sqrt();

            let recomendated_horizontal_velocity =
                ((G * spawn_mass + object_gravity_param) / orbital_height).sqrt();

            let dx = pos_x - object_transform.translation.x;
            let dy = pos_y - object_transform.translation.y;

            let norm_dx = dx / orbital_height;
            let norm_dy = dy / orbital_height;

            let horiz_x = -norm_dy;
            let horiz_y = norm_dx;

            let vel_x = recomendated_horizontal_velocity * horiz_x + object_velocity.linvel.x;
            let vel_y = recomendated_horizontal_velocity * horiz_y + object_velocity.linvel.y;

            spawn_planet(
                (&mut commands, &mut meshes, &mut materials),
                radius,
                (pos_x, pos_y, pos_z),
                density,
                Vec2::new(vel_x, vel_y),
                Color::srgba(0.086, 0.259, 0.157, 1.0),
                zone,
            );
        }
    }
}
