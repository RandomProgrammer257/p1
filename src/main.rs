use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;
use bevy_rapier2d::prelude::*;
//use rand::Rng;

static PI: f32 = std::f32::consts::PI;
static G: f32 = 6.67e-11;

#[derive(Component)]
struct Planet;

#[derive(Component)]
struct PlanetPreGravity(f32);

// #[derive(Component)]
// struct PlanetDensity(f32);

// #[derive(Component)]
// struct PlanetVolume(f32);

#[derive(Component)]
struct IsFly(bool);

#[derive(Component)]
struct Rec;

#[derive(Component)]
struct Dir(f32);

#[derive(Component)]
struct Campos(f32);

#[derive(Component)]
struct CameraMode(u32);

#[derive(Component)]
struct PlayerCollis(bool);

#[derive(Bundle)]
struct PlayerBundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
    rigid_body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    gravity_scale: GravityScale,
    mass: AdditionalMassProperties,
    friction: Friction,
    restitution: Restitution,
    external_force: ExternalForce,
    is_fly: IsFly,
    dir: Dir,
    rec: Rec,
    active_events: ActiveEvents,
    is_collis: PlayerCollis,
}

#[derive(Bundle)]
struct PlanetBundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
    rigid_body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    // planet_volume: PlanetVolume,
    // planet_density: PlanetDensity,
    planet_pre_gravity: PlanetPreGravity,
    friction: Friction,
    restitution: Restitution,
    external_force: ExternalForce,
    gravity_scale: GravityScale,
    mass: AdditionalMassProperties,
    planet: Planet,
    active_events: ActiveEvents,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Startup, world_spawn)
        .add_systems(
            Update,
            (
                reset_forces_system,
                world_gravity_for_planets_system,
                player_gravity_system,
                player_control_system,
                camera_system,
            )
                .chain(),
        )
        .add_systems(Update, collision_handler)
        .run();
}

