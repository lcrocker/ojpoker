# Large data files

This directory contains large files not checked into the repo.
You can download these from
[releases](https://github.com/lcrocker/ojpoker/releases).

These are large tables used to speed up poker hand evaluation.
You can choose to enable any or all of them by enabling the
corresponding feature in Cargo.toml, and copying files from here
to the appropriate directories.

For example, to enable fast high-hand lookups, you'll first need
to copy the source file `high_tables.rs` into the directory
`rust/src/poker/games`. That file has a function that loads the
large binary tables from `~/.cargo/onejoker`, so copy the binaries
`high_\*.bin.gz` to there (or change the source to look for them
somewhere else).
Finally, enable the `"high-hand-tables"` feature in `Cargo.toml`.
Each game has its own feature and its own set of files.

