pub enum TemperatureUnits {
    Fahrenheit,
    Celsius,
}

pub struct Temperature {
    pub digit: i16,
    pub milli: i16,
    pub unit: TemperatureUnits,
}