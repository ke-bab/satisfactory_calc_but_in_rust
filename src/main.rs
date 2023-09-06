use std::any::Any;

mod game_data;
mod planner;

fn main() {
    let classes = game_data::extract("recipes.json");

    let iron_rod = classes.iter().find(|cl| cl.contains_product("IronRod") && !cl.is_alternate);
    if iron_rod.is_some() {
        println!("{:?}", iron_rod.unwrap());
    }

}

