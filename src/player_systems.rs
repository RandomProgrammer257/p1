use crate::{
    AdditionalMassProperties, Dir, ExternalForce, IsFly, MORESIZE, PI, Planet, PlanetExtraGravZone,
    PlanetPreGravity, PlanetRadius, PlayerCollis, Rec, Transform, Velocity,
};

use bevy::prelude::*;

pub fn player_gravity_system(
    mut planet_query: Query<
        (
            &PlanetPreGravity,
            &mut ExternalForce,
            &Transform,
            &PlanetRadius,
            &PlanetExtraGravZone,
        ),
        With<Planet>,
    >,
    mut player_query: Query<
        (
            &mut Transform,
            &mut ExternalForce,
            &mut Velocity,
            &AdditionalMassProperties,
            &IsFly,
            &PlayerCollis,
        ),
        (With<Rec>, Without<Planet>),
    >,
) {
    for (mut transform, mut external_force, mut velocity, mass, fly, _collision) in
        player_query.iter_mut()
    {
        let get_mass = match *mass {
            AdditionalMassProperties::Mass(m) => m,
            _ => 0.0,
        };
        let mut full_ext_forse = (0.0, 0.0);
        let mut min_dx = 0.0;
        let mut min_dy = 0.0;
        let mut range_m = f32::INFINITY;

        for (planet_pre_gravity, mut external_force_planet, transform_planet, radius, zone) in
            &mut planet_query
        {
            let dx = transform_planet.translation.x - transform.translation.x;
            let dy = transform_planet.translation.y - transform.translation.y;

            let range = (dx * dx + dy * dy).sqrt();

            if range < f32::EPSILON {
                continue;
            }

            let mut gravity_x = 0.0;
            let mut gravity_y = 0.0;

            if range > radius.0 * zone.0 {
                gravity_x = planet_pre_gravity.0 * get_mass / (range / zone.0).powf(2.0)
                    * (dx / range)
                    * 32.0;
                gravity_y = planet_pre_gravity.0 * get_mass / (range / zone.0).powf(2.0)
                    * (dy / range)
                    * 32.0;
            }

            if range <= radius.0 * zone.0 {
                gravity_x = 9.8 * MORESIZE * dx * get_mass / range.powf(1.0);
                gravity_y = 9.8 * MORESIZE * dy * get_mass / range.powf(1.0);
            }
            if range < range_m {
                min_dx = dx;
                min_dy = dy;
                range_m = range;
            }

            full_ext_forse.0 += gravity_x;
            full_ext_forse.1 += gravity_y;

            external_force_planet.force.x -= gravity_x;
            external_force_planet.force.y -= gravity_y;
        }

        if !fly.0 {
            if range_m > f32::EPSILON {
                let direction = Vec2::new(min_dx, min_dy).normalize();
                let angle = direction.y.atan2(direction.x);
                transform.rotation = Quat::from_rotation_z(angle + PI / 2.0);
            }
            velocity.angvel = 0.0;
        }
        external_force.force.x += full_ext_forse.0;
        external_force.force.y += full_ext_forse.1;
    }
}

pub fn player_control_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<
        (
            &Transform,
            &mut ExternalForce,
            &mut Velocity,
            &mut Dir,
            &mut IsFly,
            &PlayerCollis,
        ),
        With<Rec>,
    >,
) {
    for (transform, mut external_force, mut velocity, mut direction, mut fly, is_collis) in
        player_query.iter_mut()
    {
        let mut full_ext_forse = (0.0, 0.0);
        let full_velocity = (0.0, 0.0);

        if keys.just_pressed(KeyCode::KeyF) {
            fly.0 = !fly.0;
        }

        if !fly.0 {
            if is_collis.0 {
                if keys.pressed(KeyCode::KeyD) {
                    full_ext_forse.0 += (direction.0 + PI / 2.0).cos() * 64.0;
                    full_ext_forse.1 += (direction.0 + PI / 2.0).sin() * 64.0;
                }

                if keys.pressed(KeyCode::KeyA) {
                    full_ext_forse.0 += (direction.0 - PI / 2.0).cos() * 64.0;
                    full_ext_forse.1 += (direction.0 - PI / 2.0).sin() * 64.0;
                }

                if keys.just_pressed(KeyCode::KeyW) {
                    full_ext_forse.0 -= (direction.0).cos() * 64.0 * 10.0;
                    full_ext_forse.1 -= (direction.0).sin() * 64.0 * 10.0;
                }
            }
        } else {
            if keys.pressed(KeyCode::KeyD) {
                external_force.torque -=
                    10.0 * MORESIZE * MORESIZE * MORESIZE * MORESIZE * MORESIZE;
            }

            if keys.pressed(KeyCode::KeyA) {
                external_force.torque +=
                    10.0 * MORESIZE * MORESIZE * MORESIZE * MORESIZE * MORESIZE;
            }

            if keys.pressed(KeyCode::KeyW) {
                full_ext_forse.0 -= direction.0.cos() * 5.0 * 64.0;
                full_ext_forse.1 -= direction.0.sin() * 5.0 * 64.0;
            }
            if keys.pressed(KeyCode::KeyS) {
                full_ext_forse.0 += direction.0.cos() * 5.0 * 64.0;
                full_ext_forse.1 += direction.0.sin() * 5.0 * 64.0;
            }
        }

        let forward = transform.local_y();
        direction.0 = forward.y.atan2(forward.x);

        velocity.linvel.x += full_velocity.0;
        velocity.linvel.y += full_velocity.1;
        external_force.force.x -= full_ext_forse.0 * MORESIZE * MORESIZE * MORESIZE * MORESIZE;
        external_force.force.y -= full_ext_forse.1 * MORESIZE * MORESIZE * MORESIZE * MORESIZE;
    }
}
