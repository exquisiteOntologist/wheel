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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 3.0, 0.0)
            .looking_at(Vec3::new(0.0, 1.0, -0.0), Vec3::Y),
        ..default()
    });

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
        Wheel,
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
struct Wheel;

#[derive(Default)]
struct WheelState {
    speed_y: f32,
    speed_z: f32,
}

#[derive(Resource, Default)]
struct Game {
    player_wheel: WheelState,
}

fn spin_wheel(mut q: Query<&mut Transform, With<Wheel>>, time: Res<Time>, game: ResMut<Game>) {
    // note that the wheel is
    for mut t in &mut q {
        t.rotate_local_z(game.player_wheel.speed_z);

        // turning
        if game.player_wheel.speed_y != 0.0 {
            if game.player_wheel.speed_y < 0.0 {
                // t.rotate_local_x(-0.1);
                // t.rotate_x(-0.1);
                // rotation.x = -0.3;
            } else if game.player_wheel.speed_y > 0.0 {
                // t.rotate_local_x(0.1);
                // t.rotate_x(0.1);
                // rotation.x = 0.3;
            }
            t.rotate_y(game.player_wheel.speed_y);
        }
        let mut rotation = t.rotation;
        // println!("Y rotation {:?}", rotation.y);
        // t.transfor
        // rotation.x = 2.0;
        // t.rotate(rotation);
        // t.rotatea
        // t.with_rotation(rotation);
    }
}

fn move_wheel(mut q: Query<&mut Transform, With<Wheel>>, time: Res<Time>, mut game: ResMut<Game>) {
    for mut t in &mut q {
        // https://allenchou.net/2019/08/trigonometry-basics-sine-cosine/
        // https://stackoverflow.com/questions/46697502/how-to-move-a-sprite-according-to-an-angle-in-pygame
        // https://bevyengine.org/examples/Transforms/transform/

        // let d = t.forward();
        // t.di
        // t.translation += direction * game.player_wheel.speed_z;
        // t.translation += direction * game.player_wheel.speed_z * time.delta_seconds();
        // t.translation += t.directiono * game.player_wheel.speed_z * time.delta_seconds();
        // t.translation.mul_add(, b)
        // t.translation.x += game.player_wheel.speed_z;
        // t.translation += t.translation.x * t.translation.z * game.player_wheel.speed_z;
        //
        // t.translation = t.transform_point(Vec3::new(0.0, t.rotation.y, 0.5));
        // rot.x
        // let (_, angle) = t.rotation.to_axis_angle();
        let angle = t.rotation.y;
        // let angle_rad = Rad(angle); // probably passing in degrees here, not rads
        let speed = game.player_wheel.speed_z;

        println!("speed {:?}", speed);
        println!("angle {:?}", angle);
        // println!("rad   {:?}", angle_rad.0);
        // Rad::cos(angle_rad);

        let new_x = t.translation.x + (speed * angle.cos());
        let new_z = t.translation.z + (speed * angle.sin());
        let new_y = t.translation.y;

        // let d = t.forward();
        // d.
        // t.looking_to(direction, up)
        // t.translation = Vec3::new(new_x, new_y, new_z);

        // let new_t = t
        //     .with_translation(Vec3::new(speed, 0.0, 0.0))
        //     // .with_scale(Vec3::splat(0.01))
        //     .with_rotation(base_rotation * Quat::from_rotation_y(-fox_angle));

        // new_t.forward()
        //
        let forward = t.local_x();
        let mut rotation = t.rotation.normalize();
        rotation.z = 0.;
        rotation.x = 0.;
        // rotation.y *= -1.;
        // rotation.
        let jon_x = Direction3d::new(rotation * -Vec3::X).unwrap();
        // let alt_t = Transform::from_xyz(t.translation.x, t.translation.y, 0.);
        // let forward = t.forward();
        // t.local
        // t.translation.forw
        t.translation += jon_x * (speed * 100.) * time.delta_seconds();
        // t.translation += forward * (speed * 100.) * time.delta_seconds();

        // NOTE: The reason it rotates weird is because we are spinning z
        //
        // > > > The solution is to have a seperate mesh object for the wheel to the position object.
        // > > > The mesh will need to be parented to the position object.
        //
        // ! ! !

        // let dir = Direction3d::from_xyz(t.translation.x, t.translation.y, 1.).unwrap();
        // Direction3d::
        // t.translation += dir * (speed * 10.) * time.delta_seconds();

        // 3.0.cos
        //
        // t.translation.x += speed;

        //
        // t.rotation.angle
        // let move_to = Vec3::new(
        //     t.local_x(),
        //     t.local_y(). + game.player_wheel.speed_z,
        //     t.local_z(),
        // );
        // t.translation = move_to;
    }

    // Slow down speed
    if game.player_wheel.speed_z > 0.0 {
        game.player_wheel.speed_z -= FORWARD_SPEED * (game.player_wheel.speed_z / MAX_SPEED) * 0.5;
    } else if game.player_wheel.speed_z < 0.0 {
        game.player_wheel.speed_z += FORWARD_SPEED * (game.player_wheel.speed_z / -MAX_SPEED) * 0.5;
    }

    // Slow down turn
    if game.player_wheel.speed_y > 0.0 {
        game.player_wheel.speed_y -= TURN_SPEED * (game.player_wheel.speed_y / MAX_TURN_SPEED);
    } else if game.player_wheel.speed_y < 0.0 {
        game.player_wheel.speed_y += TURN_SPEED * (game.player_wheel.speed_y / -MAX_TURN_SPEED);
    }
}

fn move_camera(time: Res<Time>, game: ResMut<Game>) {
    // for mut t in &mut q {
    //     //
    // }
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
            game.player_wheel.speed_y += TURN_SPEED;
        }
    } else if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        if game.player_wheel.speed_y > -MAX_TURN_SPEED {
            game.player_wheel.speed_y -= TURN_SPEED;
        }
    }
}
