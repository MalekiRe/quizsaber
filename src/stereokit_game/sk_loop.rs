use stereokit::lifecycle::DrawContext;
use stereokit::StereoKit;
use anyhow::Result;

trait SkGameLoop<InitData, RunData> {
    fn init(sk: &StereoKit, init_data: InitData) -> Result<Self> where Self: Sized;
    fn tick(&self, sk: &StereoKit, ctx: &DrawContext, run_data: RunData) -> Result<()>;
}
