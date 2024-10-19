# ojpoker | Updated: October 19, 2024

This is code for the [OneJoker](https://onejoker.org) project,
aiming to create libraries and other digital tools for handling playing cards
and card games in general, and more spcifically poker and its many variants.
I am writing it to provide some advantages over other libraries I have
encountered on the net:

- Completeness: Card-handling code covers things that others don't like
  jokers and foreign decks. Poker code handles more variants. The two
  initial languages (Dart and Rust) cover many application areas, and I
  plan to add more languages.
- Correctness: Many libraries do not correctly handle things like lowball
  and badugi hands, or betting limits and procedures. Author is a long-time
  poker player and casino manager with extensive knowledge of the rules,
  and implements them carefully.
- Performance: Poker code is *fast*, taking full advantage of modern 64-bit
  machines. It does sacrifice memory efficiency for this in some cases.

This codebase produces two separate libraries in [Rust](https://rust-lang.org)
and [Dart](https://dart.dev) (library documentation
[here](https://lcrocker.github.io/ojpoker/)).

# Requirements

If you already have the Dart and Rust development environments and the
[Deno](https://deno.com) typescript runtime on your machine, you can jump
right in and run the build and test scripts:
```
./clean_build_all.ts
./test_all.ts
```

## Rust

If you are only interested in the Rust code, you can go to the `rust`
directory and use the usual commands: `cargo build`, `cargo test`, etc.
I am current using Rust/Cargo 1.80.0.

At least one source file is built by script. These are checked into the
repo so you won't have to build them, but if you want to make changes,
you'll have to install Deno to rebuild them.

## Dart

If you are only interested in the Dart code, you can go to the `dart`
directory and use the usual commands: `dart run`, `dart test`, etc.
I'm currently using Dart 3.5.3. 
The codebase does not use Flutter, so you can install Flutter or not
as you prefer.

At least one source file is built by script. These are checked into the
repo so you won't have to build them, but if you want to make changes,
you'll have to install Deno to rebuild them.

## TypeScript

I am using Deno 2.0.0 (TypeScript 5.6.2) for the scripts, though
most older versions should work.
Deno scripts are used for automating build tasks, building and
converting data files, and building some source files.

# Data files

I use many serialized data sets for things like test data and pre-computed
lookup tables. The smaller and more necessary ones are checked into the repo
in the `data` directory along with conversion scripts.
[JSONC](https://code.visualstudio.com/docs/languages/json#_json-with-comments)
format is used for ease of human editing and
[MessagePack](https://msgpack.org) binary format for performance.
The very large ones (such as the poker evaluator lookup tables) are not in
the repo, but can be downloaded from the repo's
[Releases](https://githib.com/lcrocker/ojpoker/releases) area.

# More information

More detailed documentation and dicussion of the code can be found on
the [ojpoker wiki](https://github.com/lcrocker/ojpoker/wiki) on GitHub.
Other resources are available at the
[OneJoker](https://onejoker.org) website.
