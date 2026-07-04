use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::text::TextLayoutInfo;
use bevy_rapier2d::prelude::CollisionEvent;
use bevy_rapier2d::prelude::*;

//use rand::Rng;

mod player_systems;
use player_systems::{player_control_system, player_gravity_system};

mod info_showing;
use info_showing::{planet_frame_handler, planet_info_handler, planet_get_info_handler};

pub const MORESIZE: f32 = 10.0;

pub static PI: f32 = std::f32::consts::PI;
pub static G: f32 = 6.67e-11;

//------Planets-------//

#[derive(Component)]
pub struct Planet;

#[derive(Component)]
pub struct PlanetPreGravity(pub f32);

#[derive(Component)]
pub struct PlanetRadius(pub f32);

#[derive(Component)]
pub struct PlanetExtraGravZone(pub f32);

#[derive(Component)]
pub struct PlanetDensity(pub f32);

#[derive(Component)]
pub struct PlanetVolume(pub f32);
//------Player-------//

#[derive(Component)]
pub struct IsFly(pub bool);

#[derive(Component)]
pub struct PlayerCollis(pub bool);

#[derive(Component)]
pub struct Rec;

#[derive(Component)]
pub struct Dir(pub f32);

//------Cameras-------//

#[derive(Component)]
pub struct Campos(pub f32);

#[derive(Component)]
pub struct CameraMode(pub u32);

#[derive(Component)]
pub struct CameraWorldAngle(pub f32);

//------Stars-------//

#[derive(Component)]
pub struct Star;

//------Information----//

#[derive(Component)]
pub struct PlanetInfo;

#[derive(Component)]
pub struct PlanetInfoText;

#[derive(Component)]
pub struct PlanetInfoZone;

#[derive(Component)]
pub struct FrameComponent;

static STARRADIUS: f32 = 1280.0 * MORESIZE;
static STARDANSITY: f32 = 3.8e10;

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
    radius: PlanetRadius,
    gravity_scale: GravityScale,
    planet_volume: PlanetVolume,
    planet_density: PlanetDensity,
    planet_pre_gravity: PlanetPreGravity,
    friction: Friction,
    restitution: Restitution,
    external_force: ExternalForce,
    mass: AdditionalMassProperties,
    zone: PlanetExtraGravZone,
    planet: Planet,
    active_events: ActiveEvents,
}

#[derive(Bundle)]
struct TheMainStarBundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
    rigid_body: RigidBody,
    collider: Collider,
    radius: PlanetRadius,
    mass: AdditionalMassProperties,
    star_pre_gravity: PlanetPreGravity,
    main_star: Star,
}

