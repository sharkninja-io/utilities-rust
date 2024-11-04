use std::{fs, vec, path::Path};
use serde::{Serialize, Deserialize};
use crate::crypt::decrypt_str;
use crate::http::request::{Request, Method};
use crate::http::client::SHARED;
use std::io::Cursor;
use crate::db::Bucket;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct JavaScriptFilesDB {
    pub files: Vec<JavaScriptFile>
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct JavaScriptFile {
    pub name: String, // Example: "360_features"
    pub entries: Vec<JavaScriptFileEntry>
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct JavaScriptFileEntry {
    pub version: String, // Example: "1.2.3"
    pub relative_path: String // Example: "javascript/360_features-1.2.3.js"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestData {
    pub dependencies: Vec<ManifestObject>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestObject {
    pub name: String,
    pub version: String,
}

impl JavaScriptFile {
    pub fn new() -> Self {
        Self { name: String::new(), entries: vec![] }
    }

    pub fn update_db_from_manifest(bucket: &Bucket<String, Vec<JavaScriptFileEntry>>, base_folder_path: String, download_url: &str, manifest_name: &str) {
        let response_text = {
            let url = format!("{}{}.bin", download_url, manifest_name);
            let req = Request { url: url, method: Method::GET, body: None, timeout: 60, headers: vec![] };
            let http_client = SHARED.lock().unwrap();
            let response = http_client.send_request(req);
            
            response.text().unwrap()
        };

        dbg!(response_text.clone());
        let decrypted_manifest = decrypt_str(&response_text, "config_encr.yml").unwrap();
        dbg!(decrypted_manifest.clone());
        let json: ManifestData = serde_json::from_str(&decrypted_manifest).unwrap();

        let full_folder_path = format!("{}/javascript", base_folder_path.clone());

        let mut script_files: Vec<JavaScriptFile> = vec![];

        for i in 0..json.dependencies.len() {
            Self::check_for_dep(&mut script_files, full_folder_path.clone(), json.dependencies[i].name.clone(), json.dependencies[i].version.clone(), &download_url);
        }

        for i in 0..script_files.len() {
            let key = script_files[i].name.clone();
            let value = script_files[i].entries.clone();
            bucket.insert(&key, &value).unwrap();
        } 
    }
    
    fn check_for_dep(java_script_files: &mut Vec<Self>, full_folder_path: String, dep_name: String, dep_ver: String, download_url: &str) {
        let file_path = format!("{}/{}-{}/js.js", full_folder_path, dep_name.clone(), dep_ver.clone());
        let found = match fs::metadata(Path::new(&file_path)) {
            Ok(_) => true,
            Err(_) => false,
        };

        if let Some(file) = java_script_files.iter_mut().find(|file| file.name == dep_name) {
            if file.entries.iter().find(|file_data| file_data.version == dep_ver).is_none() {
                file.entries.push(JavaScriptFileEntry { version: dep_ver.clone(), relative_path: format!("{}/{}-{}/js.js", full_folder_path, &dep_name, &dep_ver) });
            }
        }
        else {
            java_script_files.push(JavaScriptFile { name: dep_name.clone(), entries: vec![JavaScriptFileEntry { version: dep_ver.clone(), relative_path: format!("{}/{}-{}/js.js", full_folder_path, &dep_name, &dep_ver) }] });
        }

        if !found {
            Self::download_packages(dep_name, dep_ver, full_folder_path, download_url);
        }
        else {
            println!("Found dep {}-{}/js.js", dep_name, dep_ver);
        }
    }

    pub fn download_packages(dep_name: String, dep_ver: String, full_folder_path: String, download_url: &str)
    {
        let response = {
            let url = format!("{}packages/{}-{}.bin", download_url, dep_name.clone(), dep_ver.clone());
            let req = Request { url: url, method: Method::GET, body: None, timeout: 60, headers: vec![] };
            let http_client = SHARED.lock().unwrap();
            
            http_client.send_request(req)
        };

        let archive = response.content;
        let new_dir = format!("{}/{}-{}", full_folder_path, dep_name, dep_ver);
        let target_dir = Path::new(&new_dir);

        let res: Result<(), zip_extract::ZipExtractError> = zip_extract::extract(Cursor::new(archive), &target_dir, true);
        println!("Downloading {}-{}/js.js and saving here {}", dep_name, dep_ver, full_folder_path);
    }
}

impl JavaScriptFilesDB {
    pub fn new() -> Self {
        Self { files: vec![] }
    }
}