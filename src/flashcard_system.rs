use anyhow::Context;
use rand::thread_rng;
use random_number::random;
use spaced_rs::{schedule, UserReview};
use stereokit::lifecycle::DrawContext;
use stereokit::StereoKit;
use crate::flashcard_mode::FlashCardButtonPress;
use crate::IndexWrapper;
use crate::parser::{FlashCardData, parse, TEST_FILE};
use crate::stereokit_game::sk_loop::SkGameLoop;

pub struct FlashCardSystem {
}
impl SkGameLoop<(), (&mut Vec<FlashCardData>, &mut Option<Guess>, &mut IndexWrapper)> for FlashCardSystem {
    fn init(sk: &StereoKit, init_data: ()) -> anyhow::Result<Self> where Self: Sized {
        Ok(Self{})
    }

    fn tick(&mut self, sk: &StereoKit, ctx: &DrawContext, run_data: (&mut Vec<FlashCardData>, &mut Option<Guess>, &mut IndexWrapper)) -> anyhow::Result<()> {
        let (cards, possible_guess, card_index) = run_data;
        let current_flashcard = cards.get_mut(card_index.0).context("Missing")?;
        if let Some(guess) = possible_guess {
            let user_review = match guess {
                Guess::Correct => {
                    UserReview::TooEasy
                }
                Guess::Incorrect => {
                    UserReview::TooHard
                }
            };
            current_flashcard.sched_data = schedule(current_flashcard.sched_data.clone(), user_review, Default::default(), 0.99);
            card_index.0 = rand::seq::index::sample_weighted(&mut thread_rng(), cards.len(), |index| {
                1.0/(cards.get(index).unwrap().sched_data.memory_strength)
            }, 4)?.index(random!(0..3));
        }
        Ok(())
    }
}

pub enum Guess {
    Correct,
    Incorrect
}