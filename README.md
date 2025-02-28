# elsewhere

Keep track of the current time where other folks are


# Usage: 

```
$ ew help
Elsewhere `ew` helps keep track of what time it is elsewhere

Usage: ew [COMMAND]

Commands:
  add   Add a new person to the `ew` config
  rm    Remove a person from the `ew` config by name
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```


## Build or Install

Building with `cargo` is the currently supported build method. If you do not have `cargo` installed,
checkout [rustup.rs](https://rustup.rs/). After installing rust, you can clone this project and build it:

```bash
$ git clone git@github.com:holmandw/elsewhere.git
$ cd elsewhere
# to run without installing
$ cargo run -- --help
# displays currently configured users
# otherwise, install it with
$ cargo install --path .
# then it can be used from anywhere in your terminal like
$ ew --help
```


## Example Usage:

```
# start fresh
$ rm .ew.toml
$ ew help
Elsewhere `ew` helps keep track of what time it is elsewhere

Usage: ew [COMMAND]

Commands:
  add   Add a new person to the `ew` config
  rm    Remove a person from the `ew` config by name
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
$ ew add Sara America/Chicago
$ ew add Joe America/New_York
$ ew add Addison America/Los_Angeles
$ ew
Addison    15:10 -0800    Thu Feb 27 2025
Joe        18:10 -0500    Thu Feb 27 2025
Sara       17:10 -0600    Thu Feb 27 2025
$ ew rm Joe
$ ew
Addison    15:10 -0800    Thu Feb 27 2025
Sara       17:10 -0600    Thu Feb 27 2025
$ cat .ew.toml
[[person]]
name = "Addison"
tz = "America/Los_Angeles"

[[person]]
name = "Sara"
tz = "America/Chicago"
```

NB: names can be > 1 word, they just need to be surrounded by quotes.
e.g. `ew add "Bob Belcher" US/Eastern`


# Features

* stores config in `$HOME/.ew.toml`
* sorts the config and printed values by name


# TODO/Ideas

* eliminate usage of `unwrap` and `expect`
* support different location for toml config file. e.g. configurable based on env var
* make sorting optional to honor insert order, either via config file or env var
* make `strftime` configurable, either via config file or env var

