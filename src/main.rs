#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate slog;
extern crate slog_term;

use slog::DrainExt;

extern crate gtk;
use gtk::prelude::*;
use gtk::{Label, Window, WindowType};

extern crate regex;

extern crate rustc_serialize;
extern crate docopt;

pub mod backend;

mod tools;
pub use tools::Temperature;
pub use tools::TemperatureUnits;

mod provider;
pub use provider::Provider;
pub use provider::ProviderLoader;

pub mod plugins;

// Log system is initialized before main
lazy_static! {
    pub static ref MAIN_LOGGER: slog::Logger = {
        let drain = slog_term::streamer().full().build().fuse();
        slog::Logger::root(drain, o!("place" => move |info: &slog::Record| {
            format!("{}:{} {}", info.file(), info.line(), info.module())
        }))
    };
}

fn main() {
    info!(MAIN_LOGGER, "Application started");

    // Init the graphical stack
    gtk::init().expect("Failed to initialize GTK.");
    info!(MAIN_LOGGER, "Gui backend initialized");

    let providers = backend::backend_loader();
    info!(MAIN_LOGGER, "All backends loaded");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Météo");
    window.maximize();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let data = providers.last().unwrap().get_temperature().expect("Unable to get a temperature");
    let label_str = format!("Temp: {}.{} °C", data.digit, data.milli);

    let label = Label::new(Some(&label_str));

    window.add(&label);
    window.show_all();

    gtk::main();
    info!(MAIN_LOGGER, "Application is shutting down");

}
