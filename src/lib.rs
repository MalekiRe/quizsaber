mod parser;
mod saber_game_loop;
mod quiz_saber_stage;
mod popup_menu;
mod stereokit_game;
mod main_menu;
pub mod anki_parser;
mod flashcard_1;
mod flashcard;
mod flashcard_mode;
mod flashcard_system;

use std::ops::{Add, Deref, Mul, Neg};
use glam::{Mat4, Quat, Vec2, Vec3, vec3, Vec3Swizzles};
use prisma::{Rgb, Rgba};
use stereokit::material::{DEFAULT_ID_MATERIAL, Material};
use stereokit::mesh::Mesh;
use stereokit::model::Model;
use stereokit::render::RenderLayer;
use stereokit::{high_level, material, mesh, pose, Settings, StereoKit, ui};
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
use random_number::random;
use stereokit::font::Font;
use stereokit::high_level::collider::{Collider, ColliderType};
use stereokit::high_level::quat_from_angles;
use stereokit::input::Handed;
use stereokit::input::Handed::{Left, Right};
use stereokit::lifecycle::DrawContext;
use stereokit::pose::Pose;
use stereokit::shader::Shader;
use stereokit::text::{TextAlign, TextFit, TextStyle};
use stereokit::ui::{MoveType, WindowType};
use stereokit::values::{Color128, SKMatrix};
use crate::flashcard_mode::FlashCardMode;
use crate::flashcard_system::FlashCardSystem;

use crate::main_menu::MainMenuWindow;
use crate::saber_game_loop::{FlashCardSaberStage, SaberOffsets};
use crate::quiz_saber_stage::{QuizSaberStage, QuizSaberStageType};
use crate::parser::{parse, TEST_FILE};
use crate::stereokit_game::sk_loop::SkGameLoop;
use crate::stereokit_game::stage::SkStage;

pub fn my_func() -> Result<()> {
    println!("Starting up QuizSaber!");
    let sk = Settings::default().init().context("startup error")?;

    let mut saber_offsets = SaberOffsets::default();
    let mut saber_game_loop = FlashCardSaberStage::init(&sk, ())?;
    let mut main_menu = MainMenuWindow::init(&sk, ())?;
    let mut stage = QuizSaberStage::new(QuizSaberStageType::MainMenu);
    let mut flash_card_stage = FlashCardMode::init(&sk, ())?;


    let mut flashcard_system = FlashCardSystem::init(&sk, ())?;

    let mut flash_cards_data = parse(TEST_FILE);
    let mut index: usize = random!(0, flash_cards_data.len()-1);
    let mut guess = None;
    sk.run(|sk, ctx| {
        match stage.get() {
            QuizSaberStageType::FlashCardSaberStage => saber_game_loop.tick(sk, ctx, (&mut stage, &mut saber_offsets)).unwrap(),
            QuizSaberStageType::MainMenu => main_menu.tick(sk, ctx, (&mut stage, &mut saber_offsets)).unwrap(),
            QuizSaberStageType::FlashCardStage => flash_card_stage.tick(sk, ctx, (&mut flash_cards_data, index, &mut guess)).unwrap(),
        }
        let mut wrapper = IndexWrapper(index);
        flashcard_system.tick(&sk, &ctx, (&mut flash_cards_data, &mut guess, &mut wrapper)).unwrap();
        index = wrapper.0;
        guess = None;
    }, |_| {});
    Ok(())
}

struct IndexWrapper(usize);

