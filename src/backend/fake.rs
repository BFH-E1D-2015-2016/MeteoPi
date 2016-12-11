use ::Provider;
use ::Temperature;
use ::TemperatureUnits;


pub struct FakeProvider;

impl Provider for FakeProvider {
    fn get_temperature(&self) -> Option<Temperature> {
        Some(Temperature {
            digit: 23,
            milli: 500,
            unit: TemperatureUnits::Celsius,
        })
    }
}