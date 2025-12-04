# figcon

A stupid-simple synchronous serde_json config file manager

I made this package for extremely basic json configuration for development or small applications, optimizing for simplicity/functionality over performance or quality. Feel free to never use this crate if you so please.

It is by no means a proper or efficient config manager, nor a database.

Key/Value pairs work fine with any `serde_json::Value` type, but because of the simple nature of this package,
you can only make single-layer configurations using the code in this crate.

If you wish to make object trees or categories, make your own Objects with `serde_json::Value` and set them with the config. You will just have to handle the Value type manually.

## Use Example

```rust
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
```

Output of the example above (./config.json)

```json
{
  "Dynamic String Key": 1234,
  "Static String Key": "Static Value Type"
}
```