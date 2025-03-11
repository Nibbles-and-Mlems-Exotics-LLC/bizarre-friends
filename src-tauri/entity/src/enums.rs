use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum MassUnits {
    Gram = 1,
    Ounce = 28.3495,
    Pound = 453.592,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LiquidUnits {
    Liter = 1,
    FluidOunce = 0.0295735156,
    Cup = 0.236588125,
    Gallon = 3.78541,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LengthUnits {
    Meter = 1,
    Inch = 0.0254,
    Foot = 0.3048,
    Yard = 0.9144,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TemperatureUnits {
    Celsius = 1.0,
    Farenheight = 1.80,
}
