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
use stereokit::text::TextStyle;
use stereokit::ui::{MoveType, WindowType};
use stereokit::values::Color128;
use crate::main_menu::credits_menu::CreditsMenu;
use crate::main_menu::main_menu::MainMenu;
use crate::main_menu::settings_menu::SettingsMenu;
use crate::quiz_saber_stage::{QuizSaberStage, QuizSaberStageType};
use crate::saber_game_loop::SaberOffsets;
use crate::stereokit_game::sk_loop::SkGameLoop;

pub struct MainMenuWindow {
    pose: Pose,
    main_menu_stage: MainMenuStage,
    settings_menu: SettingsMenu,
    main_menu: MainMenu,
    credits_menu: CreditsMenu,
}

impl SkGameLoop<(), (&mut QuizSaberStage, &mut SaberOffsets)> for MainMenuWindow {
    fn init(sk: &StereoKit, init_data: ()) -> anyhow::Result<Self> where Self: Sized {
        Ok(Self{
            pose: Pose::new([0.0, 0.0, -0.4].into(), quat_from_angles(0.0, 180.0, 0.0).into()),
            main_menu_stage: MainMenuStage::new(MainMenuStageType::Main),
            settings_menu: SettingsMenu::init(sk, ())?,
            main_menu: MainMenu::init(sk, ())?,
            credits_menu: CreditsMenu::init(sk, ())?,
        })
    }

    fn tick(&mut self, sk: &StereoKit, ctx: &DrawContext, run_data: (&mut QuizSaberStage, &mut SaberOffsets)) -> anyhow::Result<()> {
        let (quiz_saber_stage, saber_offsets) = run_data;
        ui::window::try_window(ctx, "QuizSaber", &mut self.pose, [0.7, 0.7].into(), WindowType::WindowBody, MoveType::MoveNone, |ui| -> anyhow::Result<()> {
            match self.main_menu_stage.deref() {
                MainMenuStageType::Main => self.main_menu.tick(sk, ctx, (&mut self.main_menu_stage, quiz_saber_stage, ui))?,
                MainMenuStageType::Settings => self.settings_menu.tick(sk, ctx, (ui, &mut self.main_menu_stage, saber_offsets))?,
                MainMenuStageType::Credits => self.credits_menu.tick(sk, ctx, (ui, &mut self.main_menu_stage))?,
            }
            Ok(())
        })?;
        Ok(())
    }
}

fn new_ui_text_style(sk: &StereoKit, font_size: f32) -> TextStyle {
    TextStyle::new(sk, Font::default(sk), font_size, Color128::new(Rgb::new(1.0, 1.0, 1.0), 1.0))
}

enum MainMenuStageType {
    Main,
    Settings,
    Credits,
}
struct MainMenuStage(MainMenuStageType);
impl MainMenuStage {
    pub fn set(&mut self, other: MainMenuStageType) {
        self.0 = other;
    }
    pub fn new(other: MainMenuStageType) -> Self {
        Self(other)
    }
}
impl Deref for MainMenuStage {
    type Target = MainMenuStageType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}