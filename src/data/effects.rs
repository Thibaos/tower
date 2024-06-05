use bevy::prelude::{Vec2, Vec3, Vec4};
use bevy_hanabi::{
    Attribute, BinaryOperator, BuiltInOperator, ColorOverLifetimeModifier, EffectAsset, Gradient,
    LinearDragModifier, Module, OrientMode, OrientModifier, ScalarType, SetAttributeModifier,
    SetPositionCircleModifier, SetVelocityCircleModifier, ShapeDimension, SizeOverLifetimeModifier,
    Spawner, ValueType,
};

pub fn new_effect_asset() -> EffectAsset {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 0.8, 0.8, 1.0));
    gradient.add_key(0.5, Vec4::new(1.0, 0.5, 0.5, 1.0));
    gradient.add_key(1.0, Vec4::new(1.0, 0.1, 0.1, 0.0));

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0., Vec2::splat(0.05));
    size_gradient.add_key(1., Vec2::splat(0.0));

    // Create a new expression module
    let mut module = Module::default();

    let init_pos = SetPositionCircleModifier {
        center: module.lit(-Vec3::Y),
        axis: module.lit(Vec3::Y),
        radius: module.lit(0.1),
        dimension: ShapeDimension::Surface,
    };

    let init_speed_sample =
        module.builtin(BuiltInOperator::Rand(ValueType::Scalar(ScalarType::Float)));
    let init_speed_min = module.lit(10.0);
    let init_speed_range = module.lit(10.0);
    let init_speed_2 = module.binary(BinaryOperator::Mul, init_speed_sample, init_speed_range);

    let init_velocity = SetVelocityCircleModifier {
        center: module.lit(-Vec3::Y),
        axis: module.lit(Vec3::Y),
        speed: module.binary(BinaryOperator::Add, init_speed_2, init_speed_min),
    };

    let update_drag = LinearDragModifier::new(module.lit(4.0));

    let lifetime = module.lit(1.5);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let age = module.builtin(BuiltInOperator::Rand(ValueType::Scalar(ScalarType::Float)));
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    EffectAsset::new(vec![1024], Spawner::once(256.0.into(), true), module)
        .with_name("dash_effect")
        .init(init_age)
        .init(init_pos)
        .init(init_velocity)
        .init(init_lifetime)
        .update(update_drag)
        .render(ColorOverLifetimeModifier { gradient })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        })
        .render(OrientModifier::new(OrientMode::FaceCameraPosition))
}
