mod parser;
mod flashcard;

use std::ops::{Add, Mul, Neg};
use glam::{Mat4, Quat, Vec3, vec3, Vec3Swizzles};
use prisma::{Rgb, Rgba};
use stereokit::material::{DEFAULT_ID_MATERIAL, Material};
use stereokit::mesh::Mesh;
use stereokit::model::Model;
use stereokit::render::RenderLayer;
use stereokit::{high_level, mesh, pose, Settings, StereoKit, ui};
use stereokit::high_level::math_traits::{MatrixTrait, PosTrait, RotationTrait, ScaleTrait};

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on", logger(level = "debug", tag = "my-tag")))]
pub fn main() {
    println!("Hello World");
    my_func().unwrap();
}
use anyhow::{Context, Result};
use mint::{Quaternion, Vector3};
use stereokit::font::Font;
use stereokit::high_level::collider::{Collider, ColliderType};
use stereokit::high_level::quat_from_angles;
use stereokit::input::Handed;
use stereokit::input::Handed::Right;
use stereokit::lifecycle::DrawContext;
use stereokit::pose::Pose;
use stereokit::text::{TextAlign, TextFit, TextStyle};
use stereokit::values::{Color128, SKMatrix};
use crate::flashcard::{Card, FlashCard, IntoRand};
use crate::parser::TEST_FILE;


pub fn my_func() -> Result<()> {
    let WHITE: Rgba<f32> = Rgba::new(Rgb::new(1_f32, 1_f32, 1_f32), 1_f32).into();
    println!("Hello, world!");
    let flash_cards = parser::parse(TEST_FILE);
    let rand_cards: Option<(Card, Card, Card)> = flash_cards.into_rand();
    println!("random_cards: {:?}", rand_cards);
    let mut stereokit = Settings::default().init().unwrap();
    let cube_material = Material::copy_from_id(&stereokit, DEFAULT_ID_MATERIAL).context("error")?;

    let mut flash_card = FlashCard::new(&stereokit, &cube_material, rand_cards.unwrap().0.definition.0.as_str(), 0.05)?;

    let mut test_cube = high_level::model::Model::from_mesh(&stereokit, &Mesh::gen_cube(&stereokit, [0.5, 0.5, 0.5], 1).context("err")?, &cube_material)?;
    test_cube.set_collider(&stereokit, ColliderType::CapsuleCollider);
    test_cube.rotate(30.0, 30.0, 30.0);
    flash_card.set_pos(0.0, 0.0, -1.5);
    flash_card.set_rotation(0.0, 180.0, 0.0);
    let mut val = 0;
    let mut val2 = 0;
    stereokit.run(|sk, ctx| {
        flash_card.draw(ctx);
        flash_card.translate(0.0, 0.0, 0.005);
        test_cube.draw(ctx);

        if flash_card.model.collider_intersects(sk, test_cube.get_collider().unwrap()) {
            val += 1;
            val2 += 1;
            println!("intersecting: {}", val2);
            val = 0;
        }

    }, |_| {});
    Ok(())
}
