#' Recursively list files and optionally directories, following symlinks
#' @param dir string: starting directory
#' @param include_dirs logical: if TRUE, include directories in the results. Note that excluding directories is significantly slower
#' @param multithreaded logical: if TRUE, use multiple threads. Might be counter-productive on hardware that can't cope with simultaneous requests (spinning disks?)
#' @return A character vector
#' @export
fastrls <- function(dir, include_dirs = TRUE, multithreaded = TRUE) {
    out <- if (isTRUE(multithreaded)) .Call(wrap__walkdirp, dir, isTRUE(include_dirs)) else .Call(wrap__walkdir, dir, isTRUE(include_dirs))
    out[nzchar(out)]
}
