use stereokit::lifecycle::DrawContext;
use stereokit::StereoKit;
use stereokit::ui::WindowContext;
use crate::main_menu::{MainMenuStage, MainMenuStageType, new_ui_text_style};
use crate::quiz_saber_stage::{QuizSaberStage, QuizSaberStageType};
use crate::stereokit_game::sk_loop::SkGameLoop;
use crate::stereokit_game::stage::SkStage;

pub struct MainMenu {
}

impl SkGameLoop<(), (&mut MainMenuStage, &mut QuizSaberStage, &WindowContext)> for MainMenu {
    fn init(sk: &StereoKit, init_data: ()) -> anyhow::Result<Self> where Self: Sized {
        Ok(Self{})
    }

    fn tick(&mut self, sk: &StereoKit, ctx: &DrawContext, run_data: (&mut MainMenuStage, &mut QuizSaberStage, &WindowContext)) -> anyhow::Result<()> {
        let (main_menu_stage, quiz_saber_stage, ui) = run_data;
        ui.text_style(new_ui_text_style(sk, 0.05),  |ui| {
            if ui.button("Start") {
                quiz_saber_stage.set(QuizSaberStageType::FlashCardSaberStage);
            }
            if ui.button("Settings") {
                main_menu_stage.set(MainMenuStageType::Settings);
            }
            if ui.button("Credits") {
                main_menu_stage.set(MainMenuStageType::Credits);
            }
        });
        Ok(())
    }
}