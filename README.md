# rtend
> My memory is so bad I need to take notes

Unix: [![Build Status](https://travis-ci.com/Rudo2204/rtend.svg?branch=master)](https://travis-ci.com/Rudo2204/rtend)\
Windows: [![Build status](https://ci.appveyor.com/api/projects/status/3ltt06neh2uns9y0?svg=true)](https://ci.appveyor.com/project/Rudo2204/rtend)\
License: [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

My rust implementation of the original [isaac's tend](https://github.com/isaacmorneau/tend/).\
It's basically a simple rust wrapper for sqlite, it makes a nice CLI note taking program.

## Usage
```
rtend v0.1.4
Simple CLI note taking program

USAGE:
    rtend [OPTIONS] <SUBCOMMAND>

GLOBAL OPTIONS:
        -p, --profile <name>    Temporarily operates on a different database

SUBCOMMANDS:
    add       Add new things
    delete    Delete things
    edit      Edit things
    find      Find thing by its information
    init      Initialize the database
    list      List information about things
    skim      Skim over the database

------------------------------------------------------------------
rtend add [OPTIONS] -- Add new things

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -a, --alias <entity_id> <name>                Add an alias to an entity
    -e, --entity <name>                           Add a new entity
    -r, --relation <entity_id_a> <entity_id_b>    Add a relation between two entities
    -d, --relation-snippet <relation_id>          Add a snippet to a relation
    -s, --snippet <entity_id>                     Add a snippet to an entity

------------------------------------------------------------------
rtend delete [FLAGS] [OPTIONS] -- Delete things

FLAGS:
    -f, --force    Force delete everything related to the command
                   Use with --entity or --relation

OPTIONS:
    -a, --alias <alias_id>                          Delete an alias
    -e, --entity <entity_id>                        Delete an entity
    -r, --relation <relation_id>                    Delete a relation
    -d, --relation-snippet <relation_snippet_id>    Delete a relation snippet
    -s, --snippet <snippet_id>                      Delete a snippet

------------------------------------------------------------------
rtend edit [OPTIONS] -- Edit things

OPTIONS:
    -a, --alias <alias_id>                          Edit an alias
    -d, --relation-snippet <relation_snippet_id>    Edit a relation snippet
    -s, --snippet <snippet_id>                      Edit a snippet

------------------------------------------------------------------
rtend find [FLAGS] [OPTIONS] -- Find thing by its information

FLAGS:
    -v, --verbose    Increase verbosity level
                     Use with --alias or --relation

OPTIONS:
    -a, --alias <name>                 Find an entity by alias
    -r, --relation <entity_id>         Find an relation by entity id
    -d, --relation-snippet <string>    Find an entity by its relation snippet
    -s, --snippet <string>             Find an entity by its snippet

------------------------------------------------------------------
rtend list [FLAGS] [OPTIONS] -- List information about things

FLAGS:
        --stats      List stats about the database
    -v, --verbose    Increase verbosity level
                     Use with --entity or --relation
                     Or as a standalone flag

OPTIONS:
    -a, --alias <entity_id>                 Lists aliases of an entity
    -e, --entity <entity_id>                Lists information about an entity
    -r, --relation <relation_id>            Lists relations of an entity
    -d, --relation-snippet <relation_id>    Lists relation snippets of an entity
    -s, --snippet <entity_id>               Lists snippets of an entity
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
