
<!-- README.md is generated from README.Rmd. Please edit that file -->

# fastrls

<!-- badges: start -->

<!-- badges: end -->

Experiments in using Rust for faster directory listing.

## Installation

Note that you need [Rust installed on your
system](https://www.rust-lang.org/tools/install) first. Then:

``` r
remotes::install_github("AustralianAntarcticDivision/fastrls")
```

## Benchmark

On a test directory containing about 100k files in total (Ubuntu 20.04,
PC601 NVMe SK Hynix 1TB SSD):

``` r
library(microbenchmark)

## clear disk cache before each test is run
clear_cache <- function() system2("sudo", "sh -c 'sync; echo 3 > /proc/sys/vm/drop_caches'")

microbenchmark(
    base = dir("~/temp/", recursive = TRUE, all.files = TRUE, include.dirs = TRUE),
    fs = fs::dir_ls(".", recurse = TRUE, all = TRUE, fail = FALSE),
    fastrls = fastrls::fastrls(".", n_threads = 4L),

    times = 10L, control = list(warmup = 1L), setup = clear_cache())
```

    Unit: milliseconds
        expr       min        lq      mean    median        uq       max neval cld
        base 4715.6781 4757.8224 4816.8141 4809.9948 4833.9616 4976.0532    10 a
          fs 1545.2190 1575.2991 1657.5119 1603.7403 1657.4901 1969.0511    10  b
     fastrls  596.5576  609.0954  614.6693  614.6403  622.1317  627.5728    10   c
