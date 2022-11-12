use prisma::Rgb;
use stereokit::lifecycle::DrawContext;
use stereokit::{StereoKit, ui};
use stereokit::font::Font;
use stereokit::high_level::quat_from_angles;
use stereokit::pose::Pose;
use stereokit::text::TextStyle;
use stereokit::ui::{MoveType, WindowContext, WindowType};
use stereokit::values::Color128;
use crate::main_menu::{MainMenuState, new_ui_text_style};
use crate::misc_traits::{GameState, SKLoop, UiLoop};

pub struct MainMenu {
}
impl UiLoop<MainMenuState> for MainMenu {
    fn create(sk: &StereoKit) -> anyhow::Result<Self> where Self: Sized {
        Ok(Self {})
    }

    fn tick(&mut self, sk: &StereoKit, ctx: &DrawContext, ui: &WindowContext) -> anyhow::Result<Option<MainMenuState>> {
        ui.push_text_style(new_ui_text_style(sk, 0.05));
        if ui.button("Start") {
            return Ok(Some(MainMenuState::GameState(GameState::MainGameLoop)));
        }
        if ui.button("Settings") {
            return Ok(Some(MainMenuState::Settings));
        }
        if ui.button("Credits") {
            return Ok(Some(MainMenuState::Credits));
        }
        ui.pop_text_style();
        Ok(None)
    }
}

