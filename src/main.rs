mod game_data;

fn main() {
    let classes = game_data::extract(&"recipes.json".to_string());

    let iron_rod = classes.iter().find(|cl| cl.contains_product("IronRod") && !cl.is_alternate);
    if iron_rod.is_some() {
        println!("{:?}", iron_rod.unwrap());
    }
}

