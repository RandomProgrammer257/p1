use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

static PI: f32 = std::f32::consts::PI;
static G: f32 = 6.67 * 0.000_000_000_01;

#[derive(Component)]
struct Planet;

#[derive(Component)]
struct PlanetId(i32);

#[derive(Component)]
struct PlanetPreGravity(f32);

#[derive(Component)]
struct PlanetDensity(f32);

#[derive(Component)]
struct PlanetVolume(f32);

#[derive(Component)]
struct IsFly(bool);

#[derive(Component)]
struct Rec;

#[derive(Component)]
struct Dir(f32);

#[derive(Component)]
struct Campos(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Startup, world_spawn)
        .add_systems(
            Update,
            (all_player_moving, player_system, world_gravity_sistem).chain(),
        )
        .add_systems(Update, camera_system)
        .run();
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    commands.spawn((Camera2d, Campos(10.0)));

    let x = 1380.0;
    let y = 0.0;

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(5.0, 20.0))),
        MeshMaterial2d(materials.add(Color::srgba(0.69, 0.35, 0.17, 1.0))),
        Transform::from_xyz(x, y, 0.1),
        RigidBody::Dynamic,
        Collider::cuboid(2.5, 10.0),
        Velocity::linear(Vec2::new(0.0, 0.0)),
        GravityScale(0.0),
        AdditionalMassProperties::Mass(5.0),
        Restitution::coefficient(0.0),
        Friction::coefficient(0.8),
        ExternalForce {
            force: Vec2::new(0.0, 0.0), // Сила в Ньютонах (вправо)
            torque: 0.0,
        },
        IsFly(false),
        Dir(0.0),
        Rec,
    ));
}

fn planet_prepare(density: f32, radius: f32) -> (f32, f32, f32) {
    let volume = 4.0 / 3.0 * PI * radius.powf(3.0);
    let mass = density * volume;
    (mass, G * mass, volume)
}

fn spawn_planet(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,

    radius: f32,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    density: f32,
    speed: Vec2,
    color: Color,
    id: i32,
) {
    let mass = planet_prepare(density, radius).0;
    let planet_pre_gravity = planet_prepare(density, radius).1;
    let volume = planet_prepare(density, radius).2;

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(radius))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(pos_x, pos_y, pos_z),
        RigidBody::Dynamic,
        Collider::ball(radius),
        Velocity {
            linvel: speed, // Линейная скорость
            angvel: 0.2,   // Угловая скорость (радиан/сек)
        },
        PlanetId(id),
        PlanetVolume(volume),
        PlanetDensity(density),
        PlanetPreGravity(planet_pre_gravity),
        ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        },
        GravityScale(0.0),
        AdditionalMassProperties::Mass(mass),
        Planet,
    ));
}

fn world_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<
        (
            &mut PlanetPreGravity,
            &mut AdditionalMassProperties,
            &PlanetDensity,
            &PlanetVolume,
        ),
        With<Planet>,
    >,
) {
    let radius = 1020.0;
    let pos_x = 100.0;
    let pos_y = 100.0;
    let pos_z = 0.0;
    let density = 274_000_000.0;

    spawn_planet(
        &mut commands,
        &mut meshes,
        &mut materials,
        radius,
        pos_x,
        pos_y,
        pos_z,
        density,
        Vec2::new(0.0, 0.0),
        Color::srgba(0.086, 0.259, 0.157, 1.0),
        3,
    );

    let radius = 400.0;
    let pos_x = 5000.0;
    let pos_y = 100.0;
    let pos_z = 0.0;
    let density = 280_000_0000.0;

    spawn_planet(
        &mut commands,
        &mut meshes,
        &mut materials,
        radius,
        pos_x,
        pos_y,
        pos_z,
        density,
        Vec2::new(0.0, 200.0),
        Color::from(bevy::color::palettes::basic::GRAY),
        1,
    );
}

