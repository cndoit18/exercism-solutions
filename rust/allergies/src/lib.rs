use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Allergies(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0 & (*allergen as u32) == (*allergen as u32)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies = Vec::new();
        for allergen in Allergen::iter() {
            if self.is_allergic_to(&allergen) {
                allergies.push(allergen);
            }
        }
        allergies
    }
}
