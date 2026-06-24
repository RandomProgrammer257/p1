use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_square)
        .run();
}

#[derive(Component)]
struct Square;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
        MeshMaterial2d(materials.add(Color::hsl(200.0, 0.8, 0.6))),
        Transform::from_xyz(-200.0, 0.0, 0.0),
        Square,
    ));
}

fn move_square(
    time: Res<Time>,
    mut square: Single<&mut Transform, With<Square>>,
) {
    square.translation.x += 200.0 * time.delta_secs();
    
    if square.translation.x > 400.0 {
        square.translation.x = -400.0;
    }
}