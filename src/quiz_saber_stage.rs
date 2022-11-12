use std::ops::Deref;
use stereokit::lifecycle::DrawContext;
use stereokit::StereoKit;
use anyhow::Result;
use stereokit::ui::WindowContext;
use crate::stereokit_game::stage::SkStage;

pub enum QuizSaberStageType {
    FlashCardSaberStage,
    MainMenu,
}
pub struct QuizSaberStage(QuizSaberStageType);
impl SkStage<QuizSaberStageType> for QuizSaberStage {
    fn new(stage_type: QuizSaberStageType) -> Self {
        Self(stage_type)
    }

    fn get(&self) -> &QuizSaberStageType {
        &self.0
    }

    fn set(&mut self, stage_type: QuizSaberStageType) {
        self.0 = stage_type
    }
}
