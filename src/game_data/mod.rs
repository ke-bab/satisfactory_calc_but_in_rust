mod parse;

use std::fs::File;
use std::str::FromStr;
use serde::Deserialize;
use parse::ResourceDesc;


#[derive(Deserialize)]
struct Json {
    #[serde(rename = "Classes")]
    classes: Vec<JsonClass>,
}

#[derive(Deserialize)]
pub struct JsonClass {
    #[serde(rename = "ClassName")]
    pub class_name: String,
    #[serde(rename = "mDisplayName")]
    pub m_display_name: String,
    #[serde(rename = "mIngredients")]
    pub m_ingredients: String,
    #[serde(rename = "mProduct")]
    pub m_product: String,
    #[serde(rename = "mManufactoringDuration")]
    pub m_manufactoring_duration: String,
}

#[derive(Debug)]
pub struct ParsedClass {
    pub class_name: String,
    pub display_name: String,
    pub ingredients: Vec<ResourceDesc>,
    pub products: Vec<ResourceDesc>,
    pub manufactoring_duration: i32,
    pub is_alternate: bool,
}

impl ParsedClass {
    // pub fn amount_per_min(&self) -> f32 {
    //     60 / self.manufactoring_duration * self.
    // }

    pub fn contains_product(&self, string: &str) -> bool {
        for product in &self.products {
            if product.name.eq(string) {
                return true;
            }
        }
        false
    }
}

pub fn extract(file: &str) -> Vec<ParsedClass> {
    let file = File::open(file).unwrap();
    let json: Json = serde_json::from_reader(file).unwrap();
    let mut parsed_vec: Vec<ParsedClass> = Vec::new();
    for class in &json.classes {
        let ingred_desc = parse::parse_descriptions(&class.m_ingredients);
        let prod_desc = parse::parse_descriptions(&class.m_product);
        let float_duration: f32 = class.m_manufactoring_duration.parse().unwrap();
        let parsed = ParsedClass {
            class_name: parse::strip_class_name(&class.class_name).to_string(),
            display_name: class.m_display_name.to_string(),
            ingredients: ingred_desc,
            products: prod_desc,
            manufactoring_duration: float_duration as i32,
            is_alternate: class.class_name.contains("Alternate"),
        };
        parsed_vec.push(parsed);
    }
    parsed_vec
    // remember to exclude recursive recipes.
    // to check rule 2 we need go up and check is there any blueprint already with same resource as "result resource"
}

