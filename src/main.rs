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
        .add_systems(Update, planet_prepare)
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
            Mesh2d(meshes.add(Circle::new(5.0))),
            MeshMaterial2d(materials.add(Color::srgba(0.086, 0.259, 0.157, 1.0))),
            Transform::from_xyz(x, y, 0.1),
            RigidBody::Dynamic,
            Collider::ball(5.0),
            Velocity::linear(Vec2::new(0.0, 0.0)),
            GravityScale(0.0),
            AdditionalMassProperties::Mass(5000.0),
            ExternalForce {
                force: Vec2::new(0.0, 0.0), // Сила в Ньютонах (вправо)
                torque: 0.0,
            },
            Dir(0.0),
            Rec,
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh2d(meshes.add(Circle::new(5.0))),
                MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::basic::BLACK))),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
            parent.spawn((
                Mesh2d(meshes.add(Circle::new(1.0))),
                MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::basic::BLACK))),
                Transform::from_xyz(1.0, 0.0, 0.2),
            ));
/*
            parent.spawn((
                Mesh2d(meshes.add(Rectangle::new(20.0, 140.0))),
                MeshMaterial2d(materials.add(Color::from(Color::srgba(0.086, 0.259, 0.157, 1.0)))),
                Transform::from_xyz(-45.0, 0.0, 0.1),
            ));
            parent.spawn((
                Mesh2d(meshes.add(Rectangle::new(30.0, 150.0))),
                MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::basic::BLACK))),
                Collider::cuboid(15.0, 75.0),
                Transform::from_xyz(-45.0, 0.0, 0.0),
            ));
 */       });
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
    let density = 274_000_000.0;

    for i in 0..1 {
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(radius))),
            MeshMaterial2d(materials.add(Color::srgba(0.086, 0.259, 0.157, 1.0))),
            Transform::from_xyz(pos_x, pos_y, pos_z),
            RigidBody::Dynamic,
            Collider::ball(radius),
            Velocity::linear(Vec2::new(0.0, 0.0)),
            PlanetId(i),
            PlanetVolume(4.0 / 3.0 * PI * radius.powf(3.0)),
            PlanetDensity(density),
            PlanetPreGravity(0.0),
            ExternalForce {
                force: Vec2::new(0.0, 0.0), 
                torque: 0.0,
            },
            GravityScale(0.0),
            AdditionalMassProperties::Mass(0.0),
            Planet,
        ));
    }
}

fn player_system(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Velocity, &mut Transform, &mut Dir), With<Rec>>,
    mut camera_query: Query<
        (&mut Transform, &mut Campos, &mut OrthographicProjection),
        (With<Camera2d>, Without<Rec>),
    >,
) {
    for (mut velocity, transform, mut dir) in &mut player_query {
        if keys.pressed(KeyCode::KeyD) {
            velocity.angvel -= time.delta_secs() * 200.0 * PI / 180.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            velocity.angvel += time.delta_secs() * 200.0 * PI / 180.0;
        }

        if keys.pressed(KeyCode::KeyW) {
            velocity.linvel.x += time.delta_secs() * dir.0.cos() * 200.0;
            velocity.linvel.y += time.delta_secs() * dir.0.sin() * 200.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            velocity.linvel.x -= time.delta_secs() * dir.0.cos() * 200.0;
            velocity.linvel.y -= time.delta_secs() * dir.0.sin() * 200.0;
        }

        let forward = transform.local_x();
        dir.0 = forward.y.atan2(forward.x);
    }
    for mut transform in &mut camera_query {
        for transform_p in &player_query {
            transform.0.translation = transform_p.1.translation;
            transform.2.scale = transform.1.0;
        }
    }
}

fn planet_prepare(
    mut planet_query: Query<
        (
            &mut PlanetPreGravity,
            &mut AdditionalMassProperties,
            &PlanetDensity,
            &PlanetVolume,
        ),
        With<Planet>,
    >,
) {
    for (mut pre_gravity, mut mass, density, volume) in &mut planet_query {
        *mass = AdditionalMassProperties::Mass(density.0 * volume.0);
        let mut get_mass = 0.0;
        match *mass {
            AdditionalMassProperties::Mass(m) => get_mass = m,
            _ => get_mass = 0.0,
        }

        pre_gravity.0 = G * get_mass;
           
    }

}

fn world_gravity_sistem(
    mut planet_query: Query<(&PlanetPreGravity, &mut ExternalForce, &Transform), With<Planet>>,
    mut player_query: Query<(&Transform, &mut ExternalForce), (With<Rec>, Without<Planet>)>,
) {
    for (transform, mut external_force) in &mut player_query {
        let mut full_ext_forse = (0.0, 0.0);
        for (planet_pre_gravity, mut external_force_planet, transform_planet) in &mut planet_query {
            let range = ((transform_planet.translation.x - transform.translation.x).powf(2.0)
                + (transform_planet.translation.y - transform.translation.y).powf(2.0))
            .powf(0.5);

            full_ext_forse.0 += planet_pre_gravity.0 / range.powf(3.0)
                * (transform_planet.translation.x - transform.translation.x) * 50000.0;
            full_ext_forse.1 += planet_pre_gravity.0 / range.powf(3.0)
                * (transform_planet.translation.y - transform.translation.y)* 50000.0;

            external_force_planet.force.x = -full_ext_forse.0;
            external_force_planet.force.y = -full_ext_forse.1;

            external_force.force.x = full_ext_forse.0;
            external_force.force.y = full_ext_forse.1;
            
            
        println!("{}   {}",range,( external_force.force.x.powf(2.0) + external_force.force.y.powf(2.0)).powf(0.5));

        }
    }
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
