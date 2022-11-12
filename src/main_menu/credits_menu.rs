use stereokit::lifecycle::DrawContext;
use stereokit::StereoKit;
use stereokit::text::TextAlign;
use stereokit::ui::WindowContext;
use crate::main_menu::{MainMenuStage, MainMenuStageType, new_ui_text_style};
use crate::stereokit_game::sk_loop::SkGameLoop;

pub struct CreditsMenu {}
impl SkGameLoop<(), (&WindowContext, &mut MainMenuStage)> for CreditsMenu {
    fn init(sk: &StereoKit, init_data: ()) -> anyhow::Result<Self> where Self: Sized {
        Ok(Self{})
    }

    fn tick(&mut self, sk: &StereoKit, ctx: &DrawContext, run_data: (&WindowContext, &mut MainMenuStage)) -> anyhow::Result<()> {
        let (ui, main_menu_stage) = run_data;
        ui.text_style(new_ui_text_style(sk, 0.07),  |ui| {
            ui.text("Credits", TextAlign::TopCenter);
        });
        ui.text_style(new_ui_text_style(sk, 0.03), |ui| {
            ui.text("Original Idea: Ryezun", TextAlign::CenterLeft);
            ui.text("Developer: Malek", TextAlign::CenterLeft);
            ui.text("Music: meganeko on YouTube", TextAlign::CenterLeft);
        });
        ui.text_style(new_ui_text_style(sk, 0.05), |ui| {
            if ui.button("Back") {
                main_menu_stage.set(MainMenuStageType::Main);
            }
        });
        Ok(())
    }
}