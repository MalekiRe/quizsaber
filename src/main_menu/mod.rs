mod main_menu;
mod settings_menu;
mod credits_menu;

use std::ops::Deref;
use prisma::Rgb;
use stereokit::font::Font;
use stereokit::high_level::quat_from_angles;
use stereokit::lifecycle::DrawContext;
use stereokit::pose::Pose;
use stereokit::{StereoKit, ui};
use stereokit::high_level::model::Model;
use stereokit::shader::Shader;
use stereokit::text::TextStyle;
use stereokit::ui::{MoveType, WindowType};
use stereokit::values::Color128;
use crate::main_menu::credits_menu::CreditsMenu;
use crate::main_menu::main_menu::MainMenu;
use crate::main_menu::settings_menu::SettingsMenu;
use crate::quiz_saber_stage::{QuizSaberStage, QuizSaberStageType};
use crate::saber_game_loop::SaberOffsets;
use crate::stereokit_game::sk_loop::SkGameLoop;
use crate::stereokit_game::stage::SkStage;

pub struct MainMenuWindow {
    pose: Pose,
    main_menu_stage: MainMenuStage,
    pub settings_menu: SettingsMenu,
    main_menu: MainMenu,
    credits_menu: CreditsMenu,
}

impl SkGameLoop<(), (&mut QuizSaberStage, &mut SaberOffsets, &mut Model, &mut Model)> for MainMenuWindow {
    fn init(sk: &StereoKit, init_data: ()) -> anyhow::Result<Self> where Self: Sized {
        Ok(Self {
            pose: Pose::new([0.0, 0.0, -0.4].into(), quat_from_angles(0.0, 180.0, 0.0).into()),
            main_menu_stage: MainMenuStage::new(MainMenuStageType::Main),
            settings_menu: SettingsMenu::init(sk, ())?,
            main_menu: MainMenu::init(sk, ())?,
            credits_menu: CreditsMenu::init(sk, ())?,
        })
    }

    fn tick(&mut self, sk: &StereoKit, ctx: &DrawContext, run_data: (&mut QuizSaberStage, &mut SaberOffsets, &mut Model, &mut Model)) -> anyhow::Result<()> {
        let (quiz_saber_stage, saber_offsets, right_saber, left_saber) = run_data;
        ui::window::try_window(ctx, "QuizSaber", &mut self.pose, [1.7, 1.0].into(), WindowType::WindowBody, MoveType::MoveNone, |ui| -> anyhow::Result<()> {
            match self.main_menu_stage.get() {
                MainMenuStageType::Main => self.main_menu.tick(sk, ctx, (&mut self.main_menu_stage, quiz_saber_stage, ui))?,
                MainMenuStageType::Settings => self.settings_menu.tick(sk, ctx, (ui, &mut self.main_menu_stage, saber_offsets))?,
                MainMenuStageType::Credits => self.credits_menu.tick(sk, ctx, (ui, &mut self.main_menu_stage))?,
            }
            Ok(())
        })?;
        if matches!(self.main_menu_stage.get(), MainMenuStageType::Settings) {
            if self.settings_menu.enable_sabers {
                saber_offsets.set_sabers_to_hands(sk, right_saber, left_saber);
                right_saber.draw(ctx);
                left_saber.draw(ctx);
            }
        }
        Ok(())
    }
}

pub struct MenuInitData {}

fn new_ui_text_style(sk: &StereoKit, font_size: f32) -> TextStyle {
    TextStyle::new(sk, Font::default(sk), font_size, Color128::new(Rgb::new(1.0, 1.0, 1.0), 1.0))
}

enum MainMenuStageType {
    Main,
    Settings,
    Credits,
}

struct MainMenuStage(MainMenuStageType);

impl SkStage<MainMenuStageType> for MainMenuStage {
    fn new(stage_type: MainMenuStageType) -> Self {
        Self(stage_type)
    }

    fn get(&self) -> &MainMenuStageType {
        &self.0
    }

    fn set(&mut self, stage_type: MainMenuStageType) {
        self.0 = stage_type
    }
}