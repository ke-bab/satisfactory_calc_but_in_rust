use recipes::copper_ingot::CopperIngot;

mod recipes;
mod ingredients;

fn main() {
    let recipe = CopperIngot::new();
    println!("{}", recipe.res)
}