fn main() {
    App::new()
        .insert_resource(TimestepMode::Interpolated {
            dt: 1.0 / 60.0,
            time_scale: 1.0,
            substeps: 1,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1600.0, 1200.0),
                title: "Orbital fox".to_string(),
                position: WindowPosition::At(IVec2::new(0, 0)),
                present_mode: bevy::window::PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default().with_length_unit(1.0 * MORESIZE))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Startup, world_spawn)
        .add_systems(
            Update,
            (
                reset_forces_system,
                collision_handler,
                player_control_system,
            )
                .chain(),
        )
        .add_systems(Update, (planet_frame_handler, planet_info_handler, planet_get_info_handler).chain())
        .add_systems(
            Update,
            (
                player_gravity_system,
                world_gravity_for_planets_system,
                star_gravity_system,
            )
                .after(reset_forces_system),
        )
        .add_systems(
            PostUpdate,
            camera_system.after(TransformSystem::TransformPropagate),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let _rng = rand::thread_rng();

    let x = 25100.0 * MORESIZE + STARRADIUS;
    let y = 0.0;

    let player_bundle = PlayerBundle {
        mesh: Mesh2d(meshes.add(Rectangle::new(1.2 * MORESIZE, 0.4 * MORESIZE))),
        material: MeshMaterial2d(materials.add(Color::srgba(0.69, 0.35, 0.17, 1.0))),
        transform: Transform::from_xyz(x, y, 0.1),
        rigid_body: RigidBody::Dynamic,
        collider: Collider::cuboid(0.6 * MORESIZE, 0.2 * MORESIZE),
        velocity: Velocity::linear(Vec2::new(0.0, 1150.3 * MORESIZE)),
        gravity_scale: GravityScale(0.0),
        mass: AdditionalMassProperties::Mass(5.0 * MORESIZE * MORESIZE * MORESIZE),
        friction: Friction::coefficient(0.5),
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

    let x = 0.0;
    let y = 0.0;

    let mass = 4.0 / 3.0 * PI * STARRADIUS.powf(3.0) * STARDANSITY;
    let pre_gravity = G * mass;

    let main_star = TheMainStarBundle {
        mesh: Mesh2d(meshes.add(Circle::new(STARRADIUS))),
        material: MeshMaterial2d(materials.add(Color::srgba(0.69, 0.35, 0.17, 1.0))),
        transform: Transform::from_xyz(x, y, 0.1),
        rigid_body: RigidBody::Fixed,
        collider: Collider::ball(STARRADIUS),
        radius: PlanetRadius(STARRADIUS),
        mass: AdditionalMassProperties::Mass(mass),
        star_pre_gravity: PlanetPreGravity(pre_gravity),
        main_star: Star,
    };
    commands.spawn(main_star);
    commands.spawn(player_bundle).with_children(|parent| {
        parent.spawn((
            Camera2d,
            Campos(0.3),
            CameraMode(0),
            CameraWorldAngle(0.0),
            Transform::from_xyz(0.0, 0.0, 0.1),
        ));
    });
}

fn star_gravity_system(
    mut planet_query: Query<
        (&mut ExternalForce, &AdditionalMassProperties, &Transform),
        With<Planet>,
    >,
    mut player_query: Query<
        (&mut ExternalForce, &AdditionalMassProperties, &Transform),
        (With<Rec>, Without<Planet>, Without<Star>),
    >,
    mut star_query: Query<(&PlanetPreGravity, &Transform)>,
) {
    for (star_pre_gravity, star_transform) in &mut star_query {
        for (mut planet_external_forse, planet_mass, planet_transform) in &mut planet_query {
            let mass = match planet_mass {
                AdditionalMassProperties::Mass(m) => *m,
                _ => 0.0,
            };

            let dx = star_transform.translation.x - planet_transform.translation.x;
            let dy = star_transform.translation.y - planet_transform.translation.y;
            let range = (dx * dx + dy * dy).sqrt();

            if range < 1e-10 {
                continue;
            }

            let force_magnitude = star_pre_gravity.0 * mass / (range * range);
            let force_x = force_magnitude * (dx / range);
            let force_y = force_magnitude * (dy / range);

            planet_external_forse.force.x += force_x;
            planet_external_forse.force.y += force_y;
        }
        for (mut player_external_force, player_mass, player_transform) in &mut player_query {
            let mass = match player_mass {
                AdditionalMassProperties::Mass(m) => *m,
                _ => 0.0,
            };

            let dx = star_transform.translation.x - player_transform.translation.x;
            let dy = star_transform.translation.y - player_transform.translation.y;
            let range = (dx * dx + dy * dy).sqrt();

            if range < 1e-10 {
                continue;
            }

            let force_magnitude = star_pre_gravity.0 * mass / (range * range);
            let force_x = force_magnitude * (dx / range);
            let force_y = force_magnitude * (dy / range);

            player_external_force.force.x += force_x;
            player_external_force.force.y += force_y;
        }
    }
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

fn camera_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut scroll_events: EventReader<MouseWheel>,
    player_query: Query<(&Velocity, &Transform, &Dir, &IsFly), With<Rec>>,
    mut camera_query: Query<
        (
            &mut Transform,
            &mut Campos,
            &mut OrthographicProjection,
            &mut CameraMode,
            &mut CameraWorldAngle,
        ),
        (With<Camera2d>, Without<Rec>),
    >,
) {
    for (_, _, _, mut mode, _) in &mut camera_query {
        change_camera_mode(&keys, &mut mode);
    }

    for (mut transform, cam_pos, mut ortho, mode, mut world_angle) in &mut camera_query {
        for (_vel, transform_p, _direction, _fly) in &player_query {
            let player_angle = transform_p.rotation.to_euler(EulerRot::XYZ).2;

            if mode.0 == 0 {
                transform.rotation = transform
                    .rotation
                    .slerp(Quat::from_rotation_z(-player_angle), 1.0);
            }
            if mode.0 == 1 {
                transform.rotation = transform.rotation.slerp(Quat::from_rotation_z(0.0), 1.0);
            }
            if mode.0 == 2 {
                transform.rotation = transform.rotation.slerp(Quat::from_rotation_z(PI), 1.0);
            }

            let camera_angle = transform.rotation.to_euler(EulerRot::XYZ).2;

            world_angle.0 = camera_angle + player_angle;
            ortho.scale = cam_pos.0;
        }
    }

    for event in scroll_events.read() {
        for (_, mut cam_pos, _, _, _) in &mut camera_query {
            if cam_pos.0 + event.y * 1.5 > 0.0 {
                cam_pos.0 += event.y * 1.5;
            }
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
    let radius = 32.0 * MORESIZE;
    let pos_x = 24500.0 * MORESIZE + STARRADIUS;
    let pos_y = 0.0;
    let pos_z = 0.0;
    let density = 3.8e8;
    let zone = 10.0;

    spawn_planet(
        (&mut commands, &mut meshes, &mut materials),
        radius,
        (pos_x, pos_y, pos_z),
        density,
        Vec2::new(0.0, 1022.3 * MORESIZE),
        Color::srgba(0.086, 0.259, 0.157, 1.0),
        zone,
    );

    let radius = 8.0 * MORESIZE;
    let pos_x = 25100.0 * MORESIZE + STARRADIUS;
    let pos_y = 000.0;
    let pos_z = 0.0;
    let density = 3.8e8;
    let zone = 5.0;
    for i in 0..1 {
        spawn_planet(
            (&mut commands, &mut meshes, &mut materials),
            radius,
            (pos_x + 10.0 * (i as f32), pos_y, pos_z),
            density,
            Vec2::new(0.0, 1150.0 * MORESIZE),
            Color::srgba(0.5, 0.5, 0.5, 1.0),
            zone,
        );
    }
}

fn planet_prepare(density: f32, radius: f32) -> (f32, f32, f32) {
    let volume = 4.0 / 3.0 * PI * (radius).powf(3.0);
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
    zoner: f32,
) {
    let (mass, planet_pre_gravity, volume) = planet_prepare(density, radius);
    let compound = vec![(Vec2::new(0.0, 0.0), 0.0_f32, Collider::ball(radius))];

    let planet_bundle = PlanetBundle {
        mesh: Mesh2d(ext.1.add(Circle::new(radius))),
        material: MeshMaterial2d(ext.2.add(color)),
        transform: Transform::from_xyz(pos.0, pos.1, pos.2),
        rigid_body: RigidBody::Dynamic,
        collider: Collider::compound(compound),
        velocity: Velocity {
            linvel: speed,
            angvel: 0.01,
        },
        radius: PlanetRadius(radius),
        planet_volume: PlanetVolume(volume),
        planet_density: PlanetDensity(density),
        planet_pre_gravity: PlanetPreGravity(planet_pre_gravity),
        friction: Friction::coefficient(0.5),
        restitution: Restitution::coefficient(0.0),
        external_force: ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        },
        gravity_scale: GravityScale(0.0),
        mass: AdditionalMassProperties::Mass(mass),
        zone: PlanetExtraGravZone(zoner),
        planet: Planet,
        active_events: ActiveEvents::COLLISION_EVENTS,
    };

    ext.0.spawn(planet_bundle).with_children(|parent| {
        parent.spawn((
            Mesh2d(ext.1.add(Circle::new(zoner * radius))),
            MeshMaterial2d(ext.2.add(Color::srgba(0.45, 0.56, 0.59, 0.1))),
            Transform::from_xyz(0.0, 0.0, pos.2 - 4.0),
        ));
        parent
            .spawn((
                Transform {
                    translation: Vec3::new(0.0, 0.0, 5.0),
                    rotation: Quat::from_rotation_z(PI / 2.0),
                    scale: Vec3::ONE,
                },
                PlanetInfo,
            ))
            .with_children(|grandparent| {
                grandparent.spawn((
                    Text2d::new(format!(
                        "{}",
                        radius / 10.0
                    )),
                    TextFont {
                        font: default(),
                        font_size: 30.0,
                        font_smoothing: default(),
                    },
                    TextColor(Color::WHITE),
                    Transform {
                        translation: Vec3::new(-radius, radius, 0.1),
                        rotation: Quat::from_rotation_z(PI / 2.0),
                        scale: Vec3::ONE,
                    },
                    PlanetInfoText,
                ));
            });

        parent
            .spawn((
                Transform {
                    translation: Vec3::new(0.0, 0.0, 0.1),
                    rotation: Quat::from_rotation_z(PI / 2.0),
                    scale: Vec3::ONE,
                },
                PlanetInfoZone,
            ))
            .with_children(|grandparent| {
                for i in 0..4{
                        grandparent.spawn((
                    Mesh2d(ext.1.add(Rectangle::new(2.2 * radius, 1.5))),
                    MeshMaterial2d(ext.2.add(Color::srgba(0.0, 0.9, 1.0, 1.0))),
                    Transform {
                        translation: Vec3::new(radius * 1.1 * ((i as f32) * PI/2.0).cos(), radius * 1.1 * ((i as f32) * PI/2.0).sin(), 5.1),
                        rotation: Quat::from_rotation_z((i as f32) * PI/2.0 + PI/2.0),
                        scale: Vec3::ONE,
                    },
                    FrameComponent, 
                ));
                }
            });
    });
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
            &PlanetRadius,
            &PlanetExtraGravZone,
        ),
        With<Planet>,
    >,
) {
    let planets: Vec<(Vec2, f32, f32, f32)> = planet_query
        .iter()
        .map(|(_pre_gravity, _, transform, mass, radius, zone)| {
            let mass = match mass {
                AdditionalMassProperties::Mass(m) => *m,
                _ => 0.0,
            };
            (transform.translation.truncate(), mass, radius.0, zone.0)
        })
        .collect();

    for (
        planet_pre_gravity_1,
        mut external_force_planet_1,
        transform_planet_1,
        _mass,
        radius_1,
        zone_1,
    ) in planet_query.iter_mut()
    {
        let mut full_ext_planets_force = (0.0, 0.0);

        for (transform_planet_2, get_mass_2, radius_2, zone_2) in &planets {
            let dx = transform_planet_1.translation.x - transform_planet_2.x;
            let dy = transform_planet_1.translation.y - transform_planet_2.y;

            let range = (dx * dx + dy * dy).powf(0.5);

            if range < 0.0001 {
                continue;
            }
            let gravity_x: f32;
            let gravity_y: f32;

            if range > *radius_2 * *zone_2 + zone_1.0 * radius_1.0 {
                gravity_x = planet_pre_gravity_1.0 * get_mass_2 / (range / zone_2).powf(2.0)
                    * (dx / range)
                    * 32.0;
                gravity_y = planet_pre_gravity_1.0 * get_mass_2 / (range / zone_2).powf(2.0)
                    * (dy / range)
                    * 32.0;
            } else {
                gravity_x = -9.8 * MORESIZE * dx * get_mass_2 / range.powf(2.0);
                gravity_y = -9.8 * MORESIZE * dy * get_mass_2 / range.powf(2.0);
            }

            full_ext_planets_force.0 += gravity_x;
            full_ext_planets_force.1 += gravity_y;
        }

        external_force_planet_1.force.x -= full_ext_planets_force.0;
        external_force_planet_1.force.y -= full_ext_planets_force.1;
    }
}
