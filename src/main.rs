use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

static PI: f32 = std::f32::consts::PI;
static G: f32 = 6.67 * 0.000_000_000_01;

#[derive(Component)]
struct Planet;

#[derive(Component)]
struct PlanetId(u32);

#[derive(Component)]
struct PlanetPreGravity(f32);

#[derive(Component)]
struct PlanetDensity(f32);

#[derive(Component)]
struct PlanetVolume(f32);

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
        .add_systems(Update, player_system)
        .add_systems(Update, mouse_scroll)
        .add_systems(Update, world_gravity_sistem)
        .run();
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    commands.spawn((Camera2d, Campos(10.0)));

    let x = -500.0;
    let y = 0.0;

    commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(20.0, 5.0))),
            MeshMaterial2d(materials.add(Color::srgba(0.69, 0.35, 0.17, 1.0))),
            Transform::from_xyz(x, y, 0.1),
            RigidBody::Dynamic,
            Collider::cuboid(10.0, 2.5),
            Velocity::linear(Vec2::new(0.0, 0.0)),
            GravityScale(0.0),
            AdditionalMassProperties::Mass(5000.0),
            ExternalForce {
                force: Vec2::new(0.0, 0.0), // Сила в Ньютонах (вправо)
                torque: 0.0,
            },
            Dir(0.0),
            Rec,
        ));
}

fn planet_prepare(density: f32, radius: f32) -> (f32,f32,f32) {
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
    id: u32,
) {
    let mass = planet_prepare(density,radius).0;
    let planet_pre_gravity = planet_prepare(density,radius).1;
    let volume = planet_prepare(density, radius).2;

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(radius))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(pos_x, pos_y, pos_z),
        RigidBody::Dynamic,
        Collider::ball(radius),
        Velocity::linear(speed),
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
    mut query: Query<
        (
            &mut PlanetPreGravity,
            &mut AdditionalMassProperties,
            &PlanetDensity,
            &PlanetVolume,
        ),
        With<Planet>,
    >,
) {
    let radius = 1280.0;
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
        Vec2::new(0.0, 200.0),
        Color::from(bevy::color::palettes::basic::GRAY),
        1,
    );
}

fn player_system(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<
        (&mut Velocity, &mut Transform, &mut Dir, &mut ExternalForce),
        With<Rec>,
    >,
    mut camera_query: Query<
        (&mut Transform, &mut Campos, &mut OrthographicProjection),
        (With<Camera2d>, Without<Rec>),
    >,
) {
    for mut transform in &mut camera_query {
        for transform_p in &player_query {
            transform.0.translation = transform_p.1.translation;
            transform.2.scale = transform.1.0;
        }
    }
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

        let get_mass_1 = match mass_1 {
            AdditionalMassProperties::Mass(m) => *m,
            _ => 0.0,
        };

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

fn world_gravity_sistem(
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
            &Transform,
            &mut ExternalForce,
            &AdditionalMassProperties,
            &mut Dir,
        ),
        (With<Rec>, Without<Planet>),
    >,
) {
    for (transform, mut external_force, mass, mut direction) in &mut player_query {
        let mut get_mass = 0.0;
        match *mass {
            AdditionalMassProperties::Mass(m) => get_mass = m,
            _ => get_mass = 0.0,
        }

        let mut full_ext_forse = (0.0, 0.0);

        for (planet_pre_gravity, mut external_force_planet, transform_planet, massiv) in
            &mut planet_query
        {
            let range = ((transform_planet.translation.x - transform.translation.x).powf(2.0)
                + (transform_planet.translation.y - transform.translation.y).powf(2.0))
            .powf(0.5);

            full_ext_forse.0 += planet_pre_gravity.0 / range.powf(3.0)
                * (transform_planet.translation.x - transform.translation.x)
                * get_mass;
            full_ext_forse.1 += planet_pre_gravity.0 / range.powf(3.0)
                * (transform_planet.translation.y - transform.translation.y)
                * get_mass;

            external_force.torque = 0.0;

            if keys.pressed(KeyCode::KeyD) {
                external_force.torque -= 200000.0;
            }

            if keys.pressed(KeyCode::KeyA) {
                external_force.torque += 200000.0;
            }

            if keys.pressed(KeyCode::KeyW) {
                full_ext_forse.0 += direction.0.cos() * 600000.0;
                full_ext_forse.1 += direction.0.sin() * 600000.0;
            }
            if keys.pressed(KeyCode::KeyS) {
                full_ext_forse.0 -= direction.0.cos() * 600000.0;
                full_ext_forse.1 -= direction.0.sin() * 600000.0;
            }

            let forward = transform.local_x();
            direction.0 = forward.y.atan2(forward.x);

            external_force_planet.force.x = -full_ext_forse.0;
            external_force_planet.force.y = -full_ext_forse.1;

            external_force.force.x = full_ext_forse.0;
            external_force.force.y = full_ext_forse.1;
        }
    }
    world_gravity_for_planets(planet_query);
}

fn mouse_scroll(
    time: Res<Time>,
    mut scroll_events: EventReader<MouseWheel>,
    mut camera_query: Query<&mut Campos, (With<Camera2d>, Without<Rec>)>,
) {
    for event in scroll_events.read() {
        for mut transform in &mut camera_query {
            transform.0 += event.y * 10.0 * time.delta_secs();
        }
    }
}
