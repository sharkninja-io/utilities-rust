use std::collections::HashMap;
use std::path::Path;
use std::fs::File;

use mantle_utilities::api::announcements_api::{Instructions, ItemData, Item, Frequency, Scope, Actions, Predicate, Operator, Condition};

#[test]
fn parse_json_api_data() {
    let expected = Instructions { 
        version: 1, data: ItemData { 
            items: [ 
                Item { 
                    title_label: Some("".to_string()), uuid: "973BFF02-8E3A-4A14-A32C-E098F2AE2319".to_string(), date: "2023-02-21T00:00:00+0000".to_string(), 
                    frequency: Frequency::Once, execution_scope: Scope::Application, content_urls: HashMap::from_iter([
                        ("EN_US".to_string(), "https://fisharefriends.s3.amazonaws.com/releasenotes/2023/02/matrix/index.html".to_string()),
                        ("FR_FR".to_string(), "https://fisharefriends.s3.amazonaws.com/releasenotes/2023/02/matrix/index.html".to_string()),
                        ("FR_CA".to_string(), "https://fisharefriends.s3.amazonaws.com/releasenotes/2023/02/matrix/index.html".to_string()),
                        ("ZH_CN".to_string(), "https://fisharefriends.s3.amazonaws.com/releasenotes/2023/02/matrix/index.html".to_string()),
                        ("EN_GB".to_string(), "https://fisharefriends.s3.amazonaws.com/releasenotes/2023/02/matrix/index.html".to_string()),
                        ("ES_ES".to_string(), "https://fisharefriends.s3.amazonaws.com/releasenotes/2023/02/matrix/index.html".to_string()),
                        ("IT_IT".to_string(), "https://fisharefriends.s3.amazonaws.com/releasenotes/2023/02/matrix/index.html".to_string()),
                        ("DE_DE".to_string(), "https://fisharefriends.s3.amazonaws.com/releasenotes/2023/02/matrix/index.html".to_string()),
                        ("JA_JP".to_string(), "https://fisharefriends.s3.amazonaws.com/releasenotes/2023/02/matrix/index.html".to_string()),
                      ]), actions: Actions { 
                        display_content_url: true, force_app_update: false, force_logout: false 
                    }, predicates: [ 
                        Predicate { op: Operator::And, conditions: [ Condition { name: "appVersionMin".to_string(), value: "3022".to_string() } ].to_vec()}, 
                        Predicate { op: Operator::Or, conditions: [ Condition { name: "deviceFamily".to_string(), value: "Three60".to_string() }, Condition { name: "deviceFamily".to_string(), value: "Three60WetDry".to_string() }].to_vec() }
                        ].to_vec(), included_in_history: false, is_final_item: true }].to_vec() } 
    }; 

    let actual: Instructions;

    let json_file_path = Path::new("./tests/assets/api_test_data.json");

    let file = File::open(json_file_path).unwrap();

    actual = serde_json::from_reader(file).expect("error while reading or parsing");

    assert_eq!(actual, expected);
}
