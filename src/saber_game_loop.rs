use std::ops::Mul;
use glam::{Mat4, Vec3};
use stereokit::high_level::model::Model;
use stereokit::lifecycle::DrawContext;
use stereokit::{high_level, StereoKit};
use stereokit::high_level::collider::ColliderType;
use stereokit::high_level::math_traits::{MatrixTrait, RotationTrait};
use stereokit::input::Handed::{Left, Right};
use stereokit::shader::Shader;
use anyhow::Result;
use stereokit::high_level::quat_from_angles;
use stereokit::input::Handed;
use crate::{QuizSaberStageType, quiz_saber_stage};
use crate::quiz_saber_stage::{QuizSaberStage};
use crate::stereokit_game::sk_loop::SkGameLoop;

pub struct FlashCardSaberStage {
    left_saber: Model,
    right_saber: Model,
}

impl SkGameLoop<(), (&mut QuizSaberStage, &mut SaberOffsets)> for FlashCardSaberStage {
    fn init(sk: &StereoKit, init_data: ()) -> Result<Self> where Self: Sized {
        let right_saber = include_bytes!("../assets/saber1.glb");
        let left_saber = include_bytes!("../assets/saber2.glb");
        let mut right_saber = Model::from_memory(&sk, "right_saber.glb", right_saber, Some(&Shader::p_b_r(&sk)))?;
        let mut left_saber = Model::from_memory(&sk, "left_saber.glb", left_saber, Some(&Shader::default(&sk)))?;

        right_saber.set_collider(&sk, ColliderType::CapsuleCollider);
        left_saber.set_collider(&sk, ColliderType::CapsuleCollider);
        Ok(Self{
            left_saber,
            right_saber
        })
    }

    fn tick(&mut self, sk: &StereoKit, ctx: &DrawContext, run_data: (&mut QuizSaberStage, &mut SaberOffsets)) -> Result<()> {
        let (game_state, saber_offsets) = run_data;
        let right_hand = sk.input_hand(Right).palm;
        let left_hand = sk.input_hand(Left).palm;
        self.right_saber.set_matrix(Mat4::from(right_hand.as_matrix()).mul(saber_offsets.offset_right_hand));
        self.left_saber.set_matrix(Mat4::from(left_hand.as_matrix()).mul(saber_offsets.offset_left_hand));
        sk.input_hand_visible(Handed::Right, false);
        sk.input_hand_visible(Left, false);
        self.right_saber.draw(ctx);
        self.left_saber.draw(ctx);
        Ok(())
    }
}

pub struct SaberOffsets {
    pub offset_right_hand: Mat4,
    pub offset_left_hand: Mat4,
}
impl Default for SaberOffsets {
    fn default() -> Self {
        Self {
            offset_right_hand: Mat4::from_rotation_translation(quat_from_angles(0.0, 0.0, 90.0), Vec3::new(-0.45, 0.03, -0.035)),
            offset_left_hand: Mat4::from_rotation_translation(quat_from_angles(0.0, 0.0, -90.0), [0.45, 0.03, 0.035].into()),
        }
    }
}