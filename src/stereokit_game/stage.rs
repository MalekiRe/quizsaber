pub trait SkStage<StageType> {
    fn new(stage_type: StageType) -> Self;
    fn get(&self) -> &StageType;
    fn set(&mut self, stage_type: StageType);
}