fn collision_handler(
    mut events: EventReader<CollisionEvent>,
    mut player_query: Query<&mut PlayerCollis, With<Rec>>,
) {
    for event in events.read() {
        match event {
            CollisionEvent::Started(e1, _e2, _) => {
                if player_query.contains(*e1)
                    && let Ok(mut is_collis) = player_query.get_mut(*e1)
                {
                    is_collis.0 = true;
                }
            }
            CollisionEvent::Stopped(e1, _e2, _) => {
                if player_query.contains(*e1)
                    && let Ok(mut is_collis) = player_query.get_mut(*e1)
                {
                    is_collis.0 = false;
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let _rng = rand::thread_rng();
    commands.spawn((Camera2d, Campos(10.0), CameraMode(0)));

    let x = 1380.0;
    let y = 0.0;

    let player_bundle = PlayerBundle {
        mesh: Mesh2d(meshes.add(Rectangle::new(5.0, 20.0))),
        material: MeshMaterial2d(materials.add(Color::srgba(0.69, 0.35, 0.17, 1.0))),
        transform: Transform::from_xyz(x, y, 0.1),
        rigid_body: RigidBody::Dynamic,
        collider: Collider::cuboid(2.5, 10.0),
        velocity: Velocity::linear(Vec2::new(0.0, 0.0)),
        gravity_scale: GravityScale(0.0),
        mass: AdditionalMassProperties::Mass(5.0),
        friction: Friction::coefficient(0.6),
        restitution: Restitution::coefficient(0.0),
        external_force: ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        },
        is_fly: IsFly(false),
        dir: Dir(0.0),
        rec: Rec,
        active_events: ActiveEvents::COLLISION_EVENTS,
        is_collis: PlayerCollis(false),
    };

    commands.spawn(player_bundle);
}

fn camera_system(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut scroll_events: EventReader<MouseWheel>,
    player_query: Query<(&Velocity, &Transform, &Dir, &IsFly), With<Rec>>,
    mut camera_query: Query<
        (
            &mut Transform,
            &mut Campos,
            &mut OrthographicProjection,
            &mut CameraMode,
        ),
        (With<Camera2d>, Without<Rec>),
    >,
) {
    for (_, _, _, mut mode) in &mut camera_query {
        change_camera_mode(&keys, &mut mode);
    }

    for (mut transform, cam_pos, mut ortho, mode) in &mut camera_query {
        for (_vel, transform_p, _direction, _fly) in &player_query {
            if mode.0 == 0 {
                transform.translation = transform_p.translation;
            }
            if mode.0 == 1 {
                transform.translation = transform_p.translation;

                let angle = transform_p.rotation.to_euler(EulerRot::XYZ).2;
                transform.rotation = Quat::from_rotation_z(angle - PI / 2.0);
            }
            if mode.0 == 2 {
                transform.translation = transform_p.translation;

                let angle = transform_p.rotation.to_euler(EulerRot::XYZ).2;
                transform.rotation = Quat::from_rotation_z(angle + PI / 2.0);
            }

            ortho.scale = cam_pos.0;
        }
    }

    for event in scroll_events.read() {
        for (_, mut cam_pos, _, _) in &mut camera_query {
            cam_pos.0 += event.y * 10.0 * time.delta_secs();
        }
    }
}

fn change_camera_mode(keys: &Res<ButtonInput<KeyCode>>, mode: &mut CameraMode) {
    if keys.just_pressed(KeyCode::KeyV) {
        mode.0 = (mode.0 + 1) % 3;
    }
}

fn world_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = 128.0;
    let pos_x = 100.0;
    let pos_y = 100.0;
    let pos_z = 0.0;
    let density = 274_000_000_0.0;

    spawn_planet(
        (&mut commands, &mut meshes, &mut materials),
        radius,
        (pos_x, pos_y, pos_z),
        density,
        Vec2::new(0.0, 0.0),
        Color::srgba(0.086, 0.259, 0.157, 1.0),
    );
}

fn planet_prepare(density: f32, radius: f32) -> (f32, f32, f32) {
    let volume = 4.0 / 3.0 * PI * radius.powf(3.0);
    let mass = density * volume;
    (mass, G * mass, volume)
}

fn spawn_planet(
    ext: (
        &mut Commands,
        &mut ResMut<Assets<Mesh>>,
        &mut ResMut<Assets<ColorMaterial>>,
    ),
    radius: f32,
    pos: (f32, f32, f32),
    density: f32,
    speed: Vec2,
    color: Color,
) {
    let (mass, planet_pre_gravity, _volume) = planet_prepare(density, radius);

    let compound = vec![(Vec2::new(0.0, 0.0), 0.0_f32, Collider::ball(radius))];

    let planet_bundle = PlanetBundle {
        mesh: Mesh2d(ext.1.add(Circle::new(radius))),
        material: MeshMaterial2d(ext.2.add(color)),
        transform: Transform::from_xyz(pos.0, pos.1, pos.2),
        rigid_body: RigidBody::Dynamic,
        collider: Collider::compound(compound),
        velocity: Velocity {
            linvel: speed,
            angvel: 0.2,
        },
        //planet_volume: PlanetVolume(volume),
        //  planet_density: PlanetDensity(density),
        planet_pre_gravity: PlanetPreGravity(planet_pre_gravity),
        friction: Friction::coefficient(0.6),
        restitution: Restitution::coefficient(0.0),
        external_force: ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        },
        gravity_scale: GravityScale(0.0),
        mass: AdditionalMassProperties::Mass(mass),
        planet: Planet,
        active_events: ActiveEvents::COLLISION_EVENTS,
    };

    ext.0.spawn(planet_bundle);
}

fn reset_forces_system(
    mut player_query: Query<&mut ExternalForce, (With<Rec>, Without<Planet>)>,
    mut planet_query: Query<&mut ExternalForce, With<Planet>>,
) {
    for mut force in player_query.iter_mut() {
        force.force = Vec2::ZERO;
        force.torque = 0.0;
    }
    for mut force in planet_query.iter_mut() {
        force.force = Vec2::ZERO;
        force.torque = 0.0;
    }
}

fn world_gravity_for_planets_system(
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

    for (planet_pre_gravity_1, mut external_force_planet_1, transform_planet_1, _mass) in
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

fn player_gravity_system(
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
            &IsFly,
            &PlayerCollis,
        ),
        (With<Rec>, Without<Planet>),
    >,
) {
    for (mut transform, mut external_force, mut velocity, mass, fly, collision) in
        player_query.iter_mut()
    {
        let get_mass = match *mass {
            AdditionalMassProperties::Mass(m) => m,
            _ => 0.0,
        };

        let mut full_ext_forse = (0.0, 0.0);
        let mut min_dx = 0.0;
        let mut min_dy = 0.0;
        let mut max_grav = 0.0;
        let mut range_m = 0.0;

        for (planet_pre_gravity, mut external_force_planet, transform_planet, _massiv) in
            &mut planet_query
        {
            let dx = transform_planet.translation.x - transform.translation.x;
            let dy = transform_planet.translation.y - transform.translation.y;

            let range = (dx * dx + dy * dy).sqrt();

            if range < f32::EPSILON {
                continue;
            }

            let gravity_x = planet_pre_gravity.0 / range.powf(3.0) * dx * get_mass;
            let gravity_y = planet_pre_gravity.0 / range.powf(3.0) * dy * get_mass;

            let grav_magnitude = (gravity_x * gravity_x + gravity_y * gravity_y).sqrt();
            if grav_magnitude > max_grav {
                max_grav = grav_magnitude;
                min_dx = dx;
                min_dy = dy;
                range_m = range;
            }

            full_ext_forse.0 += gravity_x;
            full_ext_forse.1 += gravity_y;

            external_force_planet.force.x = -full_ext_forse.0;
            external_force_planet.force.y = -full_ext_forse.1;
        }

        if !fly.0 {
            if !collision.0 {
                external_force.force.x += full_ext_forse.0;
                external_force.force.y += full_ext_forse.1;
            }
            if range_m > f32::EPSILON {
                let direction = Vec2::new(min_dx, min_dy).normalize();
                let angle = direction.y.atan2(direction.x);
                transform.rotation = Quat::from_rotation_z(angle + PI);
            }
            velocity.angvel = 0.0;
        } else {
            if !collision.0 {
                external_force.force.x += full_ext_forse.0;
                external_force.force.y += full_ext_forse.1;
            }
        }
    }
}

fn player_control_system(
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
        let mut full_velocity = (0.0, 0.0);

        if keys.just_pressed(KeyCode::KeyF) {
            fly.0 = !fly.0;
        }

        if !fly.0 {
            if is_collis.0 {
                if keys.pressed(KeyCode::KeyD) {
                    full_velocity.0 += (direction.0 - PI / 2.0).cos() * 1.0;
                    full_velocity.1 += (direction.0 - PI / 2.0).sin() * 1.0;
                }

                if keys.pressed(KeyCode::KeyA) {
                    full_velocity.0 += (direction.0 + PI / 2.0).cos() * 1.0;
                    full_velocity.1 += (direction.0 + PI / 2.0).sin() * 1.0;
                }

                if keys.just_pressed(KeyCode::KeyW) {
                    full_ext_forse.0 = direction.0.cos() * 600000.0;
                    full_ext_forse.1 = direction.0.sin() * 600000.0;
                }
            }
        } else {
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
