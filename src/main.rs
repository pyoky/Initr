use clap::{Subcommand, Parser};
use google_drive;
use google_drive::Client;
use google_drive::traits::FileOps;
use tokio;
use open;
use home::home_dir;
use std::fs;
use std::path;
use zip;

use initr::*;

// Google Drive API file IDs
static FIREFOX_APPIMAGE_ID: &str = "1RespoMBLqUoo1lIu9osm3HHYWRHgIXQK";
static FIREFOX_CONFIGS_ID: &str = "1nuDAWIF4JTVQFYa_a4WuJ00vB1qdegKg";

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
    },
    List,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Initialize Google Drive Client
    let mut client = init_drive().await;

    // get user's home dir
    let mut home_dir = match home_dir() {
        Some(path) => path,
        None => panic!("Impossible to get your home dir!"),
    };

    match &cli.command {
        Commands::Download { appname } => {
            match appname.as_str() {
                "firefox" => config_firefox(&client, &home_dir).await,
                _ => println!("please enter a valid app name"), 
            }
        },
        Commands::Upload { appname } => {
            match appname.as_str() {
                firefox => println!("will upload firefox"),
                _ => (),
            }
        },
        Commands::List => {
            print_files_in_initr_folder(&client).await;
        }
    }

}

async fn config_firefox(client: &google_drive::Client, dir: &path::PathBuf) {
    let mut dir = dir.clone();
    dir.push("firefox.appimage");
    println!("Downloading Firefox to {:?}", dir);
    download_file_to(client, FIREFOX_APPIMAGE_ID, dir.as_path()).await;
    dir.pop();

    dir.push("mozilla.zip");
    download_file_to(client, FIREFOX_CONFIGS_ID, dir.as_path()).await;

    let zipfile = fs::File::open(dir.as_path()).unwrap();
    let mut archive = zip::ZipArchive::new(zipfile).unwrap();

    dir.pop();
    println!("extracting to {:?}", dir);
    archive.extract(dir.as_path());
}
