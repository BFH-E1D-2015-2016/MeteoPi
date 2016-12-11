use ::Provider;

pub mod fake;


pub fn backend_loader() -> Vec<Box<Provider>> {
    vec![Box::new(fake::FakeProvider {})]
}