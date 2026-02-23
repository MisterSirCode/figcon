use serde_json::{Map, Value, json, map::{Keys, Values}};
use std::{
    fmt::Display, 
    fs::File, 
    io::{
        BufWriter, 
        Read
    }, 
    path::PathBuf
};

pub trait ValueExtensions {
    fn obj(&self) -> Option<&Map<String, Value>>;
    fn obj_mut(&mut self) -> Option<&mut Map<String, Value>>;
    fn any_keys(&self) -> bool;
    fn list_keys(&self) -> Option<Vec<String>>;
    fn list_values(&self) -> Option<Vec<Value>>;
    fn iter_keys(&mut self) -> Option<Keys<'_>>;
    fn iter_values(&mut self) -> Option<Values<'_>>;
    fn set_key(&mut self, key: String, value: Value);
    fn set_key_st(&mut self, key: &str, value: Value);
    fn get_key(&mut self, key: String) -> Option<&mut Value>;
    fn get_key_st(&mut self, key: &str) -> Option<&mut Value>;
    fn has_key(&self, key: String) -> bool;
    fn has_key_st(&self, key: &str) -> bool;
    fn remove_get_key(&mut self, key: String) -> Option<Value>;
    fn remove_get_key_st(&mut self, key: &str) -> Option<Value>;
    fn remove_key(&mut self, key: String);
    fn remove_key_st(&mut self, key: &str);
    fn set_obj(&mut self, key: String, object: Value);
    fn set_obj_st(&mut self, key: &str, object: Value);
    fn get_obj(&mut self, key: String) -> Option<&mut Value>;
    fn get_obj_st(&mut self, key: &str) -> Option<&mut Value>;
    fn new_obj(&mut self, key: String) -> Option<&mut Value>;
    fn new_obj_st(&mut self, key: &str) -> Option<&mut Value>;
}

impl ValueExtensions for Value {
    /// # Object
    /// 
    /// Return an immutable reference to an object within a value
    fn obj(&self) -> Option<&Map<String, Value>> {
        if self.is_object() { Some(self.as_object().unwrap()) }
        else { None }
    }

    /// # Object (Mutable)
    /// 
    /// Return a mutable reference to an object within a value
    fn obj_mut(&mut self) -> Option<&mut Map<String, Value>> {
        if self.is_object() { Some(self.as_object_mut().unwrap()) }
        else { None }
    }

    /// # Any Keys
    /// 
    /// Returns true if the object contains any keys (Length > 0)
    /// 
    /// Automatically returns false if used on non-objects
    fn any_keys(&self) -> bool {
        match self.obj() {
            Some(object) => {
                !object.is_empty()
            },
            None => false
        }
    }

    /// # List Keys
    /// 
    /// Attempts to return all keys within an object
    fn list_keys(&self) -> Option<Vec<String>> {
        if self.any_keys() {
            Some(self.as_object().unwrap().keys().cloned().collect())
        } else { None }
    }

    /// # List Values
    /// 
    /// Attempts to return all values within an object
    fn list_values(&self) -> Option<Vec<Value>> {
        if self.any_keys() {
            Some(self.as_object().unwrap().values().cloned().collect())
        } else { None }
    }

