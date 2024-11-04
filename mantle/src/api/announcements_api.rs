use std::collections::HashMap;
use std::str::FromStr;
use serde::Deserialize;
use chrono::prelude::*;
use crate::api::localizable::{Localizable, Locale};
use crate::api::date_generator::{DateGenerator, DateFormat};
//use crate::api::version_comparator::{AppBuildNumberVersion, AppMarketVersion, AppOSVersion};

#[derive(Debug, Deserialize)]
enum Errors {
    TimeIntervalNotMet,
    InstructionsNotAvailable,
}

#[derive(Clone)]
struct Device {
    model: String,
    dsn: String,
    family: String,
    classification: String,
}

struct AnnouncementsApi {
    robot: Option<Device>,
}

impl AnnouncementsApi {
    fn new(robot: Option<Device>) -> Self {

        Self {
            robot,
        }
    }
}

struct Metadata {
    user_country: String,
    //server_region: CountryRegionSupportServer,
    language_code: String,
    ota_nav_version: String,
    number_of_app_launches: i32,
    number_of_app_launches_with_robot: i32,
    date_since_first_launch: DateTime<Utc>,
    date_since_first_launch_with_robots: DateTime<Utc>,
}

impl Metadata {
    async fn new(dsn: String) -> Metadata {
        let user_country = "US".to_string();
        //let server_region = Self::get_server_region().await;
        let language_code = "en".to_string();
        let ota_nav_version = "1.0.0".to_string();
        let number_of_app_launches = 0;
        let number_of_app_launches_with_robot = 0;
        let date_since_first_launch = Utc::now();
        let date_since_first_launch_with_robots = Utc::now();

        Metadata {
            user_country,
            //server_region,
            language_code,
            ota_nav_version,
            number_of_app_launches,
            number_of_app_launches_with_robot,
            date_since_first_launch,
            date_since_first_launch_with_robots,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Instructions {
    pub version: i32,
    pub data: ItemData,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ItemData {
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Item {
    #[serde(rename = "titleLabel")]
    pub title_label: Option<String>,
    pub uuid: String,
    pub date: String,
    pub frequency: Frequency,
    #[serde(rename = "executionScope")]
    pub execution_scope: Scope,
    #[serde(rename = "contentURLs")]
    pub content_urls: HashMap<String, String>,
    pub actions: Actions,
    pub predicates: Vec<Predicate>,
    #[serde(rename = "includedInHistory")]
    pub included_in_history: bool,
    #[serde(rename = "isFinalItem")]
    pub is_final_item: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Frequency {
    Once,
    Always,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum Scope {
    Family,
    Model,
    Robot,
    #[serde(rename = "app")]
    Application,
    Classification,
}

impl Scope {
    fn record_tag(&self, uuid: &str, robot: &Device) -> String {
        match self {
            Scope::Family => format!("family-{}-{}", robot.family, uuid),
            Scope::Model => format!("model-{}-{}", robot.model, uuid),
            Scope::Robot => format!("robot-{}-{}", robot.dsn, uuid),
            Scope::Application => format!("app-{}", uuid),
            Scope::Classification => format!("classification-{}-{}", robot.classification, uuid),
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Actions {
    #[serde(rename = "displayContentURL")]
    pub display_content_url: bool,
    #[serde(rename = "forceAppUpdate")]
    pub force_app_update: bool,
    #[serde(rename = "forceLogout")]
    pub force_logout: bool,
}

//Remake into HashMap
#[derive(Debug, Deserialize)]
struct LocalizedHTML {
    en_us: String,
    en_gb: Option<String>,
    fr_fr: Option<String>,
    fr_ca: Option<String>,
    es_es: Option<String>,
    it_it: Option<String>,
    de_de: Option<String>,
    zh_cn: Option<String>,
    ja_jp: Option<String>,
}

/*
impl LocalizedHTML {
    fn get_html_based_on_localization(&self) -> Option<String> {
        let locale = Localizable::locale();
        match locale {
            Locale::EN => Some(self.en_us.clone()),
            Locale::EN_GB => Some(self.en_gb.clone().unwrap_or_else(|| self.en_us.clone())),
            Locale::DE => Some(self.de_de.clone().unwrap_or_else(|| self.en_us.clone())),
            Locale::ES | Locale::ES_419 => Some(self.es_es.clone().unwrap_or_else(|| self.en_us.clone())),
            Locale::FR => Some(self.fr_fr.clone().unwrap_or_else(|| self.en_us.clone())),
            Locale::FR_CA => Some(self.fr_ca.clone().unwrap_or_else(|| self.en_us.clone())),
            Locale::IT => Some(self.it_it.clone().unwrap_or_else(|| self.en_us.clone())),
            Locale::ZH_HANS => Some(self.zh_cn.clone().unwrap_or_else(|| self.en_us.clone())),
            Locale::JA => Some(self.ja_jp.clone().unwrap_or_else(|| self.en_us.clone())),
        }
    }
}
*/

trait Evaluate {
    fn evaluate(&self, robot: &Device, metadata: &Metadata) -> bool;
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Predicate {
    pub op: Operator,
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Operator {
    And,
    Or,
    Not,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Condition {
    pub name: String,
    pub value: String,
}

impl Evaluate for Condition {
    fn evaluate(&self, robot: &Device, metadata: &Metadata) -> bool {
        match CriteriaKey::from_str(&self.name) {
            Ok(criteria) => match criteria {
                CriteriaKey::All => true,
                CriteriaKey::UserCountry => metadata.user_country == self.value,
                CriteriaKey::UserLanguage => metadata.language_code == self.value,
                //CriteriaKey::ServerRegion => metadata.server_region.to_string() == self.value,
                CriteriaKey::Date => {
                    if let Ok(date) = DateGenerator::date_from_string(&self.value, DateFormat::YYYY_MM_dd_T_HH_mm_ssZ, true) {
                        return chrono::Local::now().naive_utc() == date;
                    }
                    false
                }
                CriteriaKey::DateMin => {
                    if let Ok(date) = DateGenerator::date_from_string(&self.value, DateFormat::YYYY_MM_dd_T_HH_mm_ssZ, true) {
                        return date < chrono::Local::now().naive_utc();
                    }
                    false
                }
                CriteriaKey::DateMax => {
                    if let Ok(date) = DateGenerator::date_from_string(&self.value, DateFormat::YYYY_MM_dd_T_HH_mm_ssZ, true) {
                        return chrono::Local::now().naive_utc() < date;
                    }
                    false
                }
                /*
                CriteriaKey::AppVersion => AppBuildNumberVersion::is_equal_to(&self.value),
                CriteriaKey::AppVersionMin => AppBuildNumberVersion::is_minimum_required(&self.value),
                CriteriaKey::AppVersionMax => AppBuildNumberVersion::is_maximum_required(&self.value),
                CriteriaKey::AppMarketVersion => AppMarketVersion::is_equal_to(&self.value),
                CriteriaKey::AppMarketVersionMin => AppMarketVersion::is_minimum_required(&self.value),
                CriteriaKey::AppMarketVersionMax => AppMarketVersion::is_maximum_required(&self.value),
                CriteriaKey::AppOS => AppOSVersion::is_equal_to(&self.value),
                CriteriaKey::AppOSMin => AppOSVersion::is_minimum_required(&self.value),
                CriteriaKey::AppOSMax => AppOSVersion::is_maximum_required(&self.value),
                CriteriaKey::DeviceFamily => match RobotFamily::from_str(&self.value) {
                    Ok(family) => robot.family == family,
                    _ => false,
                },*/
                CriteriaKey::DeviceModel => robot.model == self.value,
                CriteriaKey::ScmVersion | CriteriaKey::ScmVersionMin | CriteriaKey::ScmVersionMax => false,
                CriteriaKey::NavVersion => metadata.ota_nav_version == self.value,
                CriteriaKey::NavVersionMin | CriteriaKey::NavVersionMax => false,
                CriteriaKey::NavVersionPrefix => metadata.ota_nav_version.starts_with(&self.value),
                CriteriaKey::NavVersionContains => metadata.ota_nav_version.contains(&self.value),
                CriteriaKey::AppLaunchCount => {
                    if let Ok(count) = self.value.parse::<i32>() {
                        return metadata.number_of_app_launches == count;
                    }
                    false
                }
                CriteriaKey::AppLaunchCountMin => {
                    if let Ok(count) = self.value.parse::<i32>() {
                        return count <= metadata.number_of_app_launches;
                    }
                    false
                }
                CriteriaKey::AppLaunchCountMax => {
                    if let Ok(count) = self.value.parse::<i32>() {
                        return metadata.number_of_app_launches <= count;
                    }
                    false
                }
                CriteriaKey::AppLaunchCountWithRobot => {
                    if let Ok(count) = self.value.parse::<i32>() {
                        return metadata.number_of_app_launches_with_robot == count;
                    }
                    false
                }
                CriteriaKey::AppLaunchCountWithRobotMin => {
                    if let Ok(count) = self.value.parse::<i32>() {
                        return count <= metadata.number_of_app_launches_with_robot;
                    }
                    false
                }
                CriteriaKey::AppLaunchCountWithRobotMax => {
                    if let Ok(count) = self.value.parse::<i32>() {
                        return metadata.number_of_app_launches_with_robot <= count;
                    }
                    false
                }
                /*
                CriteriaKey::DaysSinceFirstLaunch => {
                    if let Ok(days) = self.value.parse::<i64>() {
                        let today = chrono::Local::now().naive_utc();
                        return today.signed_duration_since(metadata.date_since_first_launch).num_days() == days;
                    }
                    false
                }
                CriteriaKey::DaysSinceFirstLaunchMin => {
                    if let Ok(days) = self.value.parse::<i64>() {
                        let today = chrono::Local::now().naive_utc();
                        return days <= today.signed_duration_since(metadata.date_since_first_launch).num_days();
                    }
                    false
                }
                CriteriaKey::DaysSinceFirstLaunchMax => {
                    if let Ok(days) = self.value.parse::<i64>() {
                        let today = chrono::Local::now().naive_utc();
                        return today.signed_duration_since(metadata.date_since_first_launch).num_days() <= days;
                    }
                    false
                }
                */
                CriteriaKey::DaysSinceFirstRobot | CriteriaKey::DaysSinceFirstRobotMin | CriteriaKey::DaysSinceFirstRobotMax => false,
                CriteriaKey::Missing => false,
            },
            Err(_) => false,
        }
    }
}

enum CriteriaKey {
    All,
    UserCountry,
    UserLanguage,
    //ServerRegion,
    Date,
    DateMin,
    DateMax,
    //AppVersion,
    //AppVersionMin,
    //AppVersionMax,
    //AppMarketVersion,
    //AppMarketVersionMin,
    //AppMarketVersionMax,
    //AppOS,
    //AppOSMin,
    //AppOSMax,
    //DeviceFamily,
    DeviceModel,
    ScmVersion,
    ScmVersionMin,
    ScmVersionMax,
    NavVersion,
    NavVersionMin,
    NavVersionMax,
    NavVersionPrefix,
    NavVersionContains,
    AppLaunchCount,
    AppLaunchCountMin,
    AppLaunchCountMax,
    AppLaunchCountWithRobot,
    AppLaunchCountWithRobotMin,
    AppLaunchCountWithRobotMax,
    //DaysSinceFirstLaunch,
    //DaysSinceFirstLaunchMin,
    //DaysSinceFirstLaunchMax,
    DaysSinceFirstRobot,
    DaysSinceFirstRobotMin,
    DaysSinceFirstRobotMax,
    Missing,
}

impl std::str::FromStr for CriteriaKey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(CriteriaKey::All),
            "userCountry" => Ok(CriteriaKey::UserCountry),
            "userLanguage" => Ok(CriteriaKey::UserLanguage),
            //"serverRegion" => Ok(CriteriaKey::ServerRegion),
            "date" => Ok(CriteriaKey::Date),
            "dateMin" => Ok(CriteriaKey::DateMin),
            "dateMax" => Ok(CriteriaKey::DateMax),
            //"appVersion" => Ok(CriteriaKey::AppVersion),
            //"appVersionMin" => Ok(CriteriaKey::AppVersionMin),
            //"appVersionMax" => Ok(CriteriaKey::AppVersionMax),
            //"appMarketVersion" => Ok(CriteriaKey::AppMarketVersion),
            //"appMarketVersionMin" => Ok(CriteriaKey::AppMarketVersionMin),
            //"appMarketVersionMax" => Ok(CriteriaKey::AppMarketVersionMax),
            //"appOS" => Ok(CriteriaKey::AppOS),
            //"appOSMin" => Ok(CriteriaKey::AppOSMin),
            //"appOSMax" => Ok(CriteriaKey::AppOSMax),
            //"deviceFamily" => Ok(CriteriaKey::DeviceFamily),
            "deviceModel" => Ok(CriteriaKey::DeviceModel),
            "scmVersion" => Ok(CriteriaKey::ScmVersion),
            "scmVersionMin" => Ok(CriteriaKey::ScmVersionMin),
            "scmVersionMax" => Ok(CriteriaKey::ScmVersionMax),
            "navVersion" => Ok(CriteriaKey::NavVersion),
            "navVersionMin" => Ok(CriteriaKey::NavVersionMin),
            "navVersionMax" => Ok(CriteriaKey::NavVersionMax),
            "navVersionPrefix" => Ok(CriteriaKey::NavVersionPrefix),
            "navVersionContains" => Ok(CriteriaKey::NavVersionContains),
            "appLaunchCount" => Ok(CriteriaKey::AppLaunchCount),
            "appLaunchCountMin" => Ok(CriteriaKey::AppLaunchCountMin),
            "appLaunchCountMax" => Ok(CriteriaKey::AppLaunchCountMax),
            "appLaunchCountWithRobot" => Ok(CriteriaKey::AppLaunchCountWithRobot),
            "appLaunchCountWithRobotMin" => Ok(CriteriaKey::AppLaunchCountWithRobotMin),
            "appLaunchCountWithRobotMax" => Ok(CriteriaKey::AppLaunchCountWithRobotMax),
            //"daysSinceFirstLaunch" => Ok(CriteriaKey::DaysSinceFirstLaunch),
            //"daysSinceFirstLaunchMin" => Ok(CriteriaKey::DaysSinceFirstLaunchMin),
            //"daysSinceFirstLaunchMax" => Ok(CriteriaKey::DaysSinceFirstLaunchMax),
            "daysSinceFirstRobot" => Ok(CriteriaKey::DaysSinceFirstRobot),
            "daysSinceFirstRobotMin" => Ok(CriteriaKey::DaysSinceFirstRobotMin),
            "daysSinceFirstRobotMax" => Ok(CriteriaKey::DaysSinceFirstRobotMax),
            _ => Ok(CriteriaKey::Missing),
        }
    }
}