# Generated by extendr: Do not edit by hand

# nolint start

#
# This file was created with the following call:
#   .Call("wrap__make_fastrls_wrappers", use_symbols = TRUE, package_name = "fastrls")

#' @usage NULL
#' @useDynLib fastrls, .registration = TRUE
NULL

walkdir <- function(dir, include_dirs, include_files, include_links, follow_links) .Call(wrap__walkdir, dir, include_dirs, include_files, include_links, follow_links)

walkdirp <- function(dir, include_dirs, include_files, include_links, follow_links, n_threads) .Call(wrap__walkdirp, dir, include_dirs, include_files, include_links, follow_links, n_threads)


# nolint end
