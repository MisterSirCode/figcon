# figcon

A stupid-simple synchronous serde_json config file manager

I made this package for extremely basic json configuration for development or small applications, optimizing for simplicity/functionality over performance or quality. Feel free to never use this crate if you so please.

It is by no means a proper or efficient config manager, nor a database.

Key/Value pairs work fine with any `serde_json::Value` type.

If you wish to make object trees or categories, this library extends the Value implementation to allow some primitive object tree manipulation. Its manual and will panic if used on non-objects. 

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

    // Check if a key exists
    conf.has_st("Another String");
    // Ouputs: true

    // Delete keys in the config
    conf.del_st("Another String");
    // Config no longer stores that key/value pair

    // Check if a key exists after a change
    conf.has_st("Another String");
    // Ouputs: false

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