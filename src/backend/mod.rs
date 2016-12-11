use ::Provider;
use ::ProviderLoader;

pub mod fake;
pub mod rpi;

pub fn backend_loader() -> Vec<Box<Provider>> {

    let mut vector: Vec<Box<Provider>> = vec![Box::new(fake::FakeProvider {})];


    if let Some(provider) = rpi::RpiProvider::new() {
        vector.push(provider);
    };

    vector
}