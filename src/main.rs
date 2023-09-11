use std::any::Any;

mod game_data;
mod planner;

fn main() {
    let classes = game_data::extract("recipes.json");

    let iron_rod = classes.iter().filter(|cl| cl.contains_product("IronRod")).collect::<Vec<_>>();

    for r in iron_rod {
        println!("recipe name: {}", r.class_name);
        println!("recipe ingredients: ");
        for i in &r.ingredients {
            println!("- {}", i.name);
        }

    }
}

