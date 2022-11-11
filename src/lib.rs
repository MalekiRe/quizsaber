mod parser;
mod flashcard;

use std::ops::Add;
use glam::{Mat4, Quat, Vec3, vec3};
use prisma::{Rgb, Rgba};
use stereokit::material::{DEFAULT_ID_MATERIAL, Material};
use stereokit::mesh::Mesh;
use stereokit::model::Model;
use stereokit::render::RenderLayer;
use stereokit::{high_level, pose, Settings, StereoKit, ui};
use stereokit::high_level::math_traits::{PosTrait, RotationTrait, ScaleTrait};

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on", logger(level = "debug", tag = "my-tag")))]
pub fn main() {
    println!("Hello World");
    my_func().unwrap();
}
use anyhow::{Context, Result};
use mint::{Quaternion, Vector3};
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


    flash_card.translate(0.0, 0.0, -0.7);
    flash_card.set_scale(1.0, 1.0, 0.1);
    flash_card.tint.set_color(Rgb::new(0.5, 0.5, 0.5));

    let mut answer_text = high_level::text::Text::new(&stereokit, rand_cards.unwrap().0.term.0, [0.0, 0.0, -0.6], [0.0, 180.0, 0.0], [1.0, 1.0, 1.0]);
    stereokit.run(|sk, ctx| {
        flash_card.draw(ctx);
        answer_text.draw_at(ctx);

    }, |_| {});
    Ok(())
}

// struct FlashCardCube {
//     position: Vec3,
//     rotation: Quat,
//     text_rotation: Quat,
//     cube_model: Model,
//     tint: Rgba<f32>,
//     text: String,
//     visible: bool,
// }
// impl FlashCardCube {
//     pub fn new(sk: &mut StereoKit) -> Option<Self> {
//         let cube_mesh = Mesh::gen_cube(&sk, Vec3::new(0.05, 1.0, 1.0).into(), 1)?;
//         let cube_material = Material::copy_from_id(&sk, DEFAULT_ID_MATERIAL)?;
//         Some(FlashCardCube {
//             position: Default::default(),
//             cube_model: Model::from_mesh(&sk, &cube_mesh, &cube_material)?,
//             tint: Rgba::new(Rgb::new(0.6_f32, 0.5_f32, 0.5_f32), 1_f32),
//             text: "test_text".to_string(),
//             rotation: Quat::from_rotation_y(55f32),
//             text_rotation: Quat::from_rotation_y(135f32),
//             visible: true,
//         })
//     }
//     pub fn draw(&self, sk: &mut StereoKit, ctx: &DrawContext) {
//         let pos = Mat4::from_scale_rotation_translation(Vec3::new(1f32, 1f32, 1f32), self.rotation.into(), self.position).into();
//         let temp_pos = self.position.add(Vec3::new(00f32, 0f32, 0.2f32));
//         let text_pos = Mat4::from_scale_rotation_translation(Vec3::new(1f32, 1f32, 1f32), self.text_rotation.into(), temp_pos);
//         self.cube_model.draw(ctx, pos, self.tint, RenderLayer::Layer0);
//         stereokit::text::draw_at(&ctx,
//                                  &self.text.as_str(),
//                                  text_pos,
//                                  &TextStyle::default(sk),
//                                  TextAlign::Center,
//                                  TextAlign::Center,
//                                  Vec3::new(0f32, 0f32, 0f32),
//                                  Rgba::new(Rgb::new(1_f32, 1_f32, 1_f32), 1_f32).into());
//     }
//     pub fn contains(&self, sk: &StereoKit, point: Vec3) -> bool {
//         let pos = Mat4::from_scale_rotation_translation(Vec3::new(1f32, 1f32, 1f32), self.rotation.into(), self.position);
//         let mut matrix = SKMatrix::new(pos.into());
//         self.cube_model.get_bounds(sk).bounds_point_contains(matrix.transform_point(point.into()))
//     }
// }
// saber_model.draw(
//     drw_ctx,
//     Mat4::from_scale_rotation_translation(
//         vec3(0.01, 2.0, 0.01),
//         right_hand.palm.orientation.into(),
//         Vec3::from(right_hand.palm.position).add(Vec3::new(0.0, 0.9, 0.0)),
//     ).into(),
//     Rgba::new(Rgb::new(1_f32, 1_f32, 1_f32), 1_f32).into(),
//     RenderLayer::Layer0);
