/// The module that implements installer logic for IO handling

use std::{
    path::Path,
    fs::{File, create_dir_all},
    io,
    cmp::min,
    thread,
    time::Duration
};

use fltk::app::{
    Sender
};

use reqwest::{
    blocking as req_blocking,
    header as headers
};

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

/// Struct representing release data we may need
/// (like download links)
struct ReleaseData {
    def_dl_link: String,
    dlx_dl_link: String,
    spr_dl_link: String
}


/// Blocks the thread for PAUSE_DURATION seconds
fn sleep() {
    thread::sleep(PAUSE_DURATION);
}


/// Builds a client for this installer to access GitHub API
pub fn build_client() -> Result<req_blocking::Client, InstallError> {
    use headers::HeaderValue;

    let mut headers = headers::HeaderMap::new();
    headers.append(headers::USER_AGENT, HeaderValue::from_static("Monika After Story Installer"));
    headers.append(headers::ACCEPT_CHARSET, HeaderValue::from_static("utf8"));
    headers.append(headers::ACCEPT_LANGUAGE, HeaderValue::from_static("en-US"));
    headers.append(headers::CONTENT_LANGUAGE, HeaderValue::from_static("en-US"));

    let client = req_blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    return Ok(client);
}


/// Requests release data from github
fn get_release_data(client: &req_blocking::Client) -> Result<ReleaseData, InstallError> {
    const DL_URL_KEY: &str = "browser_download_url";

    let data = client.get(
        format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            crate::ORG_NAME,
            crate::REPO_NAME
        )
    ).send()?.bytes()?;

    let json_data: serde_json::Value = serde_json::from_slice(&data)?;
    let assets_list = json_data.get("assets").ok_or(InstallError::CorruptedJSON("missing the assets field"))?;

    let def_dl_link = assets_list.get(crate::DEF_VERSION_ASSET_ID).ok_or(InstallError::CorruptedJSON("missing the def version asset"))?
        .get(DL_URL_KEY).ok_or(InstallError::CorruptedJSON("missing the def version download link field"))?
        .as_str().ok_or(InstallError::CorruptedJSON("couldn't parse link to a str"))?
        .to_owned();
    let dlx_dl_link = assets_list.get(crate::DLX_VERSION_ASSET_ID).ok_or(InstallError::CorruptedJSON("missing the deluxe version asset"))?
        .get(DL_URL_KEY).ok_or(InstallError::CorruptedJSON("missing the dlx version download link field"))?
        .as_str().ok_or(InstallError::CorruptedJSON("couldn't parse link to a str"))?
        .to_owned();
    let spr_dl_link = assets_list.get(crate::SPR_ASSET_ID).ok_or(InstallError::CorruptedJSON("missing spritepack asset"))?
        .get(DL_URL_KEY).ok_or(InstallError::CorruptedJSON("missing the spritepacks download link field"))?
        .as_str().ok_or(InstallError::CorruptedJSON("couldn't parse link to a str"))?
        .to_owned();

    let data = ReleaseData {
        def_dl_link,
        dlx_dl_link,
        spr_dl_link
    };
    return Ok(data);
}


/// Downloads data from the given link using the provided client
/// the data is being written into the given file handler
fn download_to_file(
    client: &req_blocking::Client,
    sender: Sender<Message>,
    app_state: &ThreadSafeState,
    download_link: &str,
    file: &mut File
) -> Result<(), DownloadError> {
    const DEF_CHUNK_SIZE: u128 = 1024*1024*8 + 1;

    sender.send(Message::UpdateProgressBar(0.0));

    if app_state.lock().unwrap().get_abort_flag() {
        return Ok(());
    }

    let resp = client.head(download_link).send()?;
    let content_size = resp.headers().get(headers::CONTENT_LENGTH)
        .ok_or(DownloadError::InvalidContentLen)?
        .to_str().ok().ok_or(DownloadError::InvalidContentLen)?
        .parse::<u128>().ok().ok_or(DownloadError::InvalidContentLen)?;

    let chunk_size: u128 = min(DEF_CHUNK_SIZE, content_size);
    let mut low_bound: u128 = 0;
    let mut up_bound: u128 = chunk_size;
    let mut total_downloaded: u128 = 0;

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
        let received_chunk = resp.copy_to(file)? as u128;
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
    let download_link = match app_state.lock().unwrap().get_deluxe_ver_flag() {
        true => data.dlx_dl_link,
        false => data.def_dl_link
    };
    // let download_link = String::from("https://github.com/Monika-After-Story/MonikaModDev/releases/download/v0.12.9/spritepacks-combined.zip");
    let destination = app_state.lock().unwrap().get_extraction_dir().clone();

    sender.send(Message::UpdateProgressBar(0.5));
    sleep();

    // Create temp structures
    let temp_dir = create_temp_dir()?;
    let mut mas_temp_file = create_temp_file(&temp_dir, "mas.tmp")?;
    let mut spr_temp_file = create_temp_file(&temp_dir, "spr.tmp")?;

    sender.send(Message::UpdateProgressBar(1.0));
    sleep();

    // Install MAS
    sender.send(Message::Downloading);
    download_to_file(
        &client,
        sender,
        app_state,
        &download_link,
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
        &data.spr_dl_link,
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
