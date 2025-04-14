pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
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
        Self { score: score & 255 }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_score = match allergen {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        };

        self.score & allergen_score != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut result = Vec::new();

        if self.score & 1 != 0 {
            result.push(Allergen::Eggs);
        }
        if self.score & 2 != 0 {
            result.push(Allergen::Peanuts);
        }
        if self.score & 4 != 0 {
            result.push(Allergen::Shellfish);
        }
        if self.score & 8 != 0 {
            result.push(Allergen::Strawberries);
        }
        if self.score & 16 != 0 {
            result.push(Allergen::Tomatoes);
        }
        if self.score & 32 != 0 {
            result.push(Allergen::Chocolate);
        }
        if self.score & 64 != 0 {
            result.push(Allergen::Pollen);
        }
        if self.score & 128 != 0 {
            result.push(Allergen::Cats);
        }

        result
    }
}
