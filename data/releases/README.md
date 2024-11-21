# Large data files

This directory contains large files not checked into the repo.
You can download these from
[releases](https://github.com/lcrocker/ojpoker/releases).

These are large tables used to speed up poker hand evaluation.
You can choose to enable any or all of them by enabling the
corresponding feature in Cargo.toml, and copying files from here
to the appropriate directories.

For example, to enable fast high-hand lookups, enable the
`"high-hand-tables"` feature in `Cargo.toml`, then copy the source
file `high_hand_tables.rs` into the `src/poker` directory, and
finally copy the two `ojp_high_mp\*` files to `~/.cargo/onejoker`
(or change that directory in the source file).

