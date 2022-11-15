use std::ops::Mul;
use glam::{Mat4, Vec3};
use stereokit::high_level::model::Model;
use stereokit::lifecycle::DrawContext;
use stereokit::{high_level, StereoKit};
use stereokit::high_level::collider::ColliderType;
use stereokit::high_level::math_traits::{MatrixContainer, MatrixTrait, RotationTrait};
use stereokit::input::Handed::{Left, Right};
use stereokit::shader::Shader;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use stereokit::high_level::quat_from_angles;
use stereokit::input::Handed;
use crate::{QuizSaberStageType, quiz_saber_stage};
use crate::quiz_saber_stage::{QuizSaberStage};
use crate::stereokit_game::sk_loop::SkGameLoop;

pub struct SaberStage {
    pub left_saber: Model,
    pub right_saber: Model,
}

impl SkGameLoop<(&[u8], &[u8]), (&mut QuizSaberStage, &mut SaberOffsets)> for SaberStage {
    fn init(sk: &StereoKit, init_data: (&[u8], &[u8])) -> Result<Self> where Self: Sized {
        let right_saber = init_data.0;
        let left_saber = init_data.1;
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
        saber_offsets.set_sabers_to_hands(sk, &mut self.right_saber, &mut self.left_saber);
        sk.input_hand_visible(Handed::Right, false);
        sk.input_hand_visible(Left, false);
        self.right_saber.draw(ctx);
        self.left_saber.draw(ctx);
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SaberOffsets {
    pub offset_right_hand: MatrixContainer,
    pub offset_left_hand: MatrixContainer,
}
impl SaberOffsets {
    pub fn set_sabers_to_hands(&self, sk: &StereoKit, right_saber: &mut Model, left_saber: &mut Model) {
        let right_hand = sk.input_hand(Right).palm;
        let left_hand = sk.input_hand(Left).palm;
        right_saber.set_matrix(Mat4::from(right_hand.as_matrix()).mul(self.offset_right_hand.mat4));
        left_saber.set_matrix(Mat4::from(left_hand.as_matrix()).mul(self.offset_left_hand.mat4));
    }
}
impl Default for SaberOffsets {
    fn default() -> Self {
        Self {
            offset_right_hand: MatrixContainer::new(Vec3::new(-0.45, 0.03, -0.035), Vec3::new(0.0, 0.0, 90.0), Vec3::new(1.0, 1.0, 1.0)),
            offset_left_hand: MatrixContainer::new(Vec3::new(0.45, 0.03, 0.035), Vec3::new(0.0, 0.0, -90.0), Vec3::new(1.0, 1.0, 1.0))
        }
    }
}