fn world_gravity_for_planets(
    mut planet_query: Query<
        (
            &PlanetPreGravity,
            &mut ExternalForce,
            &Transform,
            &AdditionalMassProperties,
        ),
        With<Planet>,
    >,
) {
    let planets: Vec<(Vec2, f32)> = planet_query
        .iter()
        .map(|(_, _, transform, mass)| {
            let mass = match mass {
                AdditionalMassProperties::Mass(m) => *m,
                _ => 0.0,
            };
            (transform.translation.truncate(), mass)
        })
        .collect();

    for (planet_pre_gravity_1, mut external_force_planet_1, transform_planet_1, mass_1) in
        planet_query.iter_mut()
    {
        let mut full_ext_planets_force = (0.0, 0.0);

        for (transform_planet_2, get_mass_2) in &planets {
            let dx = transform_planet_1.translation.x - transform_planet_2.x;
            let dy = transform_planet_1.translation.y - transform_planet_2.y;

            let range = (dx * dx + dy * dy).powf(0.5);

            if range < 0.0001 {
                continue;
            }

            full_ext_planets_force.0 += planet_pre_gravity_1.0 / range.powf(3.0) * dx * get_mass_2;
            full_ext_planets_force.1 += planet_pre_gravity_1.0 / range.powf(3.0) * dy * get_mass_2;
        }

        external_force_planet_1.force.x = -full_ext_planets_force.0;
        external_force_planet_1.force.y = -full_ext_planets_force.1;
    }
}
fn player_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<
        (
            &mut Velocity,
            &mut Transform,
            &mut ExternalForce,
            &AdditionalMassProperties,
            &mut Dir,
            &mut IsFly,
        ),
        (With<Rec>, Without<Planet>),
    >,
    mut camera_query: Query<
        (&mut Transform, &mut Campos, &mut OrthographicProjection),
        (With<Camera2d>, Without<Rec>),
    >,
) {
    for (mut velocity, mut transform, mut external_force, mass, mut direction, mut fly) in
        player_query.iter_mut()
    {
        let mut full_ext_forse = (0.0, 0.0);
        let mut full_velocity = (0.0, 0.0);

        //для походов

        if keys.just_pressed(KeyCode::KeyF) {
            if fly.0 == true {
                fly.0 = false
            } else {
                fly.0 = true
            }
        }

        if fly.0 == false {
            if keys.pressed(KeyCode::KeyD) {
                full_velocity.0 += (direction.0 - PI / 2.0).cos() * 3.0;
                full_velocity.1 += (direction.0 - PI / 2.0).sin() * 3.0;
            }

            if keys.pressed(KeyCode::KeyA) {
                full_velocity.0 += (direction.0 + PI / 2.0).cos() * 3.0;
                full_velocity.1 += (direction.0 + PI / 2.0).sin() * 3.0;
            }

            if keys.just_pressed(KeyCode::KeyW) {
                full_ext_forse.0 += (direction.0).cos() * 600.0;
                full_ext_forse.1 += (direction.0).sin() * 600.0;
            }
        } else {
            //для полетов

            if keys.pressed(KeyCode::KeyD) {
                external_force.torque -= 20000.0;
            }

            if keys.pressed(KeyCode::KeyA) {
                external_force.torque += 20000.0;
            }

            if keys.pressed(KeyCode::KeyW) {
                full_ext_forse.0 += direction.0.cos() * 60000.0;
                full_ext_forse.1 += direction.0.sin() * 60000.0;
            }
            if keys.pressed(KeyCode::KeyS) {
                full_ext_forse.0 -= direction.0.cos() * 60000.0;
                full_ext_forse.1 -= direction.0.sin() * 60000.0;
            }
        }
        let forward = transform.local_x();
        direction.0 = forward.y.atan2(forward.x);

        velocity.linvel.x += full_velocity.0;
        velocity.linvel.y += full_velocity.1;
        external_force.force.x += full_ext_forse.0;
        external_force.force.y += full_ext_forse.1;
    }
}

