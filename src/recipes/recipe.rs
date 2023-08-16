use crate::ingredients::Ingredient;

pub struct Recipe {
    pub res: String,
    pub multiplier: f32,
    pub input_ingredients: Vec<Ingredient>,
}

impl Recipe {
    pub fn new(
        res: String,
        multi: Option<f32>,
        ingredients: Option<Vec<Ingredient>>,
    ) -> Recipe {
        Recipe {
            res,
            multiplier: multi.unwrap_or(1.0),
            input_ingredients: ingredients.unwrap_or(Vec::new()),
        }
    }
}

