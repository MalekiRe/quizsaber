use stereokit::lifecycle::DrawContext;
use stereokit::StereoKit;
pub trait SkDrawMut {
    fn draw_mut(&mut self, sk: &StereoKit, ctx: &DrawContext);
}
pub trait SkDraw {
    fn draw(&self, sk: &StereoKit, ctx: &DrawContext);
}