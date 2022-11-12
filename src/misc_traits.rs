use std::ops::Deref;
use stereokit::lifecycle::DrawContext;
use stereokit::StereoKit;
use anyhow::Result;
use stereokit::ui::WindowContext;

pub trait SKLoop<T> {
    fn create(sk: &StereoKit) -> Result<Self> where Self: Sized;
    fn tick(&mut self, sk: &StereoKit, ctx: &DrawContext) -> Result<Option<T>>;
}
pub trait UiLoop<T> {
    fn create(sk: &StereoKit) -> Result<Self> where Self: Sized;
    fn tick(&mut self, sk: &StereoKit, ctx: &DrawContext, ui: &WindowContext) -> Result<Option<T>>;
}

pub enum QuizSaberStageType {
    FlashCardSaberStage,
    MainMenu,
}
pub struct QuizSaberStage(QuizSaberStageType);
impl QuizSaberStage {
    pub fn set(&mut self, other: QuizSaberStageType) {
        self.0 = other
    }
    pub fn from(other: QuizSaberStageType) -> Self {
        Self(other)
    }
}
impl Deref for QuizSaberStage {
    type Target = QuizSaberStageType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}