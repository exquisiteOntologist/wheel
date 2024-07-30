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
    transform::{bundles::TransformBundle, components::Transform},
};
use bevy_hanabi::{
    Attribute, ColorOverLifetimeModifier, EffectAsset, EffectProperties, ExprWriter, Gradient,
    HanabiPlugin, LinearDragModifier, OrientMode, OrientModifier, ParticleEffect,
    ParticleEffectBundle, SetAttributeModifier, SetPositionCircleModifier,
    SetPositionCone3dModifier, SetPositionSphereModifier, SetVelocityTangentModifier,
    ShapeDimension, SizeOverLifetimeModifier, Spawner, TangentAccelModifier,
};
use bevy_rapier3d::na::Rotation3;

use crate::{
    components::wheel::WheelState,
    resources::{Game, PlayerWheel, WheelParticles},
    utils::{
        angles::degrees_to_radians,
        roll_pitch_yaw::{quaternion_from_rpy_quat, roll_pitch_yaw_from_quat},
    },
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

        // particles.rotate_x(0.);
        // particles.rotate_y(0.);
        // particles.rotate_z(0.);
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

        // println!("particles rot {}", particles.rotation);
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
/// May be similar to Attribute::COLOR
fn clr(a: f32, b: f32, c: f32, d: f32, divider: f32) -> Vec4 {
    Vec4::new(a / divider, b / divider, c / divider, d)
}

pub const MAX_SAND_RATE: f32 = 5000.;

pub fn setup_particles(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut parent: Query<&Parent>,
) -> Entity {
    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(
        0.0,
        // Vec4::new(255.0 / 100., 255.0 / 100., 227.0 / 100., 0.9),
        clr(238., 218., 187., 0.9, 100.),
        // clr(0., 0., 0., 0.9, 100.),
    );
    color_gradient1.add_key(
        0.1,
        // Vec4::new(255.0 / 100., 255.0 / 100., 227.0 / 100., 0.5),
        clr(246., 229., 202., 0.5, 100.),
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
    size_gradient1.add_key(0.3, Vec2::new(0.08, 0.01));
    size_gradient1.add_key(1.0, Vec2::splat(0.0));

    let writer = ExprWriter::new();

    let pos_center = writer.add_property("pos_center", Vec3::ZERO.into());
    let pos_axis = writer.add_property("pos_axis", Vec3::X.into());

    // NOTE this is a 2D circle so it tends to try to face a specific direction
    // let init_pos = SetPositionCircleModifier {
    //     center: writer.prop(pos_center).expr(),
    //     axis: writer.prop(pos_axis).expr(),
    //     radius: writer.lit(1.3).expr(),
    //     dimension: ShapeDimension::Surface,
    // };

    // let init_pos = SetPositionSphereModifier {
    //     center: writer.lit(Vec3::ZERO).expr(),
    //     // axis: writer.lit(Vec3::Z).expr(),
    //     radius: writer.lit(1.3).expr(),
    //     dimension: ShapeDimension::Surface,
    // };

    // cone at back of wheel
    let init_pos = SetPositionCone3dModifier {
        dimension: ShapeDimension::Surface,
        height: writer.lit(2.).expr(),
        base_radius: writer.lit(1.3).expr(),
        top_radius: writer.lit(0.2).expr(),
    };

    // // a cone acting as a circle. emits inside wheel.
    // let init_pos = SetPositionCone3dModifier {
    //     dimension: ShapeDimension::Surface,
    //     height: writer.lit(0.1).expr(),
    //     base_radius: writer.lit(1.25).expr(),
    //     top_radius: writer.lit(1.3).expr(),
    // };

    // Note that if the velocity doesn't follow the character,
    // it can be more like wind, with the particles seemingly
    // going in the direction of a wind.
    let init_vel = SetVelocityTangentModifier {
        origin: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        // axis: writer.prop(pos_axis).expr(),
        // speed: writer.lit(1.6).uniform(writer.lit(3.)).expr(),
        speed: writer.lit(-0.5).uniform(writer.lit(3.)).expr(),
    };

    //
    //
    //
    //
    //
    //

    // NEED TO TRY CHANGING THE ACCESS
    //
    //
    // After, similary this will need to be done for velocity etc.

    //

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

    // opacity
    let opacity = writer.lit(0.2).uniform(writer.lit(0.9)).expr();
    let init_opacity = SetAttributeModifier::new(Attribute::ALPHA, opacity);

    // spawner
    let spawner = Spawner::rate(MAX_SAND_RATE.into()).with_starts_active(false);

    // acceleration - the axis affects the direction the particles go
    // let tangent_accel = TangentAccelModifier::constant(&mut module, Vec3::ZERO, Vec3::Y, 30.);
    let tangent_accel = TangentAccelModifier::new(
        writer.lit(Vec3::ZERO).expr(),
        writer.lit(Vec3::Y).expr(),
        writer.lit(10.).uniform(writer.lit(30.)).expr(),
    );
    //
    // let tangent_accel = TangentAccelModifier::new(
    //     writer.lit(Vec3::ZERO).expr(),
    //     // writer.lit(Vec3::Y).expr(),
    //     writer.prop(pos_axis).expr(),
    //     writer.lit(10.).uniform(writer.lit(30.)).expr(),
    // );

    let mut module = writer.finish();

    let effect1 = effects.add(
        EffectAsset::new(vec![16384, 16384], spawner, module)
            .with_name("particles_portal")
            .init(init_pos)
            .init(init_age)
            .init(init_lifetime)
            .init(init_vel)
            .init(init_opacity)
            .update(update_drag)
            // the acceleration makes the particles flow in a direction other than behind
            .update(tangent_accel)
            .render(ColorOverLifetimeModifier {
                gradient: color_gradient1,
            })
            .render(SizeOverLifetimeModifier {
                gradient: size_gradient1,
                screen_space_size: false,
            })
            // .render(OrientModifier::new(OrientMode::AlongVelocity).with_rotation(rotation)),
            .render(OrientModifier::new(OrientMode::AlongVelocity)),
    );

    let particles = commands.spawn((
        // Name::new("portal"),
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect1),
            transform: Transform::from_xyz(-2.7, 0., 0.)
                // transform: Transform::from_xyz(0., 1.1, 0.)
                // .with_rotation(Quat::from_rotation_x(degrees_to_radians(-90.))),
                .with_rotation(Quat::from_rotation_z(degrees_to_radians(-120.))),
            // .with_rotation(Quat::from_rotation_z(degrees_to_radians(-0.))),
            ..Default::default()
        },
        // TransformBundle::default(),
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
        app.add_systems(Startup, setup);
        app.add_systems(Update, (move_particles /*update_particles*/,));
    }
}
