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
Addison    15:10 -0800	Thu Feb 27 2025
Joe        18:10 -0500	Thu Feb 27 2025
Sara       17:10 -0600	Thu Feb 27 2025
$ ew rm Joe
$ ew
Addison    15:10 -0800	Thu Feb 27 2025
Sara       17:10 -0600	Thu Feb 27 2025
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

* support different location for toml config file
* make sorting optional to honor insert order
* make `strftime` configurable

