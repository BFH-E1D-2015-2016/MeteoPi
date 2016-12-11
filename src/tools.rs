pub enum TemperatureUnits {
    Fahrenheit,
    Celsius,
}

pub struct Temperature {
    pub digit: i16,
    pub milli: u16,
    pub unit: TemperatureUnits,
}