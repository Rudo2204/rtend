# rtend
> My memory is so bad I need to take notes

CI: [![Build Status](https://github.com/Rudo2204/rtend/workflows/CI/badge.svg)](https://github.com/Rudo2204/rtend/actions)\
License: [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

My rust implementation of the original [isaac's tend](https://github.com/isaacmorneau/tend/).\
It's basically a simple rust wrapper for sqlite, it makes a nice CLI note taking program.

## Usage
```
rtend 0.2.1
rudo2204 <rudo2204@gmail.com>
Simple CLI note taking program

USAGE:
    rtend [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --profile <name>    Temporarily operates on a different database

SUBCOMMANDS:
    add       Adds new things
    delete    Deletes things
    edit      Edits things
    find      Finds thing by its information
    init      Initializes the database
    list      Lists information about things
    skim      Skims over the database
```

## Installation

Head to `releases` tab and download the pre-compiled binary of your machine's architecture or clone the repo then compile it yourself with `cargo`.

## Database schema

Here's an entity-relationship diagram of the schema used for the database:

![](schema.png)

## FAQ

### I don't see the skim feature!

Windows targets lack the `skim` feature due to the crate [skim](https://github.com/lotabout/skim) not supporting Windows.\
[Check issue #3](https://github.com/Rudo2204/rtend/issues/3) for an alternate way to implement this feature.

### Where are my databases located?

| Platform | Value                             | Example                                  |
|----------|-----------------------------------|------------------------------------------|
| Linux    | $XDG_DATA_HOME or $HOME           | /home/alice/.local/share                 |
| OSX      | $HOME/Library/Application Support | /Users/Alice/Library/Application Support |
| Windows  | {FOLDERID_RoamingAppData}         | C:\Users\Alice\AppData\Roaming           |

## Contribute

This is my first Rust project so it probably has some (nasty) bugs in it.\
[Create new issue](https://github.com/Rudo2204/rtend/issues) if you meet any bugs or have any ideas.\
Pull requests are welcomed.
