use std::ops::{Deref, DerefMut};
use rand::prelude::SliceRandom;
use rand::{thread_rng};

#[derive(Debug, Clone)]
pub struct Term(pub String);
#[derive(Debug, Clone)]
pub struct Definition(pub String);
#[derive(Debug, Clone)]
pub struct Card {
    pub(crate) term: Term,
    pub(crate) definition: Definition
}
pub struct Deck(pub Vec<Card>);

impl Deref for Deck {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Deck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deck {
    fn choose_num(&self, num: usize) -> Vec<Card> {
        let deck = &self.0;
        deck.choose_multiple(&mut thread_rng(), num).map(|my_card| {
            my_card.clone()
        }).collect()
    }
}
pub trait IntoRand<T> {
    fn into_rand(self) -> T;
}

impl IntoRand<Option<(Card, Card)>> for Deck {
    fn into_rand(self) -> Option<(Card, Card)> {
        let things = self.choose_num(2);
        Some((things.get(0)?.clone(), things.get(1)?.clone()))
    }
}
impl IntoRand<Option<(Card, Card, Card)>> for Deck {
    fn into_rand(self) -> Option<(Card, Card, Card)> {
        let things = self.choose_num(3);
        Some((things.get(0)?.clone(),
              things.get(1)?.clone(),
              things.get(2)?.clone(),
        ))
    }
}
impl IntoRand<Option<(Card, Card, Card, Card)>> for Deck {
    fn into_rand(self) -> Option<(Card, Card, Card, Card)> {
        let things = self.choose_num(4);
        Some((things.get(0)?.clone(),
              things.get(1)?.clone(),
              things.get(2)?.clone(),
              things.get(3)?.clone(),
        ))
    }
}
impl IntoRand<Option<(Card, Card, Card, Card, Card)>> for Deck {
    fn into_rand(self) -> Option<(Card, Card, Card, Card, Card)> {
        let things = self.choose_num(5);
        Some((things.get(0)?.clone(),
              things.get(1)?.clone(),
              things.get(2)?.clone(),
              things.get(3)?.clone(),
              things.get(4)?.clone(),
        ))
    }
}
