mod game_data;

fn main() {
    let classes = game_data::extract(&"recipes.json".to_string());

    let iron_rod = classes.iter().find(|x| x.class_name.contains("IronRod"));
    if iron_rod.is_some() {
        println!("{:?}", iron_rod.unwrap());
        println!("{}", "пашет");
    }
}

