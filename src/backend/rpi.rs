use ::Provider;
use ::ProviderLoader;

use ::Temperature;
use ::TemperatureUnits;

use std::env;
use std::path::PathBuf;
use std::fs;


pub struct RpiProvider;

impl Provider for RpiProvider {
    fn get_temperature(&self) -> Option<Temperature> {
        Some(Temperature {
            digit: 23,
            milli: 500,
            unit: TemperatureUnits::Celsius,
        })
    }
}

impl ProviderLoader for RpiProvider {
    fn new() -> Option<Box<Provider>> {

        // This provider only works when vcgencmd is present on the PATH variable
        match env::var_os("PATH") {
            Some(paths) => {
                for path in env::split_paths(&paths) {

                    trace!("RpiProvider: Looking for `vcgencmd` in {:?}", &path);

                    let mut file = PathBuf::from(path);
                    file.push("vcgencmd");
                    match fs::metadata(file.as_path()) {
                        Ok(meta) => {
                            if meta.is_file() {
                                info!("RpiProvider: Founded `vcgencmd` in {:?}",
                                      &file.to_path_buf());
                                return Some(Box::new(RpiProvider));
                            }
                        }
                        Err(_) => continue,
                    }
                }
            }
            None => {}
        };

        info!("RpiProvider: `vcgencmd was not founded in $PATH");
        None



    }
}