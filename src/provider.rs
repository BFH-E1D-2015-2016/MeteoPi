use ::Temperature;

pub trait Provider {
    fn get_temperature(&self) -> Option<Temperature>;
}

pub trait ProviderLoader {
    fn new() -> Option<Box<Provider>>;
}