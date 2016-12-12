use ::Provider;
use ::ProviderLoader;

use ::Temperature;
use ::TemperatureUnits;

use std::env;
use std::path::PathBuf;
use std::fs;

use std::process::Command;
use regex::Regex;

pub struct RpiProvider;

impl Provider for RpiProvider {
    fn get_temperature(&self) -> Option<Temperature> {
        let logger = ::MAIN_LOGGER.new(o!("cmd" => "vcgencmd"));
        trace!(logger,
               "RpiProvider.get_temperature: Calling `vcgencmd measure_temp`");
        if let Ok(raw) = Command::new("vcgencmd")
            .args(&["measure_temp"])
            .output() {

            if raw.status.success() {
                trace!(logger,
                       "RpiProvider.get_temperature: `vcgencmd` exited sucessfully");

                // Example of output : "temp=56.9'C\n" or "temp=56.0'C\n"
                let re = Regex::new(r"^temp=(\d+)\.(\d)'C").unwrap();
                let output = ::std::str::from_utf8(&raw.stdout).unwrap();
                if let Some(cap) = re.captures(output) {
                    trace!(logger, "RpiProvider.get_temperature: Parsing `{}`", output);
                    return Some(Temperature {
                        digit: cap.at(1).unwrap().parse::<i16>().unwrap(),
                        milli: cap.at(2).unwrap().parse::<u16>().unwrap() * 100,
                        unit: TemperatureUnits::Celsius,
                    });

                }
            }
        }
        None
    }
}

impl ProviderLoader for RpiProvider {
    fn new() -> Option<Box<Provider>> {

        let logger = ::MAIN_LOGGER.new(o!("cmd" => "vcgencmd"));

        // This provider only works when vcgencmd is present on the PATH variable
        match env::var_os("PATH") {
            Some(paths) => {
                for path in env::split_paths(&paths) {

                    trace!(logger, "RpiProvider: Looking for `vcgencmd` in {:?}", &path);

                    let mut file = PathBuf::from(path);
                    file.push("vcgencmd");
                    match fs::metadata(file.as_path()) {
                        Ok(meta) => {
                            if meta.is_file() {
                                info!(logger,
                                      "RpiProvider: Founded `vcgencmd` in {:?}",
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

        info!(logger, "RpiProvider: `vcgencmd was not founded in $PATH");
        None



    }
}