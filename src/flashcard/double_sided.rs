use std::ops::{Add, Mul, Sub};
use anyhow::{Result, Context};
use glam::{Mat4, Vec2, Vec3, Vec3Swizzles};
use prisma::{Rgb, Rgba};
use stereokit::high_level::model::Model;
use stereokit::mesh::Mesh;
use stereokit::{high_level, material, sprite, StereoKit, ui};
use stereokit::font::Font;
use stereokit::high_level::math_traits::{MatrixTrait, PosTrait, RotationTrait, ScaleTrait};
use stereokit::high_level::{quat_from_angles, quat_from_vec};
use stereokit::high_level::text::Text;
use stereokit::lifecycle::DrawContext;
use stereokit::material::{DEFAULT_ID_MATERIAL, Material};
use stereokit::pose::Pose;
use stereokit::text::{TextFit, TextStyle};
use stereokit::ui::{ButtonLayout, MoveType, WindowContext, WindowType};
use stereokit::values::Color128;
use stereokit::sprite::{Sprite, SpriteType};
use crate::flashcard::FlashCard;
use crate::flashcard_mode::FlashCardButtonPress;
use crate::stereokit_game::draw::SkDrawMut;

pub struct DoubleSidedFlashCard {
    model: Model,
    text_back: Text,
    text_front: Text,
    padding_ratio: f32,
    reverse_button_sprite: Sprite,
    rotation_state: RotationState,
}

enum RotationState {
    RotatingBack,
    RotatingFront,
    Back,
    Front,
}

