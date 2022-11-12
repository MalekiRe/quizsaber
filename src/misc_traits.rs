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

#[derive(Clone, Copy)]
pub enum GameState {
    MainGameLoop,
    MainMenu,
}