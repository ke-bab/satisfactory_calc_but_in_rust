struct Recipe {
    name: String,
    connected_recipe: Option<Box<* const Recipe>>,
}

fn main() {
    let r1 = Recipe{name: "wire".to_string(), connected_recipe: None};
    let r2 = Recipe{name: "ingot".to_string(), connected_recipe: Some(Box::new(&r1))};

    if r2.connected_recipe.is_some() {
        println!("we have recipe in recipe: {}", r2.name);
    }

    println!("will this work? {}", r1.name);
}