    /// # Iterate Keys
    /// 
    /// Attempts to return an iterator for the keys within the current object.
    fn iter_keys(&mut self) -> Option<Keys<'_>> {
        if self.any_keys() {
            Some(self.as_object().unwrap().keys())
        } else { None }
    }

    /// # Iterate Values
    /// 
    /// Attempts to return an iterator for the values within the current object.
    fn iter_values(&mut self) -> Option<Values<'_>> {
        if self.any_keys() {
            Some(self.as_object().unwrap().values())
        } else { None }
    }

    /// # Set Key
    /// 
    /// Assign a key's value within an object within a value
    /// 
    /// Will do nothing if used on non-objects
    fn set_key(&mut self, key: String, value: Value) {
        match self.obj_mut() {
            Some(object) => {
                if object.contains_key(&key) {
                    object[&key] = value;
                } else {
                    object.insert(key.to_owned(), value);
                }
            },
            None => {}
        }
    }

    /// # Set Key (Static)
    /// 
    /// Assign a key's value within an object within a value
    /// 
    /// Will do nothing if used on non-objects
    fn set_key_st(&mut self, key: &str, value: Value) {
        self.set_key(key.to_owned(), value);
    }

    /// # Get Key
    /// 
    /// Acquire a key's value within an object within a value
    /// 
    /// Will return None if used on non-objects
    fn get_key(&mut self, key: String) -> Option<&mut Value> {
        match self.obj_mut() {
            Some(object) => {
                if object.contains_key(&key) {
                    Some(&mut object[&key])
                } else {
                    None
                }
            },
            None => None
        }
    }

    /// # Get Key (Static)
    /// 
    /// Acquire a key's value within an object within a value
    /// 
    /// Will return None if used on non-objects
    fn get_key_st(&mut self, key: &str) -> Option<&mut Value> {
        self.get_key(key.to_owned())
    }

    /// # Has Key
    /// 
    /// Check if a key exists within an object within a value
    /// 
    /// Automatically returns false if used on non-objects
    fn has_key(&self, key: String) -> bool {
        match self.obj() {
            Some(object) => {
                object.contains_key(&key)
            },
            None => false
        }
    }

    /// # Has Key
    /// 
    /// Check if a key exists within an object within a value
    /// 
    /// Automatically returns false if used on non-objects
    fn has_key_st(&self, key: &str) -> bool {
        self.has_key(key.to_owned())
    }

    /// # Remove and Get Key
    /// 
    /// Remove an object's key within a value and return it if it exists
    /// 
    /// Will return None if used on non-objects
    fn remove_get_key(&mut self, key: String) -> Option<Value> {
        match self.obj_mut() {
            Some(object) => {
                object.remove(&key)
            },
            None => None
        }
    }

    /// # Remove and Get Key (Static)
    /// 
    /// Remove an object's key within a value and return it if it exists
    /// 
    /// Will return None if used on non-objects
    fn remove_get_key_st(&mut self, key: &str) -> Option<Value> {
        self.remove_get_key(key.to_owned())
    }

    /// # Remove Key
    /// 
    /// Remove an object's key within a value without respect to whether its assigned, or where its value goes
    /// 
    /// Will do nothing if used on non-objects
    fn remove_key(&mut self, key: String) {
        self.remove_get_key(key);
    }

    /// # Remove Key (Static)
    /// 
    /// Remove an object's key within a value without respect to whether its assigned, or where its value goes
    /// 
    /// Will do nothing if used on non-objects
    fn remove_key_st(&mut self, key: &str) {
        self.remove_key(key.to_owned());
    }

    /// # Set Object
    /// 
    /// Overwrite an object within the value, and combine the keys inside
    /// 
    /// Will do nothing if used on non-objects
    fn set_obj(&mut self, key: String, object: Value) {
        if !object.is_object() { return; } // Dont waste time. No object? leave
        if !object.any_keys() { return; } // Dont waste time. No keys? leave
        let can_extend = self.has_key(key.clone()) && self.any_keys();
        match self.obj_mut() {
            Some(self_obj) => {
                if can_extend {
                    self_obj.extend(object.obj().unwrap().clone());
                } else {
                    self.set_key(key, object);
                }
            },
            None => {}
        }
    }

    /// # Set Object (Static)
    /// 
    /// Overwrite an object within the value, and combine the keys inside
    /// 
    /// Will do nothing if used on non-objects
    fn set_obj_st(&mut self, key: &str, object: Value) {
        self.set_obj(key.to_owned(), object);
    }

    /// # Get Object
    /// 
    /// Get an object within the keys
    /// 
    /// Will return None if used on non-objects
    fn get_obj(&mut self, key: String) -> Option<&mut Value> {
        if !self.is_object() { return None; }
        match self.get_key(key) {
            Some(value) => {
                if value.is_object() {
                    Some(value)
                } else { None }
            },
            None => None
        }
    }

    /// # Get Object (Static)
    /// 
    /// Get an object within the keys
    /// 
    /// Will return None if used on non-objects
    fn get_obj_st(&mut self, key: &str) -> Option<&mut Value> {
        self.get_obj(key.to_owned())
    }

    /// # New Object
    /// 
    /// Create a child structure within the current config with a given key
    /// 
    /// Will return None if used on non-objects
    fn new_obj(&mut self, key: String) -> Option<&mut Value> {
        if !self.is_object() { return None; }
        self.set_key(key.clone(), json!({}));
        Some(self.get_key(key).unwrap())
    }

    /// # New Object (Static)
    /// 
    /// Create a child structure within the current config with a given key
    /// 
    /// Will return None if used on non-objects
    fn new_obj_st(&mut self, key: &str) -> Option<&mut Value> {
        self.new_obj(key.to_owned())
    }
}