fn camera_system(
    time: Res<Time>,
    mut scroll_events: EventReader<MouseWheel>,
    mut player_query: Query<
        (
            &mut Velocity,
            &mut Transform,
            &mut ExternalForce,
            &AdditionalMassProperties,
            &mut Dir,
            &mut IsFly,
        ),
        (With<Rec>, Without<Planet>),
    >,
    mut camera_query: Query<
        (&mut Transform, &mut Campos, &mut OrthographicProjection),
        (With<Camera2d>, Without<Rec>),
    >,
) {
    for (mut transform, mut cam_pos, mut ortho) in &mut camera_query {
        for (vel, transform_p, mut external_force, mass, mut direction, fly) in &player_query {
            transform.translation = transform_p.translation;
            ortho.scale = cam_pos.0;
        }
    }

    for event in scroll_events.read() {
        for (transform, mut cam_pos, ortho) in &mut camera_query {
            cam_pos.0 += event.y * 10.0 * time.delta_secs();
        }
    }
}

fn all_player_moving(
    keys: Res<ButtonInput<KeyCode>>,

    mut planet_query: Query<
        (
            &PlanetPreGravity,
            &mut ExternalForce,
            &Transform,
            &AdditionalMassProperties,
        ),
        With<Planet>,
    >,
    mut player_query: Query<
        (
            &mut Transform,
            &mut ExternalForce,
            &mut Velocity,
            &AdditionalMassProperties,
            &mut Dir,
            &IsFly,
        ),
        (With<Rec>, Without<Planet>),
    >,
) {
    for (mut transform, mut external_force, mut velocity, mass, mut direction, mut isFly) in
        &mut player_query
    {
        reset_player_moving(&mut external_force, &mut velocity);
    }
}
fn reset_player_moving(external_force: &mut ExternalForce, velocity: &mut Velocity) {
    external_force.force = Vec2::ZERO;
    external_force.torque = 0.0;
    velocity.linvel = Vec2::ZERO;
    velocity.angvel = 0.0;
}

fn world_gravity_sistem(
    mut planet_query: Query<
        (
            &PlanetPreGravity,
            &mut ExternalForce,
            &Transform,
            &AdditionalMassProperties,
        ),
        With<Planet>,
    >,
    mut player_query: Query<
        (
            &mut Transform,
            &mut ExternalForce,
            &mut Velocity,
            &AdditionalMassProperties,
            &mut Dir,
            &IsFly,
        ),
        (With<Rec>, Without<Planet>),
    >,
) {
    for (mut transform, mut external_force, mut velocity, mass, mut direction, fly) in
        &mut player_query
    {
        let mut get_mass = 0.0;
        match *mass {
            AdditionalMassProperties::Mass(m) => get_mass = m,
            _ => get_mass = 0.0,
        }

        let mut full_ext_forse = (0.0, 0.0);
        let mut min_dx = 0.0;
        let mut min_dy = 0.0;
        let mut max_grav = 0.0;
        let mut range_m = 0.0;

        for (planet_pre_gravity, mut external_force_planet, transform_planet, massiv) in
            &mut planet_query
        {
            let dx = transform_planet.translation.x - transform.translation.x;
            let dy = transform_planet.translation.y - transform.translation.y;

            let range = (dx.powf(2.0) + dy.powf(2.0)).powf(0.5);

            let gravity_x = planet_pre_gravity.0 / range.powf(3.0) * dx * get_mass;
            let gravity_y = planet_pre_gravity.0 / range.powf(3.0) * dy * get_mass;

            if ((gravity_x.powf(2.0) + gravity_y.powf(2.0)).abs()).powf(0.5) > max_grav {
                max_grav = (gravity_x.powf(2.0) + gravity_y.powf(2.0)).powf(0.5);
                min_dx = dx;
                min_dy = dy;

                range_m = range;
            }

            full_ext_forse.0 += gravity_x;
            full_ext_forse.1 += gravity_y;

            external_force_planet.force.x = -full_ext_forse.0;
            external_force_planet.force.y = -full_ext_forse.1;
        }

        if fly.0 == false {
            external_force.force.x += full_ext_forse.0 * range_m.powf(0.6);
            external_force.force.y += full_ext_forse.1 * range_m.powf(0.6);

            let direction = Vec2::new(min_dx, min_dy).normalize();

            let angle = direction.y.atan2(direction.x);
            transform.rotation = Quat::from_rotation_z(angle + PI);

            velocity.angvel = 0.0;
        } else {
            external_force.force.x += full_ext_forse.0;
            external_force.force.y += full_ext_forse.1;
        }
    }
    world_gravity_for_planets(planet_query);
}
