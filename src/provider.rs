use ::Temperature;

pub trait Provider {
    fn get_temperature(&self) -> Option<Temperature>;
}