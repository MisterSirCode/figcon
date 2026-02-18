use serde_json::{Map, Value, json};
use std::{
    fmt::Display, 
    fs::File, 
    io::{
        BufWriter, 
        Read
    }, 
    path::PathBuf
};

trait ValueExtensions {
    fn obj(&self) -> &Map<String, Value>;
    fn obj_mut(&mut self) -> &mut Map<String, Value>;
    fn set_by_key(&mut self, key: &str, value: Value);
    fn get_by_key(&self, key: &str) -> Value;
    fn remove_get_key(&mut self, key: &str) -> Option<Value>;
    fn remove_key(&mut self, key: &str);
    fn new_obj() -> Value;
}

impl ValueExtensions for Value {
    /// Return an immutable reference to an object within a value
    fn obj(&self) -> &Map<String, Value> {
        self.as_object().expect("Cannot convert a non-object to an object")
    }

    /// Return a mutable reference to an object
    fn obj_mut(&mut self) -> &mut Map<String, Value> {
        self.as_object_mut().expect("Cannot convert a non-object to an object")
    }

    /// Assign a key's value within an object within a value
    fn set_by_key(&mut self, key: &str, value: Value) {
        let object = self.obj_mut();
        if object.contains_key(key) {
            object[key] = value;
        } else {
            object.insert(key.to_owned(), value);
        }
    }

    /// Acquire a key's value within an object within a value
    fn get_by_key(&self, key: &str) -> Value {
        let object = self.obj();
        if object.contains_key(key) {
            object[key].clone()
        } else {
            Value::default()
        }
    }

    /// Remove an object's key within a value and return it if it exists
    fn remove_get_key(&mut self, key: &str) -> Option<Value> {
        self.obj_mut().remove(key)
    }

    /// Remove an object's key within a value without respect to whether its assigned, or where its value goes
    fn remove_key(&mut self, key: &str) {
        let object = self.obj_mut();
        if object.contains_key(key) {
            object.remove(key);
        } else {
            // Do nothing - Key is already gone
        }
    }

    /// Create a new object structure with no contents
    fn new_obj() -> Value {
        json!({})
    }
}

#[derive(Clone, Debug)]
/// FigCon
/// 
/// A simple synchronous config manager that relies on serde_json
///
/// It stores its own path location and can be saved/reloaded at any time
pub struct FigCon {
    live_config: Value,
    location: PathBuf
}

impl Display for FigCon {
    /// # Format
    /// 
    /// A potentially slow function which attempts to display the entire configuration as a prettified json string
    /// 
    /// It would be inadvisable to use this on larger configurations during runtime
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self.live_config).unwrap())
    }
}

impl FigCon {
    /// # Initialize the FigCon
    /// 
    /// Attempts to load a config file with the given PathBuf
    /// and returns an empty FigCon when it fails
    pub fn load_or_default(path: PathBuf) -> Self {
        if path.exists() && let Ok(file) = File::open(&path) {
            let mut buffer: String = Default::default();
            (&file).read_to_string(&mut buffer).expect("Failed to read config from storage");
            let json: Value = serde_json::from_str(&buffer).expect("JSON deserialization failed");
            FigCon { live_config: json, location: path }
        } else {
            FigCon { live_config: serde_json::Value::Object(Default::default()), location: path }
        }
    }

    /// # Set Config Path
    /// 
    /// Changing the location during runtime will not affect the live config, and it will not save to the new location automatically.
    /// 
    /// Use `.save()` immediately after if you wish to write the live config to the new location
    pub fn set_path(&mut self, path: PathBuf) {
        self.location = path;
    }

    /// # Reload Config
    /// 
    /// Pull the config file again and overwrite the config in memory
    pub fn reload(&mut self) -> Self {
        Self::load_or_default(self.location.clone())
    }

    /// # Save Config
    /// 
    /// Write the current config state synchronously to the file system
    pub fn save(&self) {
        let file = File::create(&self.location).expect("Failed to create config file"); // this works regardless of if file exists or not
        let file = BufWriter::new(file); // this makes it orders of magnitude faser
        serde_json::to_writer_pretty(file, &self.live_config).expect("Config JSON serialization / writeout failed");
    }

    /// # Get 
    /// 
    /// Get a serde_json Value with a specified key
    pub fn get(&self, key: String) -> Option<Value> {
        let conf = self.live_config.as_object().unwrap();
        if conf.contains_key(&key) {
            Some(self.live_config[key].clone())
        } else { None }
    }

    /// # Get (Static)
    /// 
    /// Get a serde_json Value with a specified key
    pub fn get_st(&self, key: &str) -> Option<Value> {
        self.get(key.to_owned())
    }

    /// # Get Object
    /// 
    /// A safe getter for objects in the config
    pub fn get_obj(&self, key: String) -> Option<Value> {
        let conf = self.live_config[key].clone();
        if conf.is_object() {
            Some(conf)
        } else { None }
    }

    /// # Get Object (Static)
    /// 
    /// A safe getter for objects in the config
    pub fn get_obj_st(&self, key: &str) -> Option<Value> {
        self.get_obj(key.to_owned())
    }

    /// # New Object
    /// 
    /// Create a child structure within the current config with a given key
    pub fn new_obj(&mut self, key: String) {
        self.set(key, Value::new_obj());
    }

    /// # New Object (Static)
    /// 
    /// Create a child structure within the current config with a given key
    pub fn new_obj_st(&mut self, key: &str) {
        self.new_obj(key.to_owned());
    }

    /// # Set
    /// 
    /// Set a value with a specified key and serde_json Value
    pub fn set(&mut self, key: String, value: Value) {
        let conf = self.live_config.as_object_mut().unwrap();
        if conf.contains_key(&key) {
            conf[&key] = value;
        } else {
            conf.insert(key, value);
        }
    }

    /// # Set (Static)
    /// 
    /// Set a value with a specified key and serde_json Value
    pub fn set_st(&mut self, key: &str, value: Value) {
        self.set(key.to_owned(), value);
    }

    /// # Set String
    /// 
    /// Set a value with a specified key and static string
    pub fn set_str(&mut self, key: &str, value: String) {
        self.set_st(key, Value::String(value));
    }

    /// # Set String (Static)
    /// 
    /// Set a value with a specified key and static string
    pub fn set_str_st(&mut self, key: &str, value: &str) {
        self.set_str(key, value.to_owned());
    }

    /// # Delete 
    /// 
    /// Removes an entry with a specified key. Returns an option with the deleted Value (if it exists)
    pub fn del(&mut self, key: String) -> Option<Value> {
        let conf = self.live_config.as_object_mut().unwrap();
        if conf.contains_key(&key) {
            conf.remove(&key)
        } else { None }
    }

    /// # Delete (Static)
    /// 
    /// Removes an entry with a specified key. Returns an option with the deleted Value (if it exists)
    pub fn del_st(&mut self, key: &str) -> Option<Value> {
        self.del(key.to_owned())
    }

    /// # Has Key
    /// 
    /// Checks if a key exists with a specified name
    pub fn has(&self, key: String) -> bool {
        self.live_config.as_object().unwrap().contains_key(&key)
    }

    /// # Has Key (Static)
    /// 
    /// Checks if a key exists with a specified name
    pub fn has_st(&self, key: &str) -> bool {
        self.has(key.to_owned())
    }
}