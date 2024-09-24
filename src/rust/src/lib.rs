use extendr_api::prelude::*;

// single threaded using walkdir. Presumably could also do this with jwalk using jwalk::Parallelism::Serial if n_threads == 1
#[extendr]
fn walkdir(dir: String, include_dirs: bool, include_files: bool, include_links: bool, follow_links: bool) -> Vec<String> {
    walkdir::WalkDir::new(dir).follow_links(follow_links).into_iter().filter_map(|e| e.ok())
        .map(|e| {
            if (include_dirs && e.file_type().is_dir()) || (include_links && e.file_type().is_symlink()) || (include_files && e.file_type().is_file()) {
                e.into_path().into_os_string().into_string().unwrap()
            } else {
                "".to_string()
            }
        }).collect()
}

// multithreaded using jwalk
#[extendr]
fn walkdirp(dir: String, include_dirs: bool, include_files: bool, include_links: bool, follow_links: bool, n_threads: usize) -> Vec<String> {
    let prl = jwalk::Parallelism::RayonNewPool(n_threads);
    jwalk::WalkDir::new(dir).follow_links(follow_links).skip_hidden(false).parallelism(prl).into_iter()
        .filter_map(|e| e.ok()).map(|e| {
            if (include_dirs && e.file_type().is_dir()) || (include_links && e.file_type().is_symlink()) || (include_files && e.file_type().is_file()) {
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
