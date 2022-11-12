use stereokit::high_level::quat_from_angles;
use stereokit::lifecycle::DrawContext;
use stereokit::pose::Pose;
use stereokit::StereoKit;
use crate::quiz_saber_stage::{QuizSaberStageType};

struct PopupMenu {
    pose: Pose
}


pub enum PopupState {
    On,
    Off
}