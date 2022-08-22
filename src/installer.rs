/// The module that implements installer logic for IO handling

use std::{
    path::Path,
    fs::{File, create_dir_all, read_dir, remove_file},
    io,
    cmp::min,
    collections::HashMap,
    thread,
    time::Duration
};

use fltk::app::{
    Sender
};

use ::reqwest::{
    blocking as reqwest,
    header as headers
};

use serde::{Serialize, Deserialize};

use zip::ZipArchive;

use crate::{
    app::{
        state::ThreadSafeState,
        Message
    },
    errors::{
        InstallError,
        DownloadError,
        ExtractionError
    }
};


const PAUSE_DURATION: Duration = Duration::from_millis(200);


pub type InstallResult = Result<(), InstallError>;
pub type ContentSize = u64;

/// Struct representing release data we may need
#[derive(Debug)]
#[allow(dead_code)]
struct ReleaseData {
    version: String,
    name: String,
    def_ver_asset: GHAsset,
    dlx_ver_asset: GHAsset,
    spr_asset: GHAsset
}

impl ReleaseData {
    /// Creates new release data
    pub fn new(
        version: String,
        name: String,
        def_ver_asset: GHAsset,
        dlx_ver_asset: GHAsset,
        spr_asset: GHAsset
    ) -> Self {
        return Self { version, name, def_ver_asset, dlx_ver_asset, spr_asset };
    }
}

/// Represents an attachment in a GitHub release
#[derive(Serialize, Deserialize, Debug)]
struct GHAsset {
    name: String,
    size: ContentSize,
    browser_download_url: String
}

impl GHAsset {
    /// Check if this asset is valid
    pub fn is_valid(&self) -> bool {
        return {
            !self.name.is_empty()
            && self.size != 0
            && !self.browser_download_url.is_empty()
            && self.browser_download_url.starts_with("https://")
            && self.browser_download_url.ends_with(".zip")
        };
    }
}

/// Represents a GitHub release
#[derive(Serialize, Deserialize, Debug)]
struct GHRelease {
    tag_name: String,
    name: String,
    assets: Vec<GHAsset>
}

impl GHRelease {
    /// Check if this release is valid
    pub fn is_valid(&self) -> bool {
        return {
            !self.tag_name.is_empty()
            && !self.name.is_empty()
            && !self.assets.len() != 0
        };
    }
}


/// Blocks the thread for PAUSE_DURATION seconds
fn sleep() {
    thread::sleep(PAUSE_DURATION);
}


/// Builds a client for this installer to access GitHub API
pub fn build_client() -> Result<reqwest::Client, InstallError> {
    let headers = crate::HEADERS.clone();
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    return Ok(client);
}


/// Requests release data from github
fn get_release_data(client: &reqwest::Client) -> Result<ReleaseData, InstallError> {
    let data = client.get(
        format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            crate::ORG_NAME,
            crate::REPO_NAME
        )
    ).send()?.bytes()?;

    let release: GHRelease = serde_json::from_slice(&data)?;
    if !release.is_valid() {
        eprintln!("Release '{:?}' is invalid", release);
        return Err(InstallError::CorruptedJSON("Latest release is invalid"));
    }

    // Create a map of the assets we need
    let mut assets_map = HashMap::new();

    // Search thru all the available assets and find the ones we need
    'outer_loop: for asset in release.assets {
        // Quit early if we filled the map
        if assets_map.len() == crate::ASSETS_NAMES_RE_MAP.len() {
            break;
        }
        // Use regex to find the assets
        for (k, v) in crate::ASSETS_NAMES_RE_MAP.iter() {
            if !assets_map.contains_key(k) && v.is_match(&asset.name) {
                if !asset.is_valid() {
                    eprintln!("Asset '{}' is invalid", asset.name);
                    return Err(InstallError::CorruptedJSON("Found a required asset, but it's invalid"));
                }
                assets_map.insert(*k, asset);
                // We need to move to the next asset since this once has been moved
                continue 'outer_loop;
            }
        }
    }

    if assets_map.len() != crate::ASSETS_NAMES_RE_MAP.len() {
        return Err(InstallError::CorruptedJSON("An asset is missing from the release"));
    }

    let data = ReleaseData::new(
        release.tag_name,
        release.name,
        assets_map.remove("def_ver").unwrap(),
        assets_map.remove("dlx_ver").unwrap(),
        assets_map.remove("spr").unwrap()
    );
    return Ok(data);
}