impl DoubleSidedFlashCard {
    pub fn new(sk: &StereoKit, flash_card: &FlashCard, pos: Vec3) -> Result<Self> {
        let mut model = high_level::model::Model::from_mesh(sk, &Mesh::gen_cube(sk, Vec3::new(1.0, 1.0, 1.0), 1).context("missing")?, &Material::copy_from_id(sk, DEFAULT_ID_MATERIAL).context("missing")?)?;
        model.tint.set_color(Rgb::new(0.7, 0.7, 0.7));
        let mut return_value = Self {
            model,
            text_front: Text::from(sk, flash_card.front.clone()),
            text_back: Text::from(sk, flash_card.back.clone()),
            reverse_button_sprite: Sprite::from_file(sk, "./assets/Reverse.png", SpriteType::Single),
            rotation_state: RotationState::Front,
            padding_ratio: 0.1,
        };
        return_value.set_scale_vec([1.0, 0.8, 0.05]);
        return_value.sync_model_text_matrix();
        return_value.text_front.text_fit = TextFit::Wrap;
        return_value.text_back.text_fit = TextFit::Wrap;
        return_value.text_back.text_style = TextStyle::new(sk, Font::default(sk), 0.04, Color128::new(Rgb::default(), 1.0));
        return_value.text_front.text_style = TextStyle::new(sk, Font::default(sk), 0.04, Color128::new(Rgb::default(), 1.0));
        return_value.rotate(0.0, 180.0, 0.0);
        Ok(return_value)
    }
    pub fn set_text(&mut self, front: &String, back: &String) {
        self.text_front.text = front.clone();
        self.text_back.text = back.clone();
    }
    pub fn from(sk: &StereoKit, flash_card: &FlashCard) -> Result<Self> {
        Self::new(sk, flash_card, Vec3::default())
    }
    pub fn default(sk: &StereoKit) -> Result<Self> {
        Self::from(sk, &FlashCard::default())
    }
    fn sync_model_text_matrix(&mut self) {
        let new_matrix = self.model.get_matrix().mul_mat4(&Mat4::from_translation(Vec3::new(0.0, 0.0, -0.501)));
        self.text_front.set_matrix(new_matrix);
        let new_matrix = self.model.get_matrix().mul_mat4(&Mat4::from_rotation_translation(quat_from_angles(0.0, 180.0, 0.0), Vec3::new(0.0, 0.0, 0.501)));
        self.text_back.set_matrix(new_matrix);
    }
    fn animate_rotation(&mut self) {
        match self.rotation_state {
            RotationState::RotatingBack => {
                self.rotate(0.0, -10.0, 0.0);
                if self.get_rotation().1 <= 0.0 {
                    self.set_rotation(0.0, 0.0, 0.0);
                    self.rotation_state = RotationState::Back
                }
            }
            RotationState::RotatingFront => {
                self.rotate(0.0, 10.0, 0.0);
                if self.get_rotation().1 >= 180.0 {
                    self.set_rotation(0.0, 180.0, 0.0);
                    self.rotation_state = RotationState::Front
                }
            }
            RotationState::Back => {}
            RotationState::Front => {}
        }
    }
    pub fn draw_mut(&mut self, sk: &StereoKit, ctx: &DrawContext, button_state: &mut Option<FlashCardButtonPress>) {
        self.animate_rotation();
        self.model.draw(ctx);
        self.text_front.draw_in(ctx);
        self.text_back.draw_in(ctx);
        let ui_button_scale = self.get_scale_vec().xy().mul(Vec2::new(self.padding_ratio, self.padding_ratio));
        let right_side = self.get_scale_vec().x/2.0;
        let bottom_side = self.get_scale_vec().y/2.0;
        let mut ui_shared = |text: &Text| {
            ui::window::window(ctx, "", &mut Pose::new(text.get_pos_vec().into(), quat_from_vec(text.get_rotation_vec()).clone().into()), text.size.clone().into(), WindowType::WindowEmpty, MoveType::MoveNone, |ui| {
                ui.text_style(TextStyle::new(sk, Font::default(sk), 0.1, Rgba::new(Rgb::new(1.0, 1.0, 1.0), 1.0)), |ui| {
                    if ui.button_image_at("", &self.reverse_button_sprite, ButtonLayout::CenterNoText, [-right_side+ui_button_scale.x, bottom_side, 0.0].into(), ui_button_scale.into()) {
                        match self.rotation_state {
                            RotationState::RotatingBack | RotationState::Back => {
                                self.rotation_state = RotationState::RotatingFront;
                            }
                            RotationState::RotatingFront | RotationState::Front => {
                                self.rotation_state = RotationState::RotatingBack;
                            }
                        }
                    }
                    if ui.button_at("Correct", [-right_side+ui_button_scale.x, 0.0, 0.0].into(), ui_button_scale.into()) {
                        println!("correct");
                        button_state.replace(FlashCardButtonPress::Correct);
                    }
                    if ui.button_at("Incorrect", [right_side, 0.0, 0.0].into(), ui_button_scale.into()) {
                        button_state.replace(FlashCardButtonPress::Incorrect);
                    }
                    // if ui.button_at("Go Back", [right_side, bottom_side, 0.0].into(), ui_button_scale.into()) {
                    //     button_state.replace(FlashCardButtonPress::GoBack);
                    // }
                });
            });
        };
        ui_shared(&self.text_front);
        ui_shared(&self.text_back);
    }
}

impl PosTrait for DoubleSidedFlashCard {
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

impl RotationTrait for DoubleSidedFlashCard {
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

impl ScaleTrait for DoubleSidedFlashCard {
    fn get_scale_vec(&self) -> Vec3 {
        self.model.get_scale_vec()
    }

    fn set_scale_vec(&mut self, scale: impl Into<stereokit::values::Vec3>) {
        let scale = glam::Vec3::from(scale.into());
        self.model.set_scale_vec(scale.add(Vec3::new(scale.x*self.padding_ratio, 0.0, 0.0)));
        self.text_front.size = self.model.get_scale_vec().xy().sub(Vec2::new(scale.x*self.padding_ratio*2.0, 0.0));
        self.text_back.size = self.model.get_scale_vec().xy().sub(Vec2::new(scale.x*self.padding_ratio*2.0, 0.0));
    }

    fn scale_vec(&mut self, scale: impl Into<stereokit::values::Vec3>) {
        let scale = glam::Vec3::from(scale.into());
        self.model.scale_vec(scale);
        self.text_front.size = self.model.get_scale_vec().xy().sub(Vec2::new(scale.x*self.padding_ratio*2.0, 0.0));
        self.text_back.size = self.model.get_scale_vec().xy().sub(Vec2::new(scale.x*self.padding_ratio*2.0, 0.0));
    }
}