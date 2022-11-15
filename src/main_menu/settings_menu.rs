use glam::Vec3;
use stereokit::high_level::model::Model;
use stereokit::lifecycle::DrawContext;
use stereokit::StereoKit;
use stereokit::text::TextAlign;
use stereokit::ui::{ConfirmMethod, WindowContext};
use crate::main_menu::{MainMenuStage, MainMenuStageType, new_ui_text_style};
use crate::quiz_saber_stage::QuizSaberStage;
use crate::saber_game_loop::SaberOffsets;
use crate::stereokit_game::sk_loop::SkGameLoop;
use crate::stereokit_game::stage::SkStage;

pub struct SettingsMenu {
    slider_val: f32,
    pub enable_sabers: bool,
    right_saber_header: CollapsableHeader,
    right_saber_pos_header: CollapsableHeader,
    right_saber_rot_header: CollapsableHeader,
    left_saber_header: CollapsableHeader,
    left_saber_pos_header: CollapsableHeader,
    left_saber_rot_header: CollapsableHeader,
}
impl SkGameLoop<(), (&WindowContext, &mut MainMenuStage, &mut SaberOffsets)> for SettingsMenu {
    fn init(sk: &StereoKit, init_data: ()) -> anyhow::Result<Self> where Self: Sized {
        Ok(Self{
            slider_val: 0.0,
            enable_sabers: false,
            right_saber_header: CollapsableHeader::from("Right Saber"),
            right_saber_pos_header: CollapsableHeader::from("Pos"),
            right_saber_rot_header: CollapsableHeader::from("Rotation"),
            left_saber_header: CollapsableHeader::from("Left Saber"),
            left_saber_pos_header: CollapsableHeader::from("Pos"),
            left_saber_rot_header: CollapsableHeader::from("Rotation"),
        })
    }

    fn tick(&mut self, sk: &StereoKit, ctx: &DrawContext, run_data: (&WindowContext, &mut MainMenuStage, &mut SaberOffsets)) -> anyhow::Result<()> {
        let (ui, quiz_saber_stage, saber_offsets) = run_data;
        let mut scale_right = 1.0;
        ui.text_style(new_ui_text_style(sk, 0.05),  |ui| {
            ui.text("Settings Menu", TextAlign::TopCenter);
            ui.text_style(new_ui_text_style(sk, 0.03),  |ui| {
                self.right_saber_header.place(ui, |ui| {
                    ui.space_sameline(0.05);
                    self.right_saber_pos_header.place(ui, |ui| {
                        slider_num(&ui, 0.1, "x", &mut saber_offsets.offset_right_hand.pos.x, -0.5, 0.5, 0.002);
                        slider_num(&ui, 0.1, "y", &mut saber_offsets.offset_right_hand.pos.y, -0.5, 0.5, 0.002);
                        slider_num(&ui, 0.1, "z", &mut saber_offsets.offset_right_hand.pos.z, -0.5, 0.5, 0.002);
                    });
                    ui.space_sameline(0.05);
                    self.right_saber_rot_header.place(ui, |ui| {
                        slider_num(&ui, 0.1, "x", &mut saber_offsets.offset_right_hand.rotation.x, -180.0, 180.0, 1.0);
                        slider_num(&ui, 0.1, "y", &mut saber_offsets.offset_right_hand.rotation.y, -180.0, 180.0, 1.0);
                        slider_num(&ui, 0.1, "z", &mut saber_offsets.offset_right_hand.rotation.z, -180.0, 180.0, 1.0);
                    });
                    slider_num(&ui, 0.05, "scale", &mut scale_right, 0.05, 4.0, 0.01);
                });
                self.left_saber_header.place(ui, |ui| {
                    ui.space_sameline(0.05);
                    self.left_saber_pos_header.place(ui, |ui| {
                        slider_num(&ui, 0.1, "x", &mut saber_offsets.offset_left_hand.pos.x, -0.5, 0.5, 0.002);
                        slider_num(&ui, 0.1, "y", &mut saber_offsets.offset_left_hand.pos.y, -0.5, 0.5, 0.002);
                        slider_num(&ui, 0.1, "z", &mut saber_offsets.offset_left_hand.pos.z, -0.5, 0.5, 0.002);
                    });
                    ui.space_sameline(0.05);
                    self.left_saber_rot_header.place(ui, |ui| {
                        slider_num(&ui, 0.1, "x", &mut saber_offsets.offset_left_hand.rotation.x, -180.0, 180.0, 1.0);
                        slider_num(&ui, 0.1, "y", &mut saber_offsets.offset_left_hand.rotation.y, -180.0, 180.0, 1.0);
                        slider_num(&ui, 0.1, "z", &mut saber_offsets.offset_left_hand.rotation.z, -180.0, 180.0, 1.0);
                    });
                    slider_num(&ui, 0.05, "scale", &mut scale_right, 0.05, 4.0, 0.01);
                });
            });
            if ui.button("Back") {
                quiz_saber_stage.set(MainMenuStageType::Main)
            }
            ui.toggle("Show Sabers", &mut self.enable_sabers);
        });
        saber_offsets.offset_right_hand.scale = Vec3::new(scale_right, scale_right, scale_right);
        saber_offsets.offset_left_hand.scale = Vec3::new(scale_right, scale_right, scale_right);
        saber_offsets.offset_left_hand.sync_matrix();
        saber_offsets.offset_right_hand.sync_matrix();
        Ok(())
    }
}

fn slider_num(ui: &WindowContext, space: f32, text: &str, val: &mut f32, min: f32, max: f32, step: f32) {
    ui.space_sameline(space);
    ui.slider(text, val, min, max, step, 0.3, ConfirmMethod::Push);
    ui.sameline();
    ui.label(val.to_string().as_str(), true);
}

struct CollapsableHeader {
    pub title: String,
    pub open: bool,
}

impl CollapsableHeader {
    pub fn from(title: &str) -> Self {
        Self::new(title, false)
    }
    pub fn new(title: &str, default_open: bool) -> Self {
        Self {
            title: title.to_string(),
            open: default_open
        }
    }
    pub fn place(&mut self, ui: &WindowContext, content_closure: impl FnOnce(&WindowContext)) {
       ui.toggle(self.title.as_str(), &mut self.open);
        if self.open {
            content_closure(ui);
        }
    }
}