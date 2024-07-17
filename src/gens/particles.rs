use bevy::{
    app::{App, Plugin, PostStartup, Startup},
    asset::Assets,
    color::Color,
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    ecs::system::EntityCommands,
    hierarchy::{BuildChildren, Parent},
    math::{Vec2, Vec3, Vec4},
    prelude::{default, Camera3dBundle, Commands, Entity, Query, ResMut},
    render::camera::Camera,
    transform::components::Transform,
};
use bevy_hanabi::{
    Attribute, ColorOverLifetimeModifier, EffectAsset, ExprWriter, Gradient, HanabiPlugin,
    LinearDragModifier, OrientMode, OrientModifier, ParticleEffect, ParticleEffectBundle,
    SetAttributeModifier, SetPositionCircleModifier, SetVelocityTangentModifier, ShapeDimension,
    SizeOverLifetimeModifier, Spawner, TangentAccelModifier,
};

use crate::resources::WheelParticles;

pub fn setup_particles(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut parent: Query<&Parent>,
) -> Entity {
    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0));
    color_gradient1.add_key(0.1, Vec4::new(4.0, 4.0, 0.0, 1.0));
    color_gradient1.add_key(0.9, Vec4::new(4.0, 0.0, 0.0, 1.0));
    color_gradient1.add_key(1.0, Vec4::new(4.0, 0.0, 0.0, 0.0));

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
        axis: writer.lit(Vec3::Y).expr(),
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
    let rotation = writer.lit(-1.59).uniform(writer.lit(1.59)).expr();

    let mut module = writer.finish();

    let tangent_accel = TangentAccelModifier::constant(&mut module, Vec3::ZERO, Vec3::Z, 30.);

    let effect1 = effects.add(
        EffectAsset::new(vec![16384, 16384], Spawner::rate(5000.0.into()), module)
            .with_name("portal")
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
            transform: Transform::from_xyz(0., 1.2, 0.),
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
    }
}
