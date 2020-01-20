# rtend
Unix: [![Build Status](https://travis-ci.com/Rudo2204/rtend.svg?branch=master)](https://travis-ci.org/Rudo2204/rtend)<br/>
Windows: [![Build status](https://ci.appveyor.com/api/projects/status/3ltt06neh2uns9y0?svg=true)](https://ci.appveyor.com/project/Rudo2204/rtend)

My rust implementation of the isaac's tend: https://github.com/isaacmorneau/tend/<br/>
Basically a simple Rust wrapper for sqlite, it makes a nice CLI note taking program.

## Overview
```
rtend 0.1.0
Simple CLI note taking program

USAGE:
    rtend [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --profile <name>    Temporary operates on a different database

SUBCOMMANDS:
    add       Adds new things
    delete    Deletes things in the database
    edit      Edit things in the database
    find      Finds a thing given its information
    init      Initializes the database
    list      Lists information about things
    skim      Skims over the database
```
# Installation
Head to `releases` tab and download the pre-compiled binary or clone the repo and compile it yourself.

# Windows
Windows targets lack the `skim` feature due to the crate [skim](https://github.com/lotabout/skim) does not support Windows.
