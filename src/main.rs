use clap::*;
use google_drive;
use tokio;
// use open;
use home::home_dir;
use std::fs;
use std::path;
use zip;
use std::process::Command;
use std::os::unix::fs::PermissionsExt;

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
    Download { appname: String },
    Upload { appname: String },
    Run { appname: String },
    List,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Initialize Google Drive Client
    let client = init_drive().await?;

    // get user's home dir
    let home_dir = match home_dir() {
        Some(path) => path,
        None => panic!("Impossible to get your home dir!"),
    };

    match &cli.command {
        Commands::Download { appname } => {
            match appname.as_str() {
                "firefox" => config_firefox(&client, &home_dir).await?,
                _ => println!("please enter a valid app name"), 
            }
        },
        Commands::Upload { appname } => {
            match appname.as_str() {
                "firefox" => println!("will upload firefox"),
                _ => (),
            }
        },
        Commands::Run { appname } => {
            match appname.as_str() {
                "firefox" => {
                    Command::new("./firefox.appimage")
                            .args(["--appimage-extract-and-run"]);
                    },
                _ => (),
            }
        }
        Commands::List => {
            print_files_in_initr_folder(&client).await;
        }
    }
    Ok(())
}

async fn config_firefox(client: &google_drive::Client, dir: &path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut dir = dir.clone();
    dir.push("firefox.appimage");
    if !dir.exists() {
        println!("Downloading Firefox Appimage to {:?}", dir);
        download_file_to(client, FIREFOX_APPIMAGE_ID, dir.as_path()).await?;
        fs::set_permissions(dir.as_path(), fs::Permissions::from_mode(0o755))?;
    } else {
        println!("Skipping downloading Appimage");
    }
    dir.pop();

    
    dir.push("mozilla.zip");
    if !dir.exists() {
        print!("Downloading Firefox configs");
        download_file_to(client, FIREFOX_CONFIGS_ID, dir.as_path()).await?;
    } else {
        print!("Skipping downloading Firefox configs");
    }

    let zipfile = fs::File::open(dir.as_path()).unwrap();
    let mut archive = zip::ZipArchive::new(zipfile).unwrap();

    dir.pop();

    println!("Extracting \"mozilla.zip\" to {:?}", dir);
    archive.extract(dir.as_path())?;
    Ok(())
}
