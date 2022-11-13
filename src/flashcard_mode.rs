use stereokit::lifecycle::DrawContext;
use stereokit::StereoKit;
use crate::flashcard::double_sided::DoubleSidedFlashCard;
use crate::flashcard::FlashCard;
use crate::stereokit_game::draw::SkDrawMut;
use crate::stereokit_game::sk_loop::SkGameLoop;

pub struct FlashCardMode {
    flash_card: DoubleSidedFlashCard,
}
impl SkGameLoop<(), ()> for FlashCardMode {
    fn init(sk: &StereoKit, init_data: ()) -> anyhow::Result<Self> where Self: Sized {
        Ok(Self{flash_card: DoubleSidedFlashCard::from(sk,
                                                       &FlashCard{
            front: "i am a term".to_string(), back: "and i am a definition!".to_string()
        })?})
    }

    fn tick(&mut self, sk: &StereoKit, ctx: &DrawContext, run_data: ()) -> anyhow::Result<()> {
        self.flash_card.draw_mut(sk, ctx);
        Ok(())
    }
}