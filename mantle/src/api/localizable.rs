#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Locale {
    // US English
    EN,
    // Great Britain English
    EN_GB,
    // German or Deutsch
    DE,
    // Spaniard Spanish
    ES,
    // Latin America Spanish
    ES_419,
    // European French
    FR,
    // Canadian French
    FR_CA,
    // Italian
    IT,
    // Simplified Chinese
    ZH_HANS,
    // Japanese
    JA,
}

impl Locale {
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "en" => Ok(Locale::EN),
            "en-GB" => Ok(Locale::EN_GB),
            "de" => Ok(Locale::DE),
            "es" => Ok(Locale::ES),
            "es-419" => Ok(Locale::ES_419),
            "fr" => Ok(Locale::FR),
            "fr-CA" => Ok(Locale::FR_CA),
            "it" => Ok(Locale::IT),
            "zh-Hans" => Ok(Locale::ZH_HANS),
            "ja" => Ok(Locale::JA),
            _ => Err(()),
        }
    }
}

pub struct Localizable {
    key: String,
    value: String,
}

impl Localizable {
    /*fn eng_bundle() -> Option<fluent_bundle::FluentBundle> {
        let locale = Locale::EN;
        let path = format!("/path/to/{:?}.ftl", locale);
        match fluent_loader::Loader::new().load_from_file(&path) {
            Ok(bundle) => Some(bundle),
            Err(_) => None,
        }
    }

    fn with_args(&self, args: &[&dyn fmt::Debug]) -> String {
        let arguments: Vec<String> = args.iter().map(|arg| format!("{:?}", arg)).collect();
        format!("{} {}", self.value, args.join(", "))
    }*/

    fn uppercased(&self) -> String {
        self.value.to_uppercase()
    }

    fn lowercased(&self) -> String {
        self.value.to_lowercase()
    }

    pub fn locale() -> Locale {
        let locale = "en";
        println!("Device Language: {:?}", locale);
        match Locale::from_str(locale) {
            Ok(locale) => locale,
            Err(_) => Locale::EN,
        }
    }
}