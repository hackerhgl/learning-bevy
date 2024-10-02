use bevy::{
    prelude::*,
    window::WindowTheme,
};
// use bevy::window::{Window, WindowResolution, WindowPlugin};

pub const HEIGHT: f32 = 600.0;
pub const WIDTH: f32 = 800.0;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::srgb(100., 100., 100.)))
    .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am a window!".into(),
                    name: Some("bevy.app".into()),
                    resolution: (WIDTH, HEIGHT).into(),
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    visible: true,
                    ..default()
                }),
                ..default()
            }),
        ))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_basic_scene)
    .run();
}

#[derive(Component)]
struct MyCameraMarker;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MyCameraMarker,
    ));
}


pub fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {
        
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5., 5.)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)).into(),
        ..default()
    });

    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)).into(),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        }
    );

}