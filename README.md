# rtend
> My memory is so bad I need to take notes

Unix: [![Build Status](https://travis-ci.com/Rudo2204/rtend.svg?branch=master)](https://travis-ci.com/Rudo2204/rtend)\
Windows: [![Build status](https://ci.appveyor.com/api/projects/status/3ltt06neh2uns9y0?svg=true)](https://ci.appveyor.com/project/Rudo2204/rtend)\
License: [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

My rust implementation of the original [isaac's tend](https://github.com/isaacmorneau/tend/).\
It's basically a simple rust wrapper for sqlite, it makes a nice CLI note taking program.

## Usage
```
rtend 0.2.0
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
# Installation
Head to `releases` tab and download the pre-compiled binary of your machine's architecture or clone the repo then compile it yourself with `cargo`.

# FAQ
### I don't see the skim feature!
Windows targets lack the `skim` feature due to the crate [skim](https://github.com/lotabout/skim) not supporting Windows.\
[Check issue #3](https://github.com/Rudo2204/rtend/issues/3) for an alternate way to implement this feature.

### Where are my databases located?
| Platform | Value                             | Example                                  |
|----------|-----------------------------------|------------------------------------------|
| Linux    | $XDG_DATA_HOME or $HOME           | /home/alice/.local/share                 |
| OSX      | $HOME/Library/Application Support | /Users/Alice/Library/Application Support |
| Windows  | {FOLDERID_RoamingAppData}         | C:\Users\Alice\AppData\Roaming           |

### FreeBSD is not supported?
I ran a build in CI (See build [#1.18](https://travis-ci.com/Rudo2204/rtend/jobs/277481017) and [#1.19](https://travis-ci.com/Rudo2204/rtend/jobs/277481018)) and it looks like it fails to compile on FreeBSD but it works fine on NetBSD (See build [#1.20](https://travis-ci.com/Rudo2204/rtend/jobs/277481019)).\
I don't know how to fix this, looking for someone to point me to the right direction.

# Contribute
This is my first Rust project so it probably has some (nasty) bugs in it.\
[Create new issue](https://github.com/Rudo2204/rtend/issues) if you meet any bugs or have any ideas.\
Pull requests are welcomed.