#[derive(Clone, Debug)]
/// # FigCon
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

    /// # Any Keys
    /// 
    /// Returns true if the object contains any keys (Length > 0)
    /// 
    /// Automatically returns false if used on non-objects
    pub fn any_keys(&self) -> bool {
        self.live_config.any_keys()
    }

    /// # List Keys
    /// 
    /// Attempts to return all keys within an object
    pub fn list_keys(&self) -> Option<Vec<String>> {
        self.live_config.list_keys()
    }

    /// # List Values
    /// 
    /// Attempts to return all values within an object
    pub fn list_values(&self) -> Option<Vec<Value>> {
        self.live_config.list_values()
    }

    /// # Iterate Keys
    /// 
    /// Attempts to return an iterator for the keys within the current object.
    pub fn iter_keys(&mut self) -> Option<Keys<'_>> {
        self.live_config.iter_keys()
    }

    /// # Iterate Values
    /// 
    /// Attempts to return an iterator for the values within the current object.
    pub fn iter_values(&mut self) -> Option<Values<'_>> {
        self.live_config.iter_values()
    }

    /// # Get Key
    /// 
    /// Acquire a key's value within an object within a value
    /// 
    /// Will return None if used on non-objects
    pub fn get_key(&mut self, key: String) -> Option<&mut Value> {
        self.live_config.get_key(key)
    }

    /// # Get Key (Static)
    /// 
    /// Acquire a key's value within an object within a value
    /// 
    /// Will return None if used on non-objects
    pub fn get_key_st(&mut self, key: &str) -> Option<&mut Value> {
        self.get_key(key.to_owned())
    }

    /// # Set Key
    /// 
    /// Assign a key's value within an object within a value
    /// 
    /// Will do nothing if used on non-objects
    pub fn set_key(&mut self, key: String, value: Value) {
        self.live_config.set_key(key, value);
    }

    /// # Set Key (Static)
    /// 
    /// Assign a key's value within an object within a value
    /// 
    /// Will do nothing if used on non-objects
    pub fn set_key_st(&mut self, key: &str, value: Value) {
        self.set_key(key.to_owned(), value);
    }

    /// # Has Key
    /// 
    /// Check if a key exists within an object within a value
    /// 
    /// Automatically returns false if used on non-objects
    pub fn has_key(&self, key: String) -> bool {
        self.live_config.has_key(key)
    }

    /// # Has Key
    /// 
    /// Check if a key exists within an object within a value
    /// 
    /// Automatically returns false if used on non-objects
    pub fn has_key_st(&self, key: &str) -> bool {
        self.has_key(key.to_owned())
    }

    /// # Remove and Get Key
    /// 
    /// Remove an object's key within a value and return it if it exists
    /// 
    /// Will return None if used on non-objects
    pub fn remove_get_key(&mut self, key: String) -> Option<Value> {
        self.live_config.remove_get_key(key)
    }

    /// # Remove and Get Key (Static)
    /// 
    /// Remove an object's key within a value and return it if it exists
    /// 
    /// Will return None if used on non-objects
    pub fn remove_get_key_st(&mut self, key: &str) -> Option<Value> {
        self.remove_get_key(key.to_owned())
    }

    /// # Remove Key
    /// 
    /// Remove an object's key within a value without respect to whether its assigned, or where its value goes
    /// 
    /// Will do nothing if used on non-objects
    pub fn remove_key(&mut self, key: String) {
        self.remove_get_key(key);
    }

    /// # Remove Key (Static)
    /// 
    /// Remove an object's key within a value without respect to whether its assigned, or where its value goes
    /// 
    /// Will do nothing if used on non-objects
    pub fn remove_key_st(&mut self, key: &str) {
        self.remove_key(key.to_owned());
    }

    /// # Set Object
    /// 
    /// Overwrite an object within the value, and combine the keys inside
    /// 
    /// Will do nothing if used on non-objects
    pub fn set_obj(&mut self, key: String, object: Value) {
        self.live_config.set_obj(key, object);
    }

    /// # Set Object (Static)
    /// 
    /// Overwrite an object within the value, and combine the keys inside
    /// 
    /// Will do nothing if used on non-objects
    pub fn set_obj_st(&mut self, key: &str, object: Value) {
        self.set_obj(key.to_owned(), object);
    }

    /// # Get Object
    /// 
    /// Get an object within the keys
    /// 
    /// Will return None if used on non-objects
    pub fn get_obj(&mut self, key: String) -> Option<&mut Value> {
        self.live_config.get_obj(key)
    }

    /// # Get Object (Static)
    /// 
    /// Get an object within the keys
    /// 
    /// Will return None if used on non-objects
    pub fn get_obj_st(&mut self, key: &str) -> Option<&mut Value> {
        self.get_obj(key.to_owned())
    }

    /// # New Object
    /// 
    /// Create a child structure within the current config with a given key
    pub fn new_obj(&mut self, key: String) -> &mut Value {
        self.live_config.new_obj(key).unwrap() // No option handling- Live config is always an object
    }

    /// # New Object (Static)
    /// 
    /// Create a child structure within the current config with a given key
    pub fn new_obj_st(&mut self, key: &str) -> &mut Value {
        self.new_obj(key.to_owned())
    }
}