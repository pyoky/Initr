use std::io;
use std::io::prelude::*;
use std::io::Write; // bring trait into scope
use std::fs;
use std::path;
use home::home_dir;
use google_drive;
use google_drive::{AccessToken, Client};
use google_drive::traits::FileOps;


static INITR_FOLDER_ID: &str = "1LgW3FWg3U1MVF_ZuP-RJ_0F42AgqeIi4";


pub async fn save_token(client: &mut google_drive::Client) -> Result<(), io::Error> {
    let access_token = client.refresh_access_token().await.unwrap();

    let serialized = serde_json::to_string(&access_token).unwrap();
    
    let mut dir = home_dir().unwrap();
    dir.push(".initr");

    let mut file = fs::File::create(dir)?;
    println!("json serialized token: {}", serialized);
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

pub async fn read_token() -> Result<AccessToken, io::Error> {
    
    let mut dir = home_dir().unwrap();
    dir.push(".initr");

    let mut file = fs::File::open(dir)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let ac: AccessToken = serde_json::from_str(&mut s)?;
    Ok(ac)

}

pub async fn print_files_in_initr_folder(client: &google_drive::Client) {
    // Make a request according to API spcifications
    let q = format!("mimeType != 'application/vnd.google-apps.folder' and \'{}\' in parents", INITR_FOLDER_ID);
    let files = client.files();
    let all_files = files.list_all(
        "user", "", false, "", false, "folder", q.as_str(), "", false, false, "").await;
    for file in all_files.unwrap() {
        println!("{} in {:?} id {:?}", file.name, file.parents, file.id);
    }
}

pub async fn download_file_to(client: &google_drive::Client, id: &str, dir: &path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let files = client.files();
    let data = files.download_by_id(id).await?;
    let mut file = fs::File::create(dir)?;
    file.write_all(data.as_ref())?;
    Ok(())
}

pub async fn init_drive() -> Result<Client, io::Error> {

    let mut dir = home_dir().ok_or(io::Error::from(io::ErrorKind::Other))?;
    dir.push(".initr");

    println!("checking .initr file");

    if dir.exists() {
        let access_token = read_token().await?;
        let drive = Client::new(
            String::from("452368723177-blg1jbfft8posccbi83jimsscv9u2gfi.apps.googleusercontent.com"),
            String::from("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"),
            String::from("urn:ietf:wg:oauth:2.0:oob"),
            String::from(access_token.access_token.as_str()),
            String::from(access_token.refresh_token.as_str())
        );
        return Ok(drive)
    }

    let mut drive = Client::new(
        String::from("452368723177-blg1jbfft8posccbi83jimsscv9u2gfi.apps.googleusercontent.com"),
        String::from("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"),
        String::from("urn:ietf:wg:oauth:2.0:oob"),
        String::from(""),
        String::from("")
    );
    
    let user_consent_url = drive.user_consent_url(&["https://www.googleapis.com/auth/drive".to_string()]);
    println!("Follow URL: {}", user_consent_url);
    open::that(&user_consent_url)?;

    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("error parsing user input");
    
    let code = buffer.as_str();
    let access_token = drive.get_access_token(code, "").await.unwrap();
    
    if access_token.access_token.len() > 2 { println!("auth success"); }
    else { panic!("auth failed");}

    save_token(&mut drive).await?;


    Ok(drive)
}
