use std::collections::HashMap;
use std::env;

/// Get an environment variable by key and return its value as a String.
/// # Arguments
/// * `key` - A string slice that holds the name of the environment variable.
/// # Returns
/// * `String` - The value of the environment variable if it exists, otherwise an error message is printed to the console.
pub fn get_env_variable(key: &str) -> String {
    dotenvy::dotenv().ok();

    match env::var(key) {
        Ok(val) => val,
        Err(e) => {
            println!("Error {}: {}", key, e);
            String::new()
        }
    }
}

/// Get multiple environment variables by their keys and return a HashMap of key-value pairs.
/// # Arguments
/// * `vec` - A vector of string slices that holds the names of the environment variables.
/// # Returns
/// * `HashMap<String, String>` - A HashMap containing the key-value pairs of the environment variables that were successfully retrieved. If an environment variable cannot be retrieved, an error message is printed to the console for that variable.
pub fn get_env_variables(vec: Vec<&str>) -> HashMap<String, String> {
    dotenvy::dotenv().ok();
    let mut env_variables = HashMap::new();

    for key in vec {
        let value = env::var(&key);
        match value {
            Ok(val) => {
                println!("{}: {:?}", key, val);
                env_variables.insert(key.to_string(), val);
            }
            Err(e) => println!("Error {}: {}", key, e),
        };
    }
    env_variables
}
