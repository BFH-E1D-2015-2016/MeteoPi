extern crate gtk;

use gtk::prelude::*;
use gtk::{Label, Window, WindowType};

pub mod backend;

mod tools;
pub use tools::Temperature;
pub use tools::TemperatureUnits;

mod provider;
pub use provider::Provider;


fn main() {

    gtk::init().expect("Failed to initialize GTK.");

    let providers = backend::backend_loader();

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Météo");
    window.maximize();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let data = providers[0].get_temperature().expect("Unable to get a temperature");
    let label_str = format!("Temp: {}.{} °C", data.digit, data.milli);

    let label = Label::new(Some(&label_str));

    window.add(&label);
    window.show_all();

    gtk::main();

}
