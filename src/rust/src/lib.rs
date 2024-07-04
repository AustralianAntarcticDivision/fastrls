use extendr_api::prelude::*;

// single threaded using walkdir. Presumably could also do this with jwalk using jwalk::Parallelism::Serial if n_threads == 1
#[extendr]
fn walkdir(dir: String, include_dirs: bool) -> Vec<String> {
    walkdir::WalkDir::new(dir).follow_links(true).into_iter().filter_map(|e| e.ok())
        .map(|e| {
            if include_dirs || !e.file_type().is_dir() {
                e.into_path().into_os_string().into_string().unwrap()
            } else {
                "".to_string()
            }
        }).collect()
}

// multithreaded using jwalk
#[extendr]
fn walkdirp(dir: String, include_dirs: bool, n_threads: usize) -> Vec<String> {
    let prl = jwalk::Parallelism::RayonNewPool(n_threads);
    jwalk::WalkDir::new(dir).follow_links(true).skip_hidden(false).parallelism(prl).into_iter()
        .filter_map(|e| e.ok()).map(|e| {
            if include_dirs || !e.file_type().is_dir() { // including dirs or it's not a dir
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
