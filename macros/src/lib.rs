

#[macro_export]
macro_rules! json_from_file {
    ($path:expr) => {
        {
            use serde_json::Value;
            use std::fs::File;
            use std::io::Read; 
            let mut file = File::open($path).expect("Unable to open the file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Unable to read the file");
            let v: Value = serde_json::from_str(&contents).expect("Invalid JSON format");
            match v {
                Value::Object(obj) => obj,
                _ => panic!("Expected a JSON object"),
            }
        }
    };
}

