use std::{
    env,
    path::PathBuf,
    fs::{File, create_dir_all},
    io,
    cmp::{min},
    thread,
    time::Duration
};

use fltk::{
    image,
    app::{
        add_handler,
        Sender
    },
    dialog::{
        NativeFileChooser,
        NativeFileChooserType,
        // NativeFileChooserOptions
    },
    enums::Event,
    window::DoubleWindow,
    prelude::{
        WidgetExt,
        WindowExt
    },
};

use reqwest::{
    blocking as req_blocking,
    header as headers
};

use zip::ZipArchive;

use crate::{
    errors::{
        InstallerError,
        DownloadError,
        ExtractionError
    },
    Message
};


/// Multiplies int by float and returns int
/// Useful to position widgets relatively of the windows size
// pub fn mul_int_float(a: i32, b: f32) -> i32 {
//     return (a as f32 * b) as i32
// }


/// Changes current active windows by hiding one window and showing another
pub fn switch_win(windows: &mut Vec<DoubleWindow>, current_id: &mut usize, new_id: usize) {
    // Sanity check
    if *current_id >= windows.len() || new_id >= windows.len() {
        return
    }
    windows[*current_id].hide();
    windows[new_id].show();
    *current_id = new_id;
}


/// Loads icon data and sets it as window icon
pub fn load_icon(win: &mut DoubleWindow) {
    let icon = image::PngImage::from_data(&crate::APP_ICON_DATA);
    win.set_icon(icon.ok());
}

/// Disables global hotkeys by consuming all shortcut events
pub fn disable_global_hotkeys() {
    add_handler(
        |ev| {
            return match ev {
                Event::Shortcut => true,
                _ => false
            }
        }
    );
}


/// Returns current working dir
pub fn get_cwd() -> PathBuf {
    let cwd = env::current_dir();
    return cwd.ok().unwrap_or_default()
}

/// Launches select directory dialogue native to the target OS
/// returns selected directory, defaults to current working directory
pub fn run_select_dir_dlg(prompt: &str) -> PathBuf {
    let mut c = NativeFileChooser::new(NativeFileChooserType::BrowseDir);

    c.set_title(prompt);

    let cwd = get_cwd();
    match c.set_directory(&cwd) {
        Err(err) => eprintln!("Failed to automatically set default dir: {err}"),
        Ok(_) => {}
    }

    c.show();

    return c.filename()
}


/// Builds a client for this installer to access GitHub API
pub fn build_client() -> Result<req_blocking::Client, InstallerError> {
    use headers::HeaderValue;

    let mut headers = headers::HeaderMap::new();
    headers.append(headers::USER_AGENT, HeaderValue::from_static("Monika After Story Installer"));
    headers.append(headers::ACCEPT_CHARSET, HeaderValue::from_static("utf8"));
    headers.append(headers::ACCEPT_LANGUAGE, HeaderValue::from_static("en-US"));
    headers.append(headers::CONTENT_LANGUAGE, HeaderValue::from_static("en-US"));

    let client = req_blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    return Ok(client)
}


/// Returns tuple of two links to the main assets:
/// defaul version download and deluxe version download
fn _get_assets_links(client: &req_blocking::Client) -> Result<(String, String), InstallerError> {
    let data = client.get(
        format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            crate::ORG_NAME,
            crate::REPO_NAME
        )
    ).send()?.bytes()?;

    let json_data: serde_json::Value = serde_json::from_slice(&data)?;
    let assets_list = json_data.get("assets").ok_or(InstallerError::CorruptedJSON("missing the assets field"))?;

    let def_link = assets_list.get(crate::DEF_VERSION_ASSET_ID).ok_or(InstallerError::CorruptedJSON("missing the def version asset"))?
        .get("browser_download_url").ok_or(InstallerError::CorruptedJSON("missing the def version download link field"))?
        .as_str().ok_or(InstallerError::CorruptedJSON("couldn't parse link to a str"))?
        .to_owned();
    let dlx_link = assets_list.get(crate::DLX_VERSION_ASSET_ID).ok_or(InstallerError::CorruptedJSON("missing the deluxe version asset"))?
        .get("browser_download_url").ok_or(InstallerError::CorruptedJSON("missing the dlx version download link field"))?
        .as_str().ok_or(InstallerError::CorruptedJSON("couldn't parse link to a str"))?
        .to_owned();

    return Ok((def_link, dlx_link))
}

