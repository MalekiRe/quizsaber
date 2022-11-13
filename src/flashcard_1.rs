use std::ops::{Add, Deref, DerefMut};
use anyhow::Context;
use glam::{Mat4, Vec3, Vec3Swizzles};
use prisma::Rgb;
use rand::prelude::SliceRandom;
use rand::{thread_rng};
use stereokit::{high_level, StereoKit};
use stereokit::font::Font;
use stereokit::high_level::math_traits::{MatrixTrait, PosTrait, RotationTrait, ScaleTrait};
use stereokit::lifecycle::DrawContext;
use stereokit::material::Material;
use stereokit::mesh::Mesh;
use stereokit::text::{TextFit, TextStyle};
use stereokit::values::Color128;
use anyhow::Result;
use stereokit::sys::vind_t;

#[derive(Debug, Clone)]
pub struct Term(pub String);
#[derive(Debug, Clone)]
pub struct Definition(pub String);
#[derive(Debug, Clone)]
pub struct Card {
    pub(crate) term: Term,
    pub(crate) definition: Definition
}
pub struct Deck(pub Vec<Card>);

impl Deref for Deck {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Deck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deck {
    pub fn choose_num(&self, num: usize) -> Vec<Card> {
        let deck = &self.0;
        deck.choose_multiple(&mut thread_rng(), num).map(|my_card| {
            my_card.clone()
        }).collect()
    }
}
pub trait IntoRand<T> {
    fn into_rand(self) -> T;
}

impl IntoRand<Option<(Card, Card)>> for Deck {
    fn into_rand(self) -> Option<(Card, Card)> {
        let things = self.choose_num(2);
        Some((things.get(0)?.clone(), things.get(1)?.clone()))
    }
}
impl IntoRand<Option<(Card, Card, Card)>> for Deck {
    fn into_rand(self) -> Option<(Card, Card, Card)> {
        let things = self.choose_num(3);
        Some((things.get(0)?.clone(),
              things.get(1)?.clone(),
              things.get(2)?.clone(),
        ))
    }
}
impl IntoRand<Option<(Card, Card, Card, Card)>> for Deck {
    fn into_rand(self) -> Option<(Card, Card, Card, Card)> {
        let things = self.choose_num(4);
        Some((things.get(0)?.clone(),
              things.get(1)?.clone(),
              things.get(2)?.clone(),
              things.get(3)?.clone(),
        ))
    }
}
impl IntoRand<Option<(Card, Card, Card, Card, Card)>> for Deck {
    fn into_rand(self) -> Option<(Card, Card, Card, Card, Card)> {
        let things = self.choose_num(5);
        Some((things.get(0)?.clone(),
              things.get(1)?.clone(),
              things.get(2)?.clone(),
              things.get(3)?.clone(),
              things.get(4)?.clone(),
        ))
    }
}
pub struct FlashCard {
    pub model: high_level::model::Model,
    pub text: high_level::text::Text,
    pub touched: bool,
    card_mesh: Mesh,
}
impl FlashCard {
    pub fn new(sk: &StereoKit, material: &Material, text: &str, char_height: f32) -> Result<Self> {
        let card_mesh = Mesh::gen_cube(sk, [1.0, 1.0, 1.0], 1).context("missing")?;
        let mut model = high_level::model::Model::from_mesh(sk, &card_mesh, material)?;
        model.tint.set_color(Rgb::new(0.7, 0.7, 0.7));
        let mut text = high_level::text::Text::new(sk, text, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        //text.text_fit = TextFit::Clip;
        text.text_fit = TextFit::Wrap;
        text.text_style = TextStyle::new(sk, Font::default(sk), char_height, Color128::new(Rgb::default(), 1.0));
        let mut card = FlashCard {
            model,
            text,
            card_mesh,
            touched: false,
        };
        card.set_scale_vec([0.8, 0.8, 0.2]);
        card.sync_model_text_matrix();
        Ok(card)
    }
    pub fn draw(&self, ctx: &DrawContext) {
        self.model.draw(ctx);
        self.text.draw_in(ctx);
    }
    fn sync_model_text_matrix(&mut self) {
        let new_matrix = self.model.get_matrix().mul_mat4(&Mat4::from_translation(Vec3::new(0.0, 0.0, -0.501)));
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
impl ScaleTrait for FlashCard {
    fn get_scale_vec(&self) -> Vec3 {
        self.model.get_scale_vec()
    }

    fn set_scale_vec(&mut self, scale: impl Into<stereokit::values::Vec3>) {
        self.model.set_scale_vec(scale);
        self.text.size = self.model.get_scale_vec().xy();
    }

    fn scale_vec(&mut self, scale: impl Into<stereokit::values::Vec3>) {
        self.model.scale_vec(scale);
        self.text.size = self.model.get_scale_vec().xy();
    }
}