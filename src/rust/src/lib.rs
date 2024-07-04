use extendr_api::prelude::*;

#[extendr]
fn walkdir(dir: String, include_dirs: bool) -> Vec<String> {
    walkdir::WalkDir::new(dir).follow_links(true).into_iter().filter_map(|e| e.ok())
        .map(|e| {
            if include_dirs || !e.metadata().unwrap().is_dir() {
                e.into_path().into_os_string().into_string().unwrap()
            } else {
                "".to_string()
            }
        }).collect()
}

// multithreaded using jwalk
#[extendr]
fn walkdirp(dir: String, include_dirs: bool) -> Vec<String> {
    jwalk::WalkDir::new(dir).follow_links(true).skip_hidden(false).into_iter()
        .filter_map(|e| e.ok()).map(|e| {
            if include_dirs || !e.metadata().unwrap().is_dir() { // including dirs or it's not a dir
                e.path().to_str().unwrap().to_string()
            } else {
                "".to_string()
            }
        }).collect()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod fastrls;
    fn walkdir;
    fn walkdirp;
}
