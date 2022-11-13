pub mod double_sided;

#[derive(Clone, Debug, Default)]
pub struct FlashCard {
    pub(crate) front: String,
    pub(crate) back: String,
}

//TODO use this instead at some point
struct AnkiSide {
    image: Option<()>,
    sound: Option<()>,
    text: Option<String>
}
//flash cards can have