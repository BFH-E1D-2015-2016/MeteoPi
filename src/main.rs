#[macro_use]
extern crate log;
extern crate env_logger;

extern crate gtk;
use gtk::prelude::*;
use gtk::{Label, Window, WindowType};

pub mod backend;

mod tools;
pub use tools::Temperature;
pub use tools::TemperatureUnits;

mod provider;
pub use provider::Provider;
pub use provider::ProviderLoader;


fn main() {

    // Init the log system
    env_logger::init().expect("Failed to initialize the log system");
    info!("Application started");

    // Init the graphical stack
    gtk::init().expect("Failed to initialize GTK.");
    info!("Gui backend initialized");

    let providers = backend::backend_loader();
    info!("All backends loaded");

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
    info!("Application is shutting down");

}
