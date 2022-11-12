use stereokit::lifecycle::DrawContext;
use stereokit::StereoKit;
use stereokit::text::TextAlign;
use stereokit::ui::{ConfirmMethod, WindowContext};
use crate::main_menu::{MainMenuStage, MainMenuStageType, new_ui_text_style};
use crate::misc_traits::QuizSaberStage;
use crate::saber_game_loop::SaberOffsets;
use crate::stereokit_game::sk_loop::SkGameLoop;

pub struct SettingsMenu {
    slider_val: f32
}
impl SkGameLoop<(), (&WindowContext, &mut MainMenuStage, &mut SaberOffsets)> for SettingsMenu {
    fn init(sk: &StereoKit, init_data: ()) -> anyhow::Result<Self> where Self: Sized {
        Ok(Self{
            slider_val: 0.0
        })
    }

    fn tick(&mut self, sk: &StereoKit, ctx: &DrawContext, run_data: (&WindowContext, &mut MainMenuStage, &mut SaberOffsets)) -> anyhow::Result<()> {
        let (ui, quiz_saber_stage, saber_offsets) = run_data;
        ui.text_style(new_ui_text_style(sk, 0.05),  |ui| {
            ui.text("Settings Menu", TextAlign::TopCenter);
            ui.text_style(new_ui_text_style(sk, 0.03),  |ui| {
                ui.slider("slider_test", &mut self.slider_val, 0.0, 10.0, 0.1, 0.1, ConfirmMethod::Push);
            });
            if ui.button("Back") {
                quiz_saber_stage.set(MainMenuStageType::Main)
            }
        });

        Ok(())
    }
}