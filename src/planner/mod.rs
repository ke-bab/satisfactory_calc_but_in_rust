use crate::game_data::Recipe;

struct Factory<'a> {
    multiplier: f32,
    ingredients: Vec<Ingredient<'a>>,
    products: Vec<Product>,
}

pub struct Ingredient<'a> {
    pub resource: String,
    pub amount: i32,
    pub connected_recipe: Option<&'a Recipe>,
}

struct Product {}
