use std::io;
use std::io::prelude::*;
use std::io::Write; // bring trait into scope
use std::fs;
use std::path;
use home;
use google_drive;
use google_drive::Client;
use google_drive::traits::FileOps;

pub async fn download_file_to(client: &google_drive::Client, id: &str, dir: &path::Path) {
    let files = client.files();
    let res = files.download_by_id(id).await;
    let data = match res {
        Ok(d) => d,
        Err(e) => panic!("error downloading : {:?}", e)
    };
    let mut file = fs::File::create(dir).unwrap();
    file.write_all(data.as_ref());
}

pub async fn init_drive() -> Client {
    let mut drive = Client::new(
        String::from("452368723177-blg1jbfft8posccbi83jimsscv9u2gfi.apps.googleusercontent.com"),
        String::from("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"),
        String::from("urn:ietf:wg:oauth:2.0:oob"),
        String::from(""),
        String::from("")
    );
    
    let user_consent_url = drive.user_consent_url(&["https://www.googleapis.com/auth/drive".to_string()]);
    println!("Follow URL: {}", user_consent_url);
    match open::that(&user_consent_url) {
        Ok(_) => (), 
        Err(err) => panic!("An error occured during authorization: {}",err),
    }
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("error parsing user input");
    
    let code = buffer.as_str();
    let access_token = drive.get_access_token(code, "").await.unwrap();
    
    if access_token.access_token.len() > 2 { println!("auth success"); }
    else { panic!("auth failed");}

    return drive;
}