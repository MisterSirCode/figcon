use std::{env, path::Path};
use figcon::FigCon;
use serde_json::{to_value};

fn main() {
    // Example setup using config.json stored adjacent to the program binary
    let mut conf = FigCon::load_or_default(
        Path::join(env::current_dir().unwrap().as_path(), "config.json")
    );

    // Set keys in the config
    conf.set_st("Static String Key", to_value("Static Value Type").unwrap());
    conf.set_str_st("Another String", "Simple Static String Value");
    conf.set("Dynamic String Key".to_owned(), to_value(1234).unwrap());

    // Get keys in the config
    conf.get_st("Static String Key");
    // Outputs: Value::String("Static Value Type")
    conf.get("Dynamic String Key".to_owned());
    // Outputs: Value::Number(1234)

    // Delete keys in the config
    conf.del_st("Another String");
    // Config no longer stores that key/value pair

    // Synchronous write to file
    conf.save();
}