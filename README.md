
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

print(microbenchmark(
    base = dir(".", recursive = TRUE, all.files = TRUE, include.dirs = TRUE),
    fs = fs::dir_ls(".", recurse = TRUE, all = TRUE, fail = FALSE),
    fastrls = fastrls::fastrls(".", n_threads = 4L),

    times = 10L, control = list(warmup = 1L), setup = clear_cache()), signif = 4)
```

    Unit: milliseconds
        expr    min     lq   mean median     uq  max neval cld
        base 4679.0 4727.0 4791.0 4742.0 4771.0 5206    10 a
          fs 1535.0 1583.0 1606.0 1610.0 1635.0 1684    10  b
     fastrls  597.2  610.7  616.7  613.3  623.8  634    10   c
