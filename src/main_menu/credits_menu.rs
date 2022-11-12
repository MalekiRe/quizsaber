use stereokit::lifecycle::DrawContext;
use stereokit::{StereoKit, ui};
use stereokit::pose::Pose;
use stereokit::text::TextAlign;
use stereokit::ui::{MoveType, WindowContext, WindowType};
use crate::main_menu::{MainMenuState, new_ui_text_style};
use crate::misc_traits::{SKLoop, UiLoop};

pub struct CreditsMenu {
}
impl UiLoop<MainMenuState> for CreditsMenu {
    fn create(sk: &StereoKit) -> anyhow::Result<Self> where Self: Sized {
        Ok(Self{})
    }

    fn tick(&mut self, sk: &StereoKit, ctx: &DrawContext, ui: &WindowContext) -> anyhow::Result<Option<MainMenuState>> {
        ui.push_text_style(new_ui_text_style(sk, 0.07));
        ui.text("Credits Menu", TextAlign::TopCenter);
        ui.push_text_style(new_ui_text_style(sk, 0.03));
        ui.text("Original Idea: Ryezun", TextAlign::CenterLeft);
        ui.text("Developer: Malek", TextAlign::CenterLeft);
        ui.text("Music: meganeko on YouTube", TextAlign::CenterLeft);
        ui.text("", TextAlign::BottomLeft);
        ui.push_text_style(new_ui_text_style(sk, 0.05));
        if ui.button("Back") {
            return Ok(Some(MainMenuState::MainState));
        }
        ui.pop_text_style();
        Ok(None)
    }
}