# kitsu.rs examples

This directory contains two examples, one example for each of the supported
HTTP clients.

All examples work by first asking you to provide an anime name which will be
used to search and return information about that anime.

To compile and run these examples, clone this repository and `cd` into the
approriate example relevant to your HTTP client requirements, like so:

```sh
$ git clone https://github.com/zeyla/kitsu.rs
$ cd kitsu.rs/examples/01_reqwest
```

Then run the project, this will compile the project as well as download and
compile all it dependencies, and finally run it:

```sh
$ cargo run
```