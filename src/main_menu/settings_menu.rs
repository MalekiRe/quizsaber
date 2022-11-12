use glam::Mat4;
use stereokit::lifecycle::DrawContext;
use stereokit::{StereoKit, ui};
use stereokit::pose::Pose;
use stereokit::text::TextAlign;
use stereokit::ui::{ConfirmMethod, MoveType, WindowContext, WindowType};
use crate::main_menu::{MainMenuState, new_ui_text_style};
use crate::misc_traits::{GameState, SKLoop, UiLoop};

pub struct SettingsMenu {
    pub offset_matrix: Option<Mat4>,
    slider_val: f32,
}
impl UiLoop<MainMenuState> for SettingsMenu {
    fn create(sk: &StereoKit) -> anyhow::Result<Self> where Self: Sized {
        Ok(Self{
            offset_matrix: None,
            slider_val: 0.0,
        })
    }

    fn tick(&mut self, sk: &StereoKit, ctx: &DrawContext, ui: &WindowContext) -> anyhow::Result<Option<MainMenuState>> {
        let mut return_value = None;
        ui.text_style(new_ui_text_style(sk, 0.05),  |ui| {
            ui.text("Settings Menu", TextAlign::TopCenter);
            ui.text_style(new_ui_text_style(sk, 0.03),  |ui| {
                ui.slider("slider_test", &mut self.slider_val, 0.0, 10.0, 0.1, 0.1, ConfirmMethod::Push);
            });
            if ui.button("Back") {
                return_value = Some(MainMenuState::MainState);
            }
        });
        Ok(return_value)
    }
}