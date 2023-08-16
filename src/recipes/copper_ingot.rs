use super::recipe::Recipe;

pub struct CopperIngot;

impl CopperIngot {
    pub fn new() -> Recipe {
        Recipe::new("copper_ingot".to_string(), None, None)
    }
}