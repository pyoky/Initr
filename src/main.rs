use clap::{Subcommand, Parser};
use google_drive;
use google_drive::Client;
use google_drive::traits::FileOps;
use tokio;
use open;
use home;
use std::fs;
use std::path;

use initr::*;

// Google Drive API file IDs
static FIREFOX_APPIMAGE_ID: &str = "1RespoMBLqUoo1lIu9osm3HHYWRHgIXQK";
static FIREFOX_CONFIGS_ID: &str = "1nuDAWIF4JTVQFYa_a4WuJ00vB1qdegKg";
static FOLDER_ID: &str = "1LgW3FWg3U1MVF_ZuP-RJ_0F42AgqeIi4";

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

    // get user's home dir
    let mut home_dir = match home::home_dir() {
        Some(path) => path,
        None => panic!("Impossible to get your home dir!"),
    };

    match &cli.command {
        Commands::Download { appname } => {
            match appname.as_str() {
                "firefox" => config_firefox(&home_dir).await,
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

async fn config_firefox(dir: &path::PathBuf) {
    
    let mut drive = init_drive().await;
    // Make a request according to API spcifications
    let q = format!("mimeType != 'application/vnd.google-apps.folder' and \'{}\' in parents", FOLDER_ID);
    let files = drive.files();
    let all_files = files.list_all(
        "user", "", false, "", false, "folder", q.as_str(), "", false, false, "").await;
    for file in all_files.unwrap() {
        println!("{} in {:?} id {:?}", file.name, file.parents, file.id);
    }
    let mut dir = dir.clone();
    dir.push("firefox.appimage");
    download_file_to(&drive, FIREFOX_APPIMAGE_ID, dir.as_path());
}