/// Downloads data from the given link using the provided client
/// the data is being written into the given file handler
fn _download_to_file(client: &req_blocking::Client, download_link: &String, file: &mut File) -> Result<(), DownloadError> {
    const DEF_CHUNK_SIZE: u128 = 1024*1024*8 + 1;

    let resp = client.head(download_link).send()?;
    let content_size = resp.headers().get(headers::CONTENT_LENGTH).ok_or(DownloadError::InvalidContentLen)?
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
            return Err(DownloadError::InvalidStatusCode(status_code))
        }

        // Write the received data
        let received_chunk = resp.copy_to(file)? as u128;
        total_downloaded += received_chunk;

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
        thread::sleep(Duration::from_secs_f32(0.25));
    }

    // println!("Total downloaded: {}", total_downloaded);

    return Ok(())
}

/// Extracts a zip archive
fn _extract_archive(archive: &File, destination_dir: &PathBuf) -> Result<(), ExtractionError> {
    let mut archive = ZipArchive::new(archive)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        let file_path = file.enclosed_name()
            .ok_or(ExtractionError::UnsafeFilepath(file.name().to_string()))?;

        let extraction_path = destination_dir.join(file_path);

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
    }
    return Ok(())
}

/// Creates a temp dir for the installer temp data
fn _create_temp_dir() -> Result<tempfile::TempDir, io::Error> {
    return tempfile::Builder::new()
        .prefix(".mas_installer-")
        .tempdir()
}

/// Creates a temp file for the installer data
fn _create_temp_file(temp_dir: &tempfile::TempDir) -> Result<File, io::Error> {
    let fp = temp_dir.path().join("temp");
    return File::options()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open(&fp)
}

/// Main method to handle game installation process, downloads it into a temp folder and then extracts
pub fn install_game(
    destination_dir: &PathBuf,
    sender: Sender<Message>,
    is_deluxe: bool
) -> Result<(), InstallerError> {
    // Mark as downloading asap
    sender.send(Message::Downloading);

    let client = build_client()?;

    // Get download link
    let (def, dlx) = _get_assets_links(&client)?;
    let download_link = match is_deluxe {
        true => dlx,
        false => def
    };
    // let download_link = String::from("https://github.com/Monika-After-Story/MonikaModDev/releases/download/v0.12.9/spritepacks-combined.zip");

    // Create temp structures
    let temp_dir = _create_temp_dir()?;
    let mut temp_file = _create_temp_file(&temp_dir)?;

    // Install
    _download_to_file(&client, &download_link, &mut temp_file)?;
    sender.send(Message::Extracting);
    _extract_archive(&temp_file, destination_dir)?;
    sender.send(Message::Done);

    drop(temp_file);

    return Ok(())
}

/// Threaded version of install_game
pub fn install_game_in_thread(
    destination_dir: &PathBuf,
    sender: Sender<Message>,
    is_deluxe: bool
) {
    // use std::sync::{
    //     Mutex,
    //     Arc
    // };
    // let client = Arc::new(Mutex::new(client));
    // let download_link = Arc::new(Mutex::new(download_link));
    // let destination_dir = Arc::new(Mutex::new(destination_dir));
    // let client = client.clone();
    // let download_link = download_link.clone();
    let destination_dir = destination_dir.clone();
    let sender = sender.clone();
    thread::spawn(
        move || -> Result<(), InstallerError> {
            // let c = client.lock().unwrap();
            // let dl = download_link.lock().unwrap();
            // let dr = destination_dir.lock().unwrap();
            // install_game(*c, *dl, *dr, sender).unwrap();
            install_game(&destination_dir, sender, is_deluxe)?;
            return Ok(())
        }
    );
}
