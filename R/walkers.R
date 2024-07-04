#' Recursively list files and optionally directories, following symlinks
#' @param dir string: starting directory
#' @param include_dirs logical: if TRUE, include directories in the results. Note that excluding directories is significantly slower
#' @param n_threads integer: if > 1, use multiple threads. Might be counter-productive on hardware that can't cope with simultaneous requests (spinning disks?)
#' @return A character vector
#' @export
fastrls <- function(dir, include_dirs = TRUE, n_threads = 2L) {
    out <- if (isTRUE(n_threads > 1)) .Call(wrap__walkdirp, dir, isTRUE(include_dirs), n_threads) else .Call(wrap__walkdir, dir, isTRUE(include_dirs))
    out[nzchar(out)]
}