// pub fn my_func() -> Result<()> {
//     println!("Hello, world!");
//     let right_saber = include_bytes!("../resources/saber1.glb");
//     let left_saber = include_bytes!("../resources/saber2.glb");
//     let flash_cards = parser::parse(TEST_FILE);
//     let rand_cards: Vec<Card> = flash_cards.choose_num(4);
//
//     // println!("random_cards: {:?}", rand_cards);
//     let mut sk = Settings::default().init().unwrap();
//     let cube_material = Material::copy_from_id(&sk, DEFAULT_ID_MATERIAL).context("error")?;
//     let mut game_flash_cards = create_from_cards(&sk, &rand_cards, Vec3::new(-1.0, -1.0, -14.0), 2, 1.5)?;
//
//     let card_choice = rand_cards.choose(&mut thread_rng()).context("missing")?.clone();
//
//     let mut question_text = high_level::text::Text::new(&sk, card_choice.term.0.as_str(), [0.0, 0.0, -6.0], [0.0, 180.0, 0.0], Vec3::new(1.0, 1.0, 1.0));
//     question_text.size = Vec2::new(20.0, 6.0);
//     question_text.text_style = TextStyle::new(&sk, Font::default(&sk), 1.0, Color128::new(Rgb::default(), 1.0));
//     question_text.text_fit = TextFit::Wrap;
//
//     let mut right_saber = high_level::model::Model::from_memory(&sk, "right_saber.glb", right_saber, Some(&Shader::p_b_r(&sk)))?;
//     let mut left_saber = high_level::model::Model::from_memory(&sk, "left_saber.glb", left_saber, Some(&Shader::default(&sk)))?;
//
//     right_saber.set_collider(&sk, ColliderType::CapsuleCollider);
//     left_saber.set_collider(&sk, ColliderType::CapsuleCollider);
//
//     let mut game_state = GameState::MainMenu;
//     let mut main_menu_pos = Pose::new([0.0, 0.3, -0.3].into(), quat_from_angles(0.0, 180.0, 0.0).into());
//     sk.run(|sk, ctx| {
//         match game_state {
//             GameState::MainMenu => {
//                 ui::window::window(ctx, "QuizSaber", &mut main_menu_pos, Vec2::new(1.0, 1.0).into(), WindowType::WindowNormal, MoveType::MoveFaceUser, |ui| {
//                     ui.push_text_style(TextStyle::new(sk, Font::default(sk), 0.04, Color128::new(Rgb::new(1.0, 1.0, 1.0), 1.0)));
//                     if ui.button("Start") {
//                         game_state = MainLoop
//                     }
//                     if ui.button("Credits") {
//
//                     }
//                     ui.pop_text_style();
//                 });
//             }
//             MainLoop => {
//                 question_text.draw_in(ctx);
//                 println!("{:?}", right_saber.get_collider(sk).unwrap());
//                 game_flash_cards.iter_mut().for_each(|card| {
//                     if card.model.collider_intersects(&sk, &right_saber.get_collider(sk).unwrap())
//                         || card.model.collider_intersects(&sk, &left_saber.get_collider(sk).unwrap()){
//                         card.touched = true;
//                         println!("intersecting!");
//                     }
//                     if !card.touched {
//                         card.draw(ctx);
//                         card.translate(0.0, 0.0, 0.01);
//                     }
//                 });
//                 let right_hand = sk.input_hand(Right).palm;
//                 let left_hand = sk.input_hand(Left).palm;
//                 right_saber.set_matrix(Mat4::from(right_hand.as_matrix()));
//                 left_saber.set_matrix(Mat4::from(left_hand.as_matrix()));
//                 right_saber.draw(ctx);
//                 left_saber.draw(ctx);
//             }
//         }
//     }, |_| {});
//     Ok(())
// }

// fn create_from_cards(sk: &StereoKit, cards: &Vec<Card>, mut corner: Vec3, row_len: i32, displacement: f32) -> Result<Vec<FlashCard>> {
//     let cube_material = &Material::copy_from_id(sk, DEFAULT_ID_MATERIAL).context("error")?;
//     let mut flash_cards = Vec::new();
//     let mut current_row = 0;
//     cards.iter().try_for_each(|card| -> Result<()> {
//         let mut flash_card = FlashCard::new(sk, cube_material, card.definition.0.as_str(), 0.05)?;
//         flash_card.rotate(0.0, 180.0, 0.0);
//
//         if current_row >= row_len {
//             current_row = 0;
//             corner.y += displacement;
//         }
//
//         flash_card.set_pos_vec(corner.add(Vec3::new(displacement*current_row as f32, 0.0, 0.0)));
//         flash_cards.push(flash_card);
//         current_row += 1;
//         Ok(())
//     })?;
//     Ok(flash_cards)
// }