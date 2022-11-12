use std::ops::Deref;
use glam::Mat4;
use prisma::Rgb;
use stereokit::font::Font;
use stereokit::lifecycle::DrawContext;
use stereokit::{StereoKit, ui};
use stereokit::high_level::quat_from_angles;
use stereokit::pose::Pose;
use stereokit::text::TextStyle;
use stereokit::ui::{MoveType, WindowType};
use stereokit::values::Color128;
use crate::main_menu::credits_menu::CreditsMenu;
use crate::main_menu::main_menu::MainMenu;
use crate::main_menu::settings_menu::SettingsMenu;
use crate::misc_traits::{GameState, SKLoop, UiLoop};

mod main_menu;
mod settings_menu;
mod credits_menu;

pub struct MainMenuWindow {
    main_menu_state: MainMenuState,
    settings_menu: SettingsMenu,
    main_menu: MainMenu,
    credits_menu: CreditsMenu,
    pose: Pose,
}
impl MainMenuWindow {
    pub fn set_settings_offset_matrix(&mut self, matrix: Mat4) {
        self.settings_menu.offset_matrix = Some(matrix);
    }
    pub fn get_settings_offset_matrix(&self) -> Option<Mat4> {
        self.settings_menu.offset_matrix
    }
}
impl SKLoop<GameState> for MainMenuWindow {
    fn create(sk: &StereoKit) -> anyhow::Result<Self> where Self: Sized {
        Ok(Self{
            main_menu_state: MainMenuState::MainState,
            settings_menu: SettingsMenu::create(&sk)?,
            main_menu: MainMenu::create(&sk)?,
            credits_menu: CreditsMenu::create(&sk)?,
            pose: Pose::new([0.0, 0.0, -0.4].into(), quat_from_angles(0.0, 180.0, 0.0).into())
        })
    }

    fn tick(&mut self, sk: &StereoKit, ctx: &DrawContext) -> anyhow::Result<Option<GameState>> {
        let mut return_val = None;
        ui::window::window(ctx, "QuizSaber", &mut self.pose, [0.7, 0.7].into(), WindowType::WindowBody, MoveType::MoveNone, |ui| {
            match &self.main_menu_state {
                MainMenuState::MainState => {
                    match self.main_menu.tick(sk, ctx, ui).unwrap() {
                        Some(state) => self.main_menu_state = state,
                        _ => ()
                    }
                }
                MainMenuState::Credits => {
                    match self.credits_menu.tick(sk, ctx, ui).unwrap() {
                        Some(state) => self.main_menu_state = state,
                        _ => ()
                    }
                }
                MainMenuState::Settings => {
                    match self.settings_menu.tick(sk, ctx, ui).unwrap() {
                        Some(state) => self.main_menu_state = state,
                        _ => ()
                    }
                }
                MainMenuState::GameState(game_state) => {
                    return_val = Some(*game_state);
                }
            }
        });
        Ok(return_val)
    }
}

enum MainMenuState {
    MainState,
    Credits,
    Settings,
    GameState(GameState)
}

fn new_ui_text_style(sk: &StereoKit, font_size: f32) -> TextStyle {
    TextStyle::new(sk, Font::default(sk), font_size, Color128::new(Rgb::new(1.0, 1.0, 1.0), 1.0))
}