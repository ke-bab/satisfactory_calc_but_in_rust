use crate::blueprints::blueprint::BluePrint;
use serde_json::{Result, Value};
use serde::Deserialize;
use std::fmt::Error;
use std::fs::File;

#[derive(Deserialize)]
struct Json {
    NativeClass: String,
    Classes: Vec<Class>,
}

#[derive(Deserialize)]
pub struct Class {
    ClassName: String,
    pub FullName: String,
    mDisplayName: String,
    mIngredients: String,
    mProduct: String,
    mManufacturingMenuPriority: String,
    mManufactoringDuration: String,
    mManualManufacturingMultiplier: String,
    mProducedIn: String,
    mRelevantEvents: String,
    mVariablePowerConsumptionConstant: String,
    mVariablePowerConsumptionFactor: String,
}

pub fn extract(file: &str) -> Vec<Class> {
    let mut file = File::open(file).unwrap();
    let json: Json = serde_json::from_reader(file).unwrap();
    json.Classes

}