# ojpoker | Updated: November 28, 2024

This is code for the [OneJoker](https://onejoker.org) project,
aiming to create libraries and other digital tools for handling playing cards
and card games in general, and more spcifically poker and its many variants.
I am writing it to provide some advantages over other libraries I have
encountered on the net:

- Completeness: Card-handling code covers things that others don't like
  jokers and foreign decks. Poker code handles more variants: high hands,
  ace-to-five lowball, deuce-to-seven lowball, London lowball, pai gow,
  action razz, badugi, badeucy, stripped decks, bugs, etc. I plan to add
  bindings for other languages.
- Correctness: Many libraries do not correctly handle things like lowball
  and badugi hands, or betting limits and procedures. Author is a long-time
  poker player and casino manager with extensive knowledge of the rules,
  and implements them carefully.
- Performance: Poker code is *fast*, taking full advantage of modern 64-bit
  machines and the latest known algorithms. It can evaluate millions of hands
  per second in a single thread on modest hardware. You can choose from
  different versions of each evaluator to best suit your needs.

This codebase is primarily a [Rust](https://rust-lang.org) library, but
will include bindings for other languages such as TypeScript and Python.
(Library documentation [here](https://lcrocker.github.io/ojpoker/)).

# Requirements

If you already have the Rust development environment and the
[Deno](https://deno.com) typescript runtime on your machine, you can jump
right in and run the build and test scripts:
```
./clean_build_all.ts
./test_all.ts
```

## Rust

If you are only interested in the Rust code, you can go to the `rust`
directory and use the usual commands: `cargo build`, `cargo test`, etc.
I am currently using Rust/Cargo 1.81.0.

## TypeScript

I am using Deno 2.0.6 (TypeScript 5.6.2) for the scripts, though
most older versions should work.
Deno scripts are used for automating build tasks and building and
converting data files.

TypeScript language binding coming soon...

## Python

Language binding coming soon...

# Lookup table files

The poker code can optionally use large binary lookup tables to speed up
its hand evaluations.
These are not checked into the repo.
The [Releases](https://githib.com/lcrocker/ojpoker/releases) area contains
these files, along with a README that explains how to use them.

## Card images

Inside the `data` directory is the `cards` directory which contains data
and Deno scripts for building [SVG](https://en.wikipedia.org/wiki/SVG)
playing card images.
These can be easily customized to the needs of your software.
You can see a sample [here](https://onejoker.org/images).

# More information

More detailed documentation and dicussion of the code can be found on
the [ojpoker wiki](https://github.com/lcrocker/ojpoker/wiki) on GitHub.
Other resources are available at the
[OneJoker](https://onejoker.org) website.
