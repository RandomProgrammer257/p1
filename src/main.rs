use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

static PI: f32 = std::f32::consts::PI;

#[derive(Component)]
struct Rec;

#[derive(Component)]
struct Dir(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, setup)
        .add_systems(Update, player_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    // Камера (новый синтаксис 0.15)
    commands.spawn(Camera2d);

    for _ in 0..1 {
        let x = rng.gen_range(-400.0..400.0);
        let y = rng.gen_range(-300.0..300.0);

        commands
            .spawn((
                Mesh2d(meshes.add(Circle::new(40.0))),
                MeshMaterial2d(materials.add(Color::srgba(0.086, 0.259, 0.157, 1.0))),
                Transform::from_xyz(x, y, 0.1),
                RigidBody::Dynamic,
                Collider::ball(50.0),
                Velocity::linear(Vec2::new(0.0, 0.0)),
                GravityScale(0.0),
                Dir(0.0),
                Rec,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Mesh2d(meshes.add(Circle::new(50.0))),
                    MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::basic::BLACK))),
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
                parent.spawn((
                    Mesh2d(meshes.add(Circle::new(5.0))),
                    MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::basic::BLACK))),
                    Transform::from_xyz(35.0, 0.0, 0.2),
                ));

                parent.spawn((
                    Mesh2d(meshes.add(Rectangle::new(10.0, 70.0))),
                    MeshMaterial2d(
                        materials.add(Color::from(Color::srgba(0.086, 0.259, 0.157, 1.0))),
                    ),
                    Transform::from_xyz(-45.0, 0.0, 0.1),
                ));
                parent.spawn((
                    Mesh2d(meshes.add(Rectangle::new(15.0, 75.0))),
                    MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::basic::BLACK))),
                    Transform::from_xyz(-45.0, 0.0, 0.0),
                ));
            });
    }
}
fn player_system(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Velocity, &mut Transform, &mut Dir), With<Rec>>,
) {
    for (mut velocity, mut transform, mut dir) in &mut player_query {
        if keys.pressed(KeyCode::KeyD) {
            velocity.angvel -= time.elapsed_secs() * 0.3 * PI / 180.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            velocity.angvel += time.elapsed_secs() * 0.3 * PI / 180.0;
        }
        if keys.pressed(KeyCode::KeyW) {
            velocity.linvel.x += time.elapsed_secs() * 0.3 * dir.0.cos();
            velocity.linvel.y += time.elapsed_secs() * 0.3 * dir.0.sin();
        }
        if keys.pressed(KeyCode::KeyS) {
            velocity.linvel.x -= time.elapsed_secs() * dir.0.cos() * 0.3;
            velocity.linvel.y -= time.elapsed_secs() * dir.0.sin() * 0.3;
        }
        end_line(&mut transform, &mut velocity);

        let forward = transform.local_x();
        dir.0 = forward.y.atan2(forward.x);
    }
}

fn end_line(transform: &mut Transform, velocity: &mut Velocity) {
    if transform.translation.x > 400.0 {
        transform.translation.x = 400.0;
        velocity.linvel.x *= -0.2;
        velocity.angvel *= 0.8;
    }
    if transform.translation.x < -400.0 {
        transform.translation.x = -400.0;
        velocity.linvel.x *= -0.2;
        velocity.angvel *= 0.8;
    }
    if transform.translation.y > 400.0 {
        transform.translation.y = 400.0;
        velocity.linvel.y *= -0.2;
        velocity.angvel *= 0.8;
    }
    if transform.translation.y < -400.0 {
        transform.translation.y = -400.0;
        velocity.linvel.y *= -0.2;
        velocity.angvel *= 0.8;
    }
}
