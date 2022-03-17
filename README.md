# Volu: directories usages and file size
Prints the size of the given directory or file, featuring the largest dirs
and a full map of each dir's sub-directories

## Features:
- Sorts and prints the largest dirs
- Prints a sized-map of sub-dirs
- Pretty Light-weight
- Fast as default

## Usage
> Some platform's are released on github
> An option is to clone this repo and run `cargo run` instead of `volu`

Current dir's usage: `volu`  
Usage of a specific dir: `volu path/to/dir/`
Print a list of all *parent dirs*: `volu -p`
Print a sorted parent dirs list: `volu -s`
Limit the number of printed parent dirs list: `volu -l 10`
Print a sized map of all dirs and their children: `volu -m`

Best use: `volu --map path/to/dir/`
Minimal use: `volu path/to/dir/`

```command
volu 0.1.0
Arian Mirahmadi (thearian@github) (mirarianmir@gmail.com)
Prints the size of the given directory, featuring the largest dirs

USAGE:
    volu.exe [OPTIONS] [DIR]

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
