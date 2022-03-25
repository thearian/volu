# Volu: directories usages and file size
[![crates.io](https://img.shields.io/crates/v/volu.svg)](https://crates.io/crates/volu)
[![crates.io](https://img.shields.io/crates/d/volu.svg)](https://crates.io/crates/volu)

Prints the size of the given directory or file, featuring the largest dirs
and a full map of each dir's child-directories

<img src="https://github.com/thearian/volu/blob/master/screenshots/volu-sm.png" width="598" height="543">

## Features:
- Pretty Light-weight (8Kb of source code and 2.2Mb of binaries)
- Usage of the directory
- Print the usage of the child-directories
- Map of the usage of all directories and their children
- Sort the child-directories by usage
- Highlight the largest directory in each directory

## Install
Using Cargo: `cargo install volu`

Using Github: download the latest binaries in the github release

## Usage
Current dir's usage: `volu`  
Usage of a specific dir: `volu path/to/dir/`  
Print a list of all *parent dirs*: `volu -p`  
Print a sorted parent dirs list: `volu -s`  
Limit the number of printed parent dirs list: `volu -l 11`  
Print a sized map of all dirs and their children: `volu -m`  

Best use: `volu -sm path/to/dir/`
Minimal use: `volu path/to/dir/`

```command
volu 0.1.1
Arian Mirahmadi (thearian@github) (mirarianmir@gmail.com)
Prints the size of the given directory, featuring the largest dirs

USAGE:
    volu[.exe] [OPTIONS] [DIR]

ARGS:
    <DIR>    The parent directory [default: .]

OPTIONS:
    -a, --all              Print all the parent directories, no limit
    -h, --help             Print help information
    -l, --limit <LIMIT>    Sort and limit the parent directories [default: 25]
    -m, --map              Print child of parent directories
    -p, --print            Print the parent directories
    -s, --sort             Sort the parent directories
    -V, --version          Print version information
```
