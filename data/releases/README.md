# Large data files

These files are large compressed binary files used as lookup
tables for poker hand evaluation.
They are not checked into the repo, but are available from the
[releases](https://github.com/lcrocker/ojpoker/releases) area.

To enable the use of these tables for a game, copy the tables
for that game (there are two each) to `$CARGO_HOME/onejoker/`,
then enable the corresponding feature in Cargo.toml.

For example, to enable fast high-hand lookups, you'll first need
to copy the `ojp_hh\*` files to `~/.cargo/onejoker`.
Do not uncompress them!
The library will do that at runtime.
Finally, enable the `"high-hand-tables"` feature in `Cargo.toml`.

