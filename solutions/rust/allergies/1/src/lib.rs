use strum::{EnumIter, IntoEnumIterator};

pub struct Allergies(Vec<Allergen>);

#[derive(Debug, PartialEq, Eq, Clone, EnumIter)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(
            Allergen::iter()
                .enumerate()
                .filter(|(pos, _)| score & 2_u32.pow(*pos as u32) != 0)
                .map(|(_, allergen)| allergen)
                .collect(),
        )
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.0.to_vec()
    }
}
