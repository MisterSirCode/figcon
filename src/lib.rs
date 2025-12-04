use serde_json::Value;
use std::{
    fmt::Display, 
    fs::File, 
    io::{
        BufWriter, 
        Read
    }, 
    path::PathBuf
};

#[derive(Clone, Debug)]
/// Configurator
/// 
/// A simple synchronous config manager that relies on serde_json
///
/// It stores its own path location and can be saved/reloaded at any time
pub struct Conf {
    live_config: Value,
    location: PathBuf
}

impl Display for Conf {
    /// A potentially slow function which attempts to display the entire configuration as a prettified json string.
    /// 
    /// It would be inadvisable to use this on larger configurations during runtime.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self.live_config).unwrap())
    }
}

impl Conf {
    /// Initialize the configurator
    /// 
    /// Attempts to load a config file with the given PathBuf
    /// and returns an empty configurator when it fails
    pub fn load_or_default(path: PathBuf) -> Self {
        if path.exists() && let Ok(file) = File::open(&path) {
            let mut buffer: String = Default::default();
            (&file).read_to_string(&mut buffer).expect("Failed to read config from storage");
            let json: Value = serde_json::from_str(&buffer).expect("JSON deserialization failed");
            Conf { live_config: json, location: path }
        } else {
            Conf { live_config: serde_json::Value::Object(Default::default()), location: path }
        }
    }

    /// Set Config Path
    /// 
    /// Changing the location during runtime will not affect the live config, and it will not save to the new location automatically.
    /// 
    /// Use `.save()` immediately after if you wish to write the live config to the new location
    pub fn set_path(&mut self, path: PathBuf) {
        self.location = path;
    }

    /// Reload Config
    /// 
    /// Pull the config file again and overwrite the config in memory
    pub fn reload(&mut self) -> Self {
        Self::load_or_default(self.location.clone())
    }

    /// Save Config
    /// 
    /// Write the current config state synchronously to the file system
    pub fn save(&self) {
        let file = File::create(&self.location).expect("Failed to create config file"); // this works regardless of if file exists or not
        let file = BufWriter::new(file); // this makes it orders of magnitude faser
        serde_json::to_writer_pretty(file, &self.live_config).expect("Config JSON serialization / writeout failed");
    }

    /// Get 
    /// 
    /// Get a serde_json Value with a specified key
    pub fn get(&self, key: String) -> Value {
        self.live_config[key].clone()
    }

    /// Set
    /// 
    /// Set a value with a specified key and serde_json Value
    pub fn set(&mut self, key: String, val: Value) {
        self.live_config[key] = val;
    }

    /// Get (Static)
    /// 
    /// Get a serde_json Value with a specified key
    pub fn get_st(&self, key: &str) -> Value {
        self.live_config[key.to_owned()].clone()
    }

    /// Set (Static)
    /// 
    /// Set a value with a specified key and serde_json Value
    pub fn set_st(&mut self, key: &str, val: Value) {
        self.live_config[key.to_owned()] = val;
    }

    /// Set String (Static)
    /// 
    /// Set a value with a specified key and static string
    pub fn set_str_st(&mut self, key: &str, val: &str) {
        self.live_config[key.to_owned()] = Value::String(val.to_owned());
    }

    /// Delete 
    /// 
    /// Removes an entry with a specified key. Returns an option with the deleted Value (if it exists)
    pub fn del(&mut self, key: String) -> Option<Value> {
        self.live_config.as_object_mut().unwrap().remove(&key)
    }

    /// Delete (Static)
    /// 
    /// Removes an entry with a specified key. Returns an option with the deleted Value (if it exists)
    pub fn del_st(&mut self, key: &str) -> Option<Value> {
        self.live_config.as_object_mut().unwrap().remove(key)
    }
}