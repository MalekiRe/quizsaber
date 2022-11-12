mod parser;
mod flashcard;

use std::ops::{Add, Mul, Neg};
use glam::{Mat4, Quat, Vec2, Vec3, vec3, Vec3Swizzles};
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
use rand::prelude::SliceRandom;
use rand::thread_rng;
use stereokit::font::Font;
use stereokit::high_level::collider::{Collider, ColliderType};
use stereokit::high_level::quat_from_angles;
use stereokit::input::Handed;
use stereokit::input::Handed::{Left, Right};
use stereokit::lifecycle::DrawContext;
use stereokit::pose::Pose;
use stereokit::shader::Shader;
use stereokit::text::{TextAlign, TextFit, TextStyle};
use stereokit::values::{Color128, SKMatrix};
use crate::flashcard::{Card, FlashCard, IntoRand};
use crate::parser::TEST_FILE;



pub fn my_func() -> Result<()> {
    println!("Hello, world!");
    let right_saber = include_bytes!("../resources/saber1.glb");
    let left_saber = include_bytes!("../resources/saber2.glb");
    let flash_cards = parser::parse(TEST_FILE);
    let rand_cards: Vec<Card> = flash_cards.choose_num(4);

    // println!("random_cards: {:?}", rand_cards);
    let mut sk = Settings::default().init().unwrap();
    let cube_material = Material::copy_from_id(&sk, DEFAULT_ID_MATERIAL).context("error")?;
    let mut game_flash_cards = create_from_cards(&sk, &rand_cards, Vec3::new(-1.0, -1.0, -14.0), 2, 1.5)?;

    let card_choice = rand_cards.choose(&mut thread_rng()).context("missing")?.clone();

    let mut question_text = high_level::text::Text::new(&sk, card_choice.term.0.as_str(), [0.0, 0.0, -6.0], [0.0, 180.0, 0.0], Vec3::new(1.0, 1.0, 1.0));
    question_text.size = Vec2::new(20.0, 6.0);
    question_text.text_style = TextStyle::new(&sk, Font::default(&sk), 1.0, Color128::new(Rgb::default(), 1.0));
    question_text.text_fit = TextFit::Wrap;

    let mut right_saber = high_level::model::Model::from_memory(&sk, "right_saber.glb", right_saber, Some(&Shader::p_b_r(&sk)))?;
    let mut left_saber = high_level::model::Model::from_memory(&sk, "left_saber.glb", left_saber, Some(&Shader::default(&sk)))?;

    right_saber.set_collider(&sk, ColliderType::CapsuleCollider);
    left_saber.set_collider(&sk, ColliderType::CapsuleCollider);

    sk.run(|sk, ctx| {
        question_text.draw_in(ctx);
        println!("{:?}", right_saber.get_collider(sk).unwrap());
        game_flash_cards.iter_mut().for_each(|card| {
            if card.model.collider_intersects(&sk, &right_saber.get_collider(sk).unwrap())
                || card.model.collider_intersects(&sk, &left_saber.get_collider(sk).unwrap()){
                card.touched = true;
                println!("intersecting!");
            }
            if !card.touched {
                card.draw(ctx);
                card.translate(0.0, 0.0, 0.01);
            }
        });
        let right_hand = sk.input_hand(Right).palm;
        let left_hand = sk.input_hand(Left).palm;
        right_saber.set_matrix(Mat4::from(right_hand.as_matrix()));
        left_saber.set_matrix(Mat4::from(left_hand.as_matrix()));
        right_saber.draw(ctx);
        left_saber.draw(ctx);
    }, |_| {});
    Ok(())
}

fn create_from_cards(sk: &StereoKit, cards: &Vec<Card>, mut corner: Vec3, row_len: i32, displacement: f32) -> Result<Vec<FlashCard>> {
    let cube_material = &Material::copy_from_id(sk, DEFAULT_ID_MATERIAL).context("error")?;
    let mut flash_cards = Vec::new();
    let mut current_row = 0;
    cards.iter().try_for_each(|card| -> Result<()> {
        let mut flash_card = FlashCard::new(sk, cube_material, card.definition.0.as_str(), 0.05)?;
        flash_card.rotate(0.0, 180.0, 0.0);

        if current_row >= row_len {
            current_row = 0;
            corner.y += displacement;
        }

        flash_card.set_pos_vec(corner.add(Vec3::new(displacement*current_row as f32, 0.0, 0.0)));
        flash_cards.push(flash_card);
        current_row += 1;
        Ok(())
    })?;
    Ok(flash_cards)

}