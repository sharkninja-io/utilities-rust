pub struct Device {
    pub platform: Platform,
    pub language_code: String,
    pub resolution: String,
}

pub enum Platform {
    iOS,
    Android,
}
