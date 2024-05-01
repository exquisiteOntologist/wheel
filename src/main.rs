//! Rolls a player-controlled wheel

use std::f32::consts::PI;

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use cgmath::{Angle, Rad};

// theoretically we could exceed the limit outside of the player speed (going down hill)
const TURN_SPEED: f32 = 0.001;
const MAX_TURN_SPEED: f32 = 0.03;
const FORWARD_SPEED: f32 = 0.001;
const MAX_SPEED: f32 = 0.05;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
        })
        .init_resource::<Game>()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Wheel".into(),
                name: Some("Wheel.app".into()),
                // resolution: (500., 300.).into(),
                ..default()
            }),
            ..default()
        }),))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                setup_scene_once_loaded,
                spin_wheel,
                move_wheel,
                move_camera,
                keyboard_animation_control,
            ),
        )
        .run();
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
) {
    // Insert a resource with the current scene information
    commands.insert_resource(Animations(vec![
        // asset_server.load("models/animated/Fox.glb#Animation2"),
        // asset_server.load("models/animated/Fox.glb#Animation1"),
        // asset_server.load("models/animated/Fox.glb#Animation0"),
        // asset_server.load("models/Wheel.glb#x"),
    ]));

    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 3.0, 0.0)
                .looking_at(Vec3::new(0.0, 1.0, -0.0), Vec3::Y),
            ..default()
        },
        PlayerCamera,
    ));

    // Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(500000.0, 500000.0)),
        material: materials.add(Color::hex("#887A63").unwrap().as_rgba()),
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 0.0, -PI / 3.5)),
        directional_light: DirectionalLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // Wheel
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/Wheel.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 1.2, 0.0),
            ..default()
        },
        PlayerCharacter,
    ));

    game.player_wheel.speed_z = MAX_SPEED;

    println!("Controls:");
    println!("  - arrow up / down: roll");
    println!("  - arrow left / right: turn direction");
}

// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[0].clone_weak()).repeat();
    }
}

#[derive(Component)]
struct PlayerCamera;

#[derive(Component)]
struct PlayerCharacter;

#[derive(Default)]
struct WheelState {
    speed_y: f32,
    speed_z: f32,
}

#[derive(Resource, Default)]
struct Game {
    player_wheel: WheelState,
}

fn spin_wheel(
    mut q: Query<&mut Transform, With<PlayerCharacter>>,
    time: Res<Time>,
    game: ResMut<Game>,
) {
    for mut t in &mut q {
        // spinning the wheel
        t.rotate_local_z(game.player_wheel.speed_z);

        // turning
        if game.player_wheel.speed_y != 0.0 {
            if game.player_wheel.speed_y < 0.0 {
                // t.rotate_local_x(-0.1);
                // t.rotate_x(-0.1);
                // rotation.x = -0.3;
                //
                // TILT
            } else if game.player_wheel.speed_y > 0.0 {
                // t.rotate_local_x(0.1);
                // t.rotate_x(0.1);
                // rotation.x = 0.3;
                //
                // TILT
            }
            t.rotate_y(game.player_wheel.speed_y);
        }
    }
}

fn move_wheel(
    mut q: Query<&mut Transform, With<PlayerCharacter>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
) {
    for mut t in &mut q {
        let angle = t.rotation.y;
        let speed = game.player_wheel.speed_z;

        println!("speed {:?}", speed);
        println!("angle {:?}", angle);

        // since we are also spinning the wheel, for the math to work we only want Y, as the wheel pivots around Y
        let mut rotation = t.rotation.normalize();
        rotation.z = 0.;
        rotation.x = 0.;
        let direction = Direction3d::new(rotation * -Vec3::X).unwrap();
        t.translation += direction * (speed * 100.) * time.delta_seconds();
    }

    // Slow down speed
    if game.player_wheel.speed_z > 0.0 {
        game.player_wheel.speed_z -= FORWARD_SPEED * (game.player_wheel.speed_z / MAX_SPEED) * 0.5;
    } else if game.player_wheel.speed_z < 0.0 {
        game.player_wheel.speed_z += FORWARD_SPEED * (game.player_wheel.speed_z / -MAX_SPEED) * 0.5;
    }

    if !(game.player_wheel.speed_z > 0.0001 || game.player_wheel.speed_z < -0.0001) {
        game.player_wheel.speed_z = 0.;
    }

    // Slow down turn
    if game.player_wheel.speed_y > 0.0 {
        game.player_wheel.speed_y -= TURN_SPEED * (game.player_wheel.speed_y / MAX_TURN_SPEED);
    } else if game.player_wheel.speed_y < 0.0 {
        game.player_wheel.speed_y += TURN_SPEED * (game.player_wheel.speed_y / -MAX_TURN_SPEED);
    }

    if !(game.player_wheel.speed_y > 0.0001 || game.player_wheel.speed_y < -0.0001) {
        game.player_wheel.speed_y = 0.;
    }
}

fn move_camera(
    time: Res<Time>,
    game: ResMut<Game>,
    // mut qChar: Query<&mut Transform, With<PlayerCharacter>>,
    // mut qCam: Query<&mut Transform, With<PlayerCamera>>,
    mut set: ParamSet<(
        Query<&mut Transform, With<PlayerCharacter>>,
        Query<&mut Transform, With<PlayerCamera>>,
    )>,
) {
    for mut t in set.p0().iter_mut() {
        println!("I'm a character");
    }

    for mut t in set.p1().iter_mut() {
        println!("I'm a camera");
    }
}

fn keyboard_animation_control(keyboard_input: Res<ButtonInput<KeyCode>>, mut game: ResMut<Game>) {
    // println!("Wheel Y speed {:?}", game.player_wheel.speed_y);

    if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        if game.player_wheel.speed_z < MAX_SPEED {
            game.player_wheel.speed_z += FORWARD_SPEED;
        }
    } else if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        if game.player_wheel.speed_z > -MAX_SPEED {
            game.player_wheel.speed_z -= FORWARD_SPEED;
        }
    }

    if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        if game.player_wheel.speed_y < MAX_TURN_SPEED {
            if game.player_wheel.speed_z >= 0. {
                game.player_wheel.speed_y += TURN_SPEED;
            } else {
                game.player_wheel.speed_y -= TURN_SPEED;
            }
        }
    } else if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        if game.player_wheel.speed_y > -MAX_TURN_SPEED {
            if game.player_wheel.speed_z >= 0. {
                game.player_wheel.speed_y -= TURN_SPEED;
            } else {
                game.player_wheel.speed_y += TURN_SPEED;
            }
        }
    }
}
