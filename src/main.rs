use std::{env, path::Path};
use figcon::{FigCon, ValueExtensions};
use serde_json::json;

fn main() {
    // Example setup using config.json stored adjacent to the program binary
    let mut conf = FigCon::load_or_default(
        Path::join(env::current_dir().unwrap().as_path(), "config.json")
    );

    // Set keys in the config
    conf.set_key_st("Static String Key", json!("Static Value Type"));
    conf.set_key("Dynamic String Key".to_owned(), json!(1234));

    // Get keys in the config
    conf.get_key_st("Static String Key");
    // Outputs: Value::String("Static Value Type")
    conf.get_key("Dynamic String Key".to_owned());
    // Outputs: Value::Number(1234)

    // Check if a key exists
    conf.has_key_st("Another String");
    // Ouputs: true

    // Delete keys in the config
    conf.remove_key_st("Another String");
    // Config no longer stores that key/value pair

    // Check if a key exists after a change
    conf.has_key_st("Another String");
    // Ouputs: false

    // Create a new child in the main
    let parent = conf.new_obj_st("Subtree One"); // Live config new objects are always valid. No option handling
    let child = parent.new_obj_st("child").unwrap();
    child.set_key_st("Child's parameter", json!(1234));
    let subchild = child.new_obj_st("Child's Subtree").unwrap();
    subchild.set_key_st("Subchild's parameter", json!("hello world!"));

    // Acquire a reference to a subtree
    let nsub = conf.get_obj_st("Subtree One").unwrap();
    // Outputs: <Child's Subtree> (Value)
    nsub.has_key_st("Child's parameter");
    // Outputs: true

    // Synchronous write to file
    conf.save();
}