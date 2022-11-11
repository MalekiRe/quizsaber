mod parser;
mod flashcard;

use std::ops::{Add, Mul, Neg};
use glam::{Mat4, Quat, Vec3, vec3};
use prisma::{Rgb, Rgba};
use stereokit::material::{DEFAULT_ID_MATERIAL, Material};
use stereokit::mesh::Mesh;
use stereokit::model::Model;
use stereokit::render::RenderLayer;
use stereokit::{high_level, pose, Settings, StereoKit, ui};
use stereokit::high_level::math_traits::{MatrixTrait, PosTrait, RotationTrait, ScaleTrait};

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on", logger(level = "debug", tag = "my-tag")))]
pub fn main() {
    println!("Hello World");
    my_func().unwrap();
}
use anyhow::{Context, Result};
use mint::{Quaternion, Vector3};
use stereokit::high_level::quat_from_angles;
use stereokit::input::Handed;
use stereokit::input::Handed::Right;
use stereokit::lifecycle::DrawContext;
use stereokit::pose::Pose;
use stereokit::text::{TextAlign, TextStyle};
use stereokit::values::SKMatrix;
use crate::flashcard::{Card, IntoRand};
use crate::parser::TEST_FILE;


pub fn my_func() -> Result<()> {
    let WHITE: Rgba<f32> = Rgba::new(Rgb::new(1_f32, 1_f32, 1_f32), 1_f32).into();
    println!("Hello, world!");
    let flash_cards = parser::parse(TEST_FILE);
    let rand_cards: Option<(Card, Card, Card)> = flash_cards.into_rand();
    println!("random_cards: {:?}", rand_cards);
    let mut stereokit = Settings::default().init().unwrap();
    let cube_mesh = Mesh::gen_cube(&stereokit, Vec3::new(1.0, 1.0, 1.0).into(), 1).context("err")?;
    let cube_material = Material::copy_from_id(&stereokit, DEFAULT_ID_MATERIAL).context("error")?;
    let cube_model = Model::from_mesh(&stereokit, &cube_mesh, &cube_material).context("error")?;


    let saber_model = Model::from_mesh(&stereokit, &cube_mesh, &cube_material).context("error")?;
    let mut my_pos = Pose::new(Vec3::new(0f32, 0f32, 30f32).into(), Quat::IDENTITY.into());

    let mut flash_card = high_level::model::Model::from_mesh(&stereokit, &cube_mesh, &cube_material)?;
    flash_card.tint.set_color(Rgb::new(0.5, 0.5, 0.5));
    let mut answer_text = high_level::text::Text::new(&stereokit, rand_cards.unwrap().0.term.0, [0.0, 0.0, -0.6], [0.0, 180.0, 0.0], [1.0, 1.0, 1.0]);

    //flash_card.translate(0.0, 0.0, -0.7);
    //flash_card.set_scale(1.0, 1.0, 0.1);
    //flash_card.tint.set_color(Rgb::new(0.5, 0.5, 0.5));

    let mut flash_card = FlashCard {
        model: flash_card,
        text: answer_text,
    };
    flash_card.set_pos(0.0, 0.0, -3.0);
    flash_card.set_rotation(0.0, 30.0, 0.0);
    let mut rotation_value = 0.0;
    stereokit.run(|sk, ctx| {
        //flash_card.draw(ctx);
        //answer_text.draw_at(ctx);
        flash_card.draw(ctx);
        flash_card.rotate(0.0, 0.6, 0.0);
        flash_card.translate(0.0, 0.0, 0.005);
        //flash_card.set_rotation(0.0, rotation_value, 0.0);
        rotation_value += 0.1;

    }, |_| {});
    Ok(())
}

struct FlashCard {
    pub model: high_level::model::Model,
    pub text: high_level::text::Text,
}
impl FlashCard {
    pub fn draw(&self, ctx: &DrawContext) {
        self.model.draw(ctx);
        self.text.draw_at(ctx);
    }
    fn sync_model_text_matrix(&mut self) {
        let new_matrix = self.model.get_matrix().mul_mat4(&Mat4::from_translation(Vec3::new(0.0, 0.0, -0.5001)));
        self.text.set_matrix(new_matrix);
    }
}

impl PosTrait for FlashCard {
    fn get_pos_vec(&self) -> Vec3 {
        self.model.get_pos_vec()
    }

    fn set_pos_vec(&mut self, pos: impl Into<stereokit::values::Vec3>) {
        self.model.set_pos_vec(pos.into().clone());
        self.sync_model_text_matrix();
    }

    fn translate_vec(&mut self, translation: impl Into<stereokit::values::Vec3>) {
        self.set_pos_vec(glam::Vec3::from(translation.into()).add(self.get_pos_vec()));
    }
}
impl RotationTrait for FlashCard {
    fn get_rotation_vec(&self) -> Vec3 {
        self.model.get_rotation_vec()
    }

    fn set_rotation_vec(&mut self, rotation: impl Into<stereokit::values::Vec3>) {
        let rotation = rotation.into();
        self.model.set_rotation_vec(rotation.clone());
        self.sync_model_text_matrix();
    }

    fn rotate_vec(&mut self, rotation: impl Into<stereokit::values::Vec3>) {
        self.set_rotation_vec(glam::Vec3::from(rotation.into()).add(self.get_rotation_vec()))
    }
}