/// Unlinks rpy and rpyc files on the given path
/// This function is "best-effort" and will silently ignore errors
fn remove_rpy(path: &Path) {
    if !path.is_dir() {
        return;
    }

    let content = read_dir(path);
    if content.is_err() {
        return;
    }

    let content = content.unwrap();
    for item in content {
        if let Ok(item) = item {
            let item_path = item.path();
            if !item_path.is_file() {
                continue;
            }
            let ext = item_path.extension();
            if let Some(ext) = ext {
                let ext = ext.to_str();
                if ext.is_none() {
                    continue;
                }
                let ext = ext.unwrap();
                match ext {
                    "rpy" | "rpyc" => {
                        if remove_file(&item_path).is_err() {
                            eprintln!("Failed to delete '{}'", item_path.display());
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}


/// Tries to query content len on the given link
fn get_content_size(client: &reqwest::Client, download_link: &str) -> Result<ContentSize, DownloadError> {
    let resp = client.head(download_link).send()?;
    let content_size = resp.headers().get(headers::CONTENT_LENGTH)
        .ok_or(DownloadError::InvalidContentLen)?
        .to_str().ok().ok_or(DownloadError::InvalidContentLen)?
        .parse::<ContentSize>().ok().ok_or(DownloadError::InvalidContentLen)?;
    return Ok(content_size);
}

/// Downloads data from the given link using the provided client
/// the data is being written into the given file handler
fn download_to_file(
    client: &reqwest::Client,
    sender: Sender<Message>,
    app_state: &ThreadSafeState,
    download_link: &str,
    content_size: Option<ContentSize>,
    file: &mut File
) -> Result<(), DownloadError> {
    const DEF_CHUNK_SIZE: ContentSize = 1024*1024*8 + 1;

    sender.send(Message::UpdateProgressBar(0.0));

    if app_state.lock().unwrap().get_abort_flag() {
        return Ok(());
    }

    let content_size: ContentSize = match content_size {
        None => get_content_size(client, download_link)?,
        Some(v) => v
    };

    let chunk_size: ContentSize = min(DEF_CHUNK_SIZE, content_size);
    let mut low_bound: ContentSize = 0;
    let mut up_bound: ContentSize = chunk_size;
    let mut total_downloaded: ContentSize = 0;

    // println!("Content size: {}", content_size);
    loop {
        // println!("{}-{}", low_bound, up_bound-1);
        let mut resp = client
            .get(download_link)
            .header(headers::RANGE, format!("bytes={}-{}", low_bound, up_bound-1))
            .send()?;

        let status_code = resp.status();
        if !status_code.is_success() {
            return Err(DownloadError::InvalidStatusCode(status_code));
        }

        // Write the received data
        let received_chunk = resp.copy_to(file)? as ContentSize;
        total_downloaded += received_chunk;

        // Update progress bar
        if content_size != 0 {
            let pb_val = total_downloaded as f64 / content_size as f64;
            sender.send(Message::UpdateProgressBar(pb_val));
        }

        // Check if we're done
        if total_downloaded >= content_size {
            break
        }

        // In case the server returned less than we asked, we need to
        // ask for the missing bits, so adjust the chunk size here
        let bound_inc = min(received_chunk, chunk_size);
        // Increment the bounds
        low_bound += bound_inc;
        up_bound = min(up_bound+bound_inc, content_size+1);
        // Slep to let the server rest
        sleep();
        // See if we want to abort
        if app_state.lock().unwrap().get_abort_flag() {
            return Ok(());
        }
    }

    // println!("Total downloaded: {}", total_downloaded);

    return Ok(());
}


/// Extracts a zip archive
fn extract_archive(
    sender: Sender<Message>,
    app_state: &ThreadSafeState,
    archive: &File,
    destination: &Path
) -> Result<(), ExtractionError> {
    sender.send(Message::UpdateProgressBar(0.0));

    if app_state.lock().unwrap().get_abort_flag() {
        return Ok(());
    }

    let mut archive = ZipArchive::new(archive)?;
    let total_files = archive.len();

    for i in 0..total_files {
        let mut file = archive.by_index(i)?;

        let file_path = file.enclosed_name()
            .ok_or(ExtractionError::UnsafeFilepath(file.name().to_string()))?;

        let extraction_path = destination.join(file_path);

        // Extract the dir
        if file.is_dir() {
            create_dir_all(&extraction_path)?;
        }
        // Extract the file
        else {
            // Create the parent dir if needed
            if let Some(parent_dir) = extraction_path.parent() {
                if !parent_dir.exists() {
                    create_dir_all(parent_dir)?;
                }
            }
            // Create the file and write to it
            let mut outfile = File::create(&extraction_path)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // Update progres bar
        let pb_val = (i as f64 + 1.0) / total_files as f64;
        sender.send(Message::UpdateProgressBar(pb_val));

        // See if we want to abort
        if app_state.lock().unwrap().get_abort_flag() {
            return Ok(());
        }
    }
    return Ok(());
}


/// Creates a temp dir for the installer temp data
fn create_temp_dir() -> Result<tempfile::TempDir, io::Error> {
    return tempfile::Builder::new()
        .prefix(".mas_installer-")
        .tempdir();
}

/// Creates a temp file for the installer data
fn create_temp_file(temp_dir: &tempfile::TempDir, name: &str) -> Result<File, io::Error> {
    let fp = temp_dir.path().join(name);
    return File::options()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open(&fp);
}


/// This runs cleanup logic on SUCCESSFUL download
fn cleanup(sender: Sender<Message>, mas_temp_file: File, spr_temp_file: File) {
    sender.send(Message::CleaningUp);
    sender.send(Message::UpdateProgressBar(0.0));
    drop(mas_temp_file);
    drop(spr_temp_file);
    sleep();
    sender.send(Message::UpdateProgressBar(1.0));
    sleep();
    sender.send(Message::Done);
}


/// Main method to handle game installation process, downloads it into a temp folder and then extracts
pub fn install_game(
    sender: Sender<Message>,
    app_state: &ThreadSafeState
) -> InstallResult {
    sender.send(Message::Preparing);
    sender.send(Message::UpdateProgressBar(0.0));

    if app_state.lock().unwrap().get_abort_flag() {
        return Ok(());
    }

    let client = build_client()?;

    // Get download link
    let data = get_release_data(&client)?;
    let main_asset = match app_state.lock().unwrap().get_deluxe_ver_flag() {
        true => data.dlx_ver_asset,
        false => data.def_ver_asset
    };
    let destination = app_state.lock().unwrap().get_extraction_dir().clone();

    sender.send(Message::UpdateProgressBar(0.5));
    sleep();

    // Create temp structures
    let temp_dir = create_temp_dir()?;
    let mut mas_temp_file = create_temp_file(&temp_dir, "mas.tmp")?;
    let mut spr_temp_file = create_temp_file(&temp_dir, "spr.tmp")?;

    // Remove old rpy/rpyc
    // Yeah...some people have rpy in the base dir...
    remove_rpy(&destination);
    remove_rpy(&destination.join("game"));

    sender.send(Message::UpdateProgressBar(1.0));
    sleep();

    // Install MAS
    sender.send(Message::Downloading);
    download_to_file(
        &client,
        sender,
        app_state,
        &main_asset.browser_download_url,
        Some(main_asset.size),
        &mut mas_temp_file
    )?;
    if app_state.lock().unwrap().get_abort_flag() {
        return Ok(());
    }
    sleep();

    sender.send(Message::Extracting);
    extract_archive(
        sender,
        app_state,
        &mas_temp_file,
        &destination
    )?;
    if app_state.lock().unwrap().get_abort_flag() {
        return Ok(());
    }
    sleep();

    // Quit early if the user doesn't want spritepacks
    if !app_state.lock().unwrap().get_install_spr_flag() {
        cleanup(sender, mas_temp_file, spr_temp_file);
        return Ok(());
    }

    // Install spritepacks
    sender.send(Message::DownloadingSpr);
    download_to_file(
        &client,
        sender,
        app_state,
        &data.spr_asset.browser_download_url,
        Some(data.spr_asset.size),
        &mut spr_temp_file
    )?;
    if app_state.lock().unwrap().get_abort_flag() {
        return Ok(());
    }
    sleep();

    sender.send(Message::ExtractingSpr);
    extract_archive(
        sender,
        app_state,
        &spr_temp_file,
        &destination.join("spritepacks")
    )?;
    if app_state.lock().unwrap().get_abort_flag() {
        return Ok(());
    }
    sleep();

    cleanup(sender, mas_temp_file, spr_temp_file);

    return Ok(());
}

/// Threaded version of install_game
pub fn install_game_in_thread(
    sender: Sender<Message>,
    app_state: &ThreadSafeState
) -> thread::JoinHandle<InstallResult> {

    let app_state = app_state.clone();

    return thread::spawn(
        move || -> InstallResult {
            return match install_game(sender, &app_state) {
                Err(e) => {
                    sender.send(Message::Error);
                    Err(e)
                },
                Ok(_) => Ok(())
            };
        }
    );
}
