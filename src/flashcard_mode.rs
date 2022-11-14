use std::ops::RangeFrom;
use anyhow::Context;
use rand::{Rng, thread_rng};
use rand::prelude::SliceRandom;
use spaced_rs::{schedule, UserReview};
use stereokit::lifecycle::DrawContext;
use stereokit::StereoKit;
use crate::flashcard::double_sided::DoubleSidedFlashCard;
use crate::flashcard::FlashCard;
use crate::flashcard_system::Guess;
use crate::flashcard_system::Guess::{Correct, Incorrect};
use crate::parser::FlashCardData;
use crate::stereokit_game::draw::SkDrawMut;
use crate::stereokit_game::sk_loop::SkGameLoop;

pub struct FlashCardMode {
    flash_card: DoubleSidedFlashCard,
    current_card_index: usize,
}
impl SkGameLoop<(), (&Vec<FlashCardData>, usize, &mut Option<Guess>)> for FlashCardMode {
    fn init(sk: &StereoKit, init_data: ()) -> anyhow::Result<Self> where Self: Sized {
        Ok(Self{flash_card: DoubleSidedFlashCard::from(sk,
                                                       &FlashCard{
            front: "i am a term".to_string(), back: "and i am a definition!".to_string()
        })?,
            current_card_index: 0
        })
    }

    fn tick(&mut self, sk: &StereoKit, ctx: &DrawContext, run_data: (&Vec<FlashCardData>, usize, &mut Option<Guess>)) -> anyhow::Result<()> {
        let (cards, index, guess) = run_data;
        let current_flashcard = cards.get(index).context("Missing")?;
        self.flash_card.set_text(&current_flashcard.front, &current_flashcard.back);
        let mut button_press = None;
        self.flash_card.draw_mut(sk, ctx, &mut button_press);
        if let Some(button_press) = button_press {
            guess.replace(match button_press {
                FlashCardButtonPress::Correct => Correct,
                FlashCardButtonPress::Incorrect => Incorrect,
            });
        }
        Ok(())
    }
}

pub enum FlashCardButtonPress {
    Correct,
    Incorrect,
}