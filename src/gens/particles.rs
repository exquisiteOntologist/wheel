use bevy::{
    app::{App, Plugin, PostStartup, Startup, Update},
    asset::Assets,
    color::Color,
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    ecs::system::EntityCommands,
    hierarchy::{BuildChildren, Parent},
    math::{Quat, Vec2, Vec3, Vec4},
    prelude::{default, Camera3dBundle, Commands, Entity, EntityRef, Query, Res, ResMut, With},
    reflect::Reflect,
    render::camera::Camera,
    time::Time,
    transform::components::Transform,
};
use bevy_hanabi::{
    Attribute, ColorOverLifetimeModifier, EffectAsset, EffectProperties, ExprWriter, Gradient,
    HanabiPlugin, LinearDragModifier, OrientMode, OrientModifier, ParticleEffect,
    ParticleEffectBundle, SetAttributeModifier, SetPositionCircleModifier,
    SetVelocityTangentModifier, ShapeDimension, SizeOverLifetimeModifier, Spawner,
    TangentAccelModifier,
};
use bevy_rapier3d::na::Rotation3;

use crate::{
    components::wheel::WheelState,
    resources::{Game, PlayerWheel, WheelParticles},
    utils::matrix::{quaternion_from_rpy_quat, roll_pitch_yaw_from_quat},
};

pub fn move_particles(
    // this may have to be global transform
    mut q_p: Query<&mut Transform, With<WheelParticles>>,
    // mut q_w: Query<&mut Transform, With<PlayerWheel>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
    // to find the direction
    mut wheel: ResMut<WheelState>,
) {
    // let mut wheels = q_w.iter_mut();

    for mut particles in q_p.iter_mut() {
        // let wheel_rotation = wheels.next().unwrap().rotation;

        particles.rotate_x(0.);
        particles.rotate_y(0.);
        particles.rotate_z(0.);
        //
        // let mut t = particles.clone();

        // let (roll, pitch, yaw) = roll_pitch_yaw_from_quat(t.rotation.conjugate());

        // let updated_rot_quat = quaternion_from_rpy_quat(0., 0., 0.);
        // t.rotation = t.rotation.normalize();
        // t.rotate(updated_rot_quat);

        // let updated_rot_quat = quaternion_from_rpy_quat(roll, pitch, yaw);
        // t.rotation = t.rotation.normalize();
        // t.rotate(updated_rot_quat);

        // let updated_rot_quat = quaternion_from_rpy_quat(0., 0., wheel.rpy.yaw);
        // t.rotation = t.rotation.normalize();
        // t.rotate(updated_rot_quat);

        // particles.rotation = t.rotation.normalize();

        println!("particles rot {}", particles.rotation);
    }
}

/// A simple marker component to identify the effect using a dynamic
/// property-based acceleration that the `update_accel()` system will control at
/// runtime.
// #[derive(Component)]
// struct DynamicRuntimeAccel;

// fn update_particles(mut query: Query<&mut EffectProperties, With<DynamicRuntimeAccel>>) {
//     let mut properties = query.single_mut();
//     let accel0 = 10.;
//     let (s, c) = (time.elapsed_seconds() * 0.3).sin_cos();
//     let accel = Vec3::new(c * accel0, s * accel0, 0.);
//     properties.set("my_accel", accel.into());
// }

/// Create a colour for a colour stop.
/// The divider value lets you adjust.
/// The lower the divider the brighter and smoother the particles will be.
/// Higher dividers may produce more accurate colours.
fn clr(a: f32, b: f32, c: f32, d: f32, divider: f32) -> Vec4 {
    Vec4::new(a / divider, b / divider, c / divider, d)
}

pub fn setup_particles(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut parent: Query<&Parent>,
) -> Entity {
    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(
        0.0,
        // Vec4::new(255.0 / 100., 255.0 / 100., 227.0 / 100., 0.9),
        clr(255., 255., 227., 0.9, 100.),
    );
    color_gradient1.add_key(
        0.1,
        // Vec4::new(255.0 / 100., 255.0 / 100., 227.0 / 100., 0.5),
        clr(255., 255., 227., 0.5, 100.),
    );
    color_gradient1.add_key(
        0.9,
        // Vec4::new(255.0 / 100., 255.0 / 100., 227.0 / 100., 0.3),
        clr(255., 255., 227., 0.3, 100.),
    );
    color_gradient1.add_key(
        1.0,
        // Vec4::new(255.0 / 100., 255.0 / 100., 227.0 / 100., 0.0),
        clr(255., 255., 227., 0.0, 100.),
    );

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.3, Vec2::new(0.2, 0.02));
    size_gradient1.add_key(1.0, Vec2::splat(0.0));

    let writer = ExprWriter::new();

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(1.3).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocityTangentModifier {
        origin: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        // speed: writer.lit(1.6).uniform(writer.lit(3.)).expr(),
        speed: writer.lit(-0.5).uniform(writer.lit(3.)).expr(),
    };

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime = writer.lit(0.6).uniform(writer.lit(1.3)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Add drag to make particles slow down a bit after the initial acceleration
    let drag = writer.lit(2.).expr();
    let update_drag = LinearDragModifier::new(drag);

    // rotation
    let rotation = writer.lit(-0.).uniform(writer.lit(1.59)).expr();

    let mut module = writer.finish();

    let tangent_accel = TangentAccelModifier::constant(&mut module, Vec3::ZERO, Vec3::Z, 30.);

    let effect1 = effects.add(
        EffectAsset::new(vec![16384, 16384], Spawner::rate(5000.0.into()), module)
            .with_name("particles_portal")
            .init(init_pos)
            .init(init_age)
            .init(init_lifetime)
            .init(init_vel)
            .update(update_drag)
            .update(tangent_accel)
            .render(ColorOverLifetimeModifier {
                gradient: color_gradient1,
            })
            .render(SizeOverLifetimeModifier {
                gradient: size_gradient1,
                screen_space_size: false,
            })
            .render(OrientModifier::new(OrientMode::AlongVelocity).with_rotation(rotation)),
    );

    let particles = commands.spawn((
        // Name::new("portal"),
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect1),
            transform: Transform::from_xyz(0., 1.2, 0.).with_rotation(Quat::from_rotation_z(1.)),
            ..Default::default()
        },
        WheelParticles,
    ));

    particles.id()
}

fn setup(commands: Commands, effects: ResMut<Assets<EffectAsset>>, parent: Query<&Parent>) {
    setup_particles(commands, effects, parent);
}

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin);
        // setup from parent instead
        app.add_systems(Startup, setup);
        app.add_systems(Update, (move_particles /*update_particles*/,));
    }
}
