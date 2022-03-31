use clap::{Subcommand, Parser};
use google_drive;
use google_drive::Client;
use google_drive::traits::FileOps;
use tokio;
use open;
use std::io;
use std::io::prelude::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Download {
        appname: String
    },
    Upload {
        appname: String
    }
}
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    

    match &cli.command {
        Commands::Download { appname } => {
            match appname.as_str() {
                "firefox" => config_firefox().await,
                _ => println!("please enter a valid app name"), 
            }
        },
        Commands::Upload { appname } => {
            match appname.as_str() {
                firefox => println!("will upload firefox"),
                _ => (),
            }
        }
    }

}

async fn config_firefox() {
    let mut drive = init_drive().await;
    let q = "mimeType != 'application/vnd.google-apps.folder' and \'1LgW3FWg3U1MVF_ZuP-RJ_0F42AgqeIi4\' in parents";
    let files = drive.files();
    let all_files = files.list_all(
        "user", 
        "",
        false,
        "",
        false,
        "folder",
        q,
        "",
        false,
        false,
        "").await;
    for file in all_files.unwrap() {
        println!("{} in {:?} id {:?}", file.name, file.parents, file.id);
    }
    let res = files.download_by_id("1RespoMBLqUoo1lIu9osm3HHYWRHgIXQK");
}

async fn init_drive() -> Client {
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