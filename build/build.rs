#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("src/static/icon.ico");
    res.compile().unwrap();// I want it to panic if it fails
}

#[cfg(not(windows))]
fn main() {
}
