//! Rolls a player-controlled wheel

use std::f32::consts::PI;

use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};

// theoretically we could exceed the limit outside of the player speed (going down hill)
const TURN_SPEED: f32 = 0.001;
const MAX_TURN_SPEED: f32 = 0.03;
const FORWARD_SPEED: f32 = 0.001;
const MAX_SPEED: f32 = 0.05;
const MAX_CAM_DISTANCE: f32 = 7.;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
        })
        .insert_resource(DirectionalLightShadowMap { size: 8192 })
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
            // first_cascade_far_bound: 200.0,
            // maximum_distance: 400.0,
            maximum_distance: 400.0,
            first_cascade_far_bound: 0.9,
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
        // PbrBundle {
        //     mesh: asset_server.load("models/Wheel.glb#Mesh0"),
        //     ..default()
        // },
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
struct MotionState {
    speed_x: f32,
    speed_y: f32,
    speed_z: f32,
}

#[derive(Resource, Default)]
struct Game {
    camera: MotionState,
    player_wheel: MotionState,
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

/// because the wheel spins and turns, get just y for the turn
fn wheel_y_rotation(rotation: &Quat) -> Quat {
    let mut rotation_y = rotation.normalize();
    rotation_y.z = 0.;
    rotation_y.x = 0.;
    rotation_y
}

fn move_wheel(
    mut q: Query<&mut Transform, With<PlayerCharacter>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
) {
    let mut t = q.single_mut();
    let speed = game.player_wheel.speed_z;

    // since we are also spinning the wheel, for the math to work we only want Y, as the wheel pivots around Y
    let rotation = wheel_y_rotation(&t.rotation);
    let direction = Direction3d::new(rotation * -Vec3::X).unwrap();
    t.translation += direction * (speed * 100.) * time.delta_seconds();

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
    mut game: ResMut<Game>,
    mut q_char: Query<(&PlayerCharacter, &mut Transform)>,
    mut q_cam: Query<(&PlayerCamera, &mut Transform), Without<PlayerCharacter>>,
) {
    let (char, mut t_char) = q_char.single_mut();
    let (cam, mut t_cam) = q_cam.single_mut();

    let distance_x = t_char.translation.x - t_cam.translation.x;
    let distance_z = t_char.translation.z - t_cam.translation.z;
    let camera_should_move_x = distance_x > MAX_CAM_DISTANCE || distance_x < -MAX_CAM_DISTANCE;
    let camera_should_move_z = distance_z > MAX_CAM_DISTANCE || distance_z < -MAX_CAM_DISTANCE;
    let m_x = if distance_x > 0. { 1. } else { -1. };
    let m_z = if distance_z > 0. { 1. } else { -1. };

    println!("Distance X {:?}", distance_x);
    println!("Move camera? {:?}", camera_should_move_x);

    if camera_should_move_x {
        game.camera.speed_x += (FORWARD_SPEED * 2.) * m_x;
    }

    if camera_should_move_z {
        game.camera.speed_z += FORWARD_SPEED * m_z;
    };

    // BEGIN char & cam translation lock
    // If not wanted it is still useful for debugging
    // let mut trans_char = t_char.translation + (t_char.right() * -5. * time.delta_seconds());
    // // trans_char.z -= 10.;

    // let mut trans_form = t_char.with_translation(trans_char);
    // trans_form.translation.y = 3.;
    // trans_form.translation.z + 10.;

    // // t_cam.translation = trans_char;
    // // t_cam.translation.y = 3.;
    t_cam.translation.x = t_char.translation.x;
    t_cam.translation.z = t_char.translation.z + 10.;
    // let otter = t_cam.translation.angle_between(t_char.translation);
    // let rotation = wheel_y_rotation(&t_char.rotation);
    // // t_cam.rotate_around(t_char.translation, rotation);
    // // t_cam.rotation.y -= otter;
    // t_cam.translate_around(trans_form.translation, rotation);
    // t_cam.rotate_around(t_char.translation, rotation);
    // t_cam.rotation.y = rotation.y;
    // t_cam.translation.x -= 10.;
    // END

    // t_cam.translation.x += game.camera.speed_x;
    // t_cam.translation.z += game.camera.speed_z;

    if game.camera.speed_x != 0. {
        let dir_m = (m_x);
        game.camera.speed_x += FORWARD_SPEED * (game.camera.speed_x / MAX_SPEED) * 0.5 * dir_m;
    }

    if !(game.camera.speed_x > 0.0001 || game.camera.speed_x < -0.0001) {
        game.camera.speed_x = 0.;
    }

    if game.camera.speed_z != 0. {
        let dir_m = (m_z);
        game.camera.speed_z += FORWARD_SPEED * (game.camera.speed_z / MAX_SPEED) * 0.5 * dir_m;
    }

    if !(game.camera.speed_z > 0.0001 || game.camera.speed_z < -0.0001) {
        game.camera.speed_z = 0.;
    }

    let t_cam_face_char = t_cam.looking_at(
        Vec3::new(t_char.translation.x, 1.0, t_char.translation.z),
        Vec3::Y,
    );
    let dir = Direction3d::new(t_cam_face_char.rotation * -Vec3::X).unwrap();
    // t_cam.rotation.y = t_cam_face_char.rotation.y;
    // let cam_rot = t_cam.rotation.normalize();
    let rot_diff = t_cam.rotation.y - t_cam_face_char.rotation.y;
    let cam_spin_m = if rot_diff > 0.005 {
        -1.
    } else if rot_diff < -0.005 {
        1.
    } else {
        0.
    };

    println!("rot diff {:?}", rot_diff);

    if cam_spin_m != 0. {
        // t_cam.rotate_y(0.001 * cam_spin_m);
        t_cam.rotate_y(0.01 * cam_spin_m);

        // t_cam.translation += dir * FORWARD_SPEED * time.delta_seconds();
    }

    // let player_translation = t_char.translation.xz();
    // let to_player = (player_translation - t_cam.translation.xz()).normalize();

    // get the quaternion to rotate from the initial enemy facing direction to the direction
    // facing the player
    // let rotate_to_player = Quat::from_rotation_arc(Vec3::Z, to_player.extend(0.));

    // rotate the enemy to face the player
    // t_cam.rotation = rotate_to_player;

    // t_cam.rotate_y(t_cam_face_char.rotation.y);
    // t_cam.len
    // println!("I'm a character Y: {:?}", t_char.1.local_y());
    // println!("I'm a camera Y: {:?}", t_cam.1.local_y());
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
