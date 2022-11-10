mod parser;
mod flashcard;

use std::ops::Add;
use glam::{Mat4, Quat, Vec3, vec3};
use prisma::{Rgb, Rgba};
use stereokit::material::{DEFAULT_ID_MATERIAL, Material};
use stereokit::mesh::Mesh;
use stereokit::model::Model;
use stereokit::render::RenderLayer;
use stereokit::Settings;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on", logger(level = "debug", tag = "my-tag")))]
pub fn main() {
    println!("Hello World");
    my_func().unwrap();
}
use anyhow::{Context, Result};
use stereokit::input::Handed;
use crate::flashcard::{Card, IntoRand};
use crate::parser::TEST_FILE;

pub fn my_func() -> Result<()> {
    println!("Hello, world!");
    let flash_cards = parser::parse(TEST_FILE);
    let rand_cards: Option<(Card, Card, Card)> = flash_cards.into_rand();
    println!("random_cards: {:?}", rand_cards);
    let stereokit = Settings::default().init().unwrap();
    let cube_mesh = Mesh::gen_cube(&stereokit, Vec3::new(1.0, 1.0, 1.0).into(), 1).context("err")?;
    let cube_material = Material::copy_from_id(&stereokit, DEFAULT_ID_MATERIAL).context("error")?;
    let cube_model = Model::from_mesh(&stereokit, &cube_mesh, &cube_material).context("error")?;


    let saber_model = Model::from_mesh(&stereokit, &cube_mesh, &cube_material).context("error")?;
    stereokit.run(|sk, drw_ctx| {
        let right_hand = sk.input_hand(Handed::Right);
        saber_model.draw(
            drw_ctx,
            Mat4::from_scale_rotation_translation(
                vec3(0.01, 2.0, 0.01),
                right_hand.palm.orientation.into(),
                Vec3::from(right_hand.palm.position).add(Vec3::new(0.0, 0.9, 0.0)),
            ).into(),
            Rgba::new(Rgb::new(1_f32, 1_f32, 1_f32), 1_f32).into(),
            RenderLayer::Layer0);
        cube_model.draw(
            drw_ctx,
            Mat4::from_scale_rotation_translation(
                vec3(0.01, 0.1, 0.1),
                Quat::IDENTITY,
                vec3(0., 0., 0.),
            ).into(),
            Rgba::new(Rgb::new(1_f32, 1_f32, 1_f32), 1_f32).into(),
            RenderLayer::Layer0);

    }, |_| {});
    Ok(())
}
