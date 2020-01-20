# rtend
Rust implementation of the isaac's tend: https://github.com/isaacmorneau/tend/<br/>
Basically a simple Rust wrapper for sqlite, it makes a nice CLI note taking program.

## Usage
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
