use stereokit::high_level::quat_from_angles;
use stereokit::lifecycle::DrawContext;
use stereokit::pose::Pose;
use stereokit::StereoKit;
use crate::misc_traits::{GameState, SKLoop};

struct PopupMenu {
    pose: Pose
}

// impl SKLoop<PopupState> for PopupMenu {
//     fn create(sk: &StereoKit) -> anyhow::Result<Self> where Self: Sized {
//         Ok(Self{pose: Pose::new([0.0, 0.0, 0.2].into(), quat_from_angles(0.0, 180.0, 0.0).into())})
//     }
//
//     fn tick(&mut self, sk: &StereoKit, ctx: &DrawContext) -> anyhow::Result<Option<PopupState>> {
//
//     }
// }

pub enum PopupState {
    On,
    Off
}