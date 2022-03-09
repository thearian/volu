# Volu: directories usages
Prints the size of the given directory, featuring the hea

## Features:
- Light-weight
- Fast
- Blame: prints the heaviest dirs

## Usage
> Work is in progress, no binaries out there yet  

Dirs usage of the current dir: `volu`  
Dirs usage of a specific dir: `volu path/to/dir/`
- Print a list of all *parent dirs*: `volu -p`
- Print a sorted parent dirs list: `volu -s`
- Limit the number of printed parent dirs list: `volu -l 10`

Best use: `volu --sort path/to/dir/`

```command
volu 0.0.1
Arian Mirahmadi (thearian@github) (mirarianmir@gmail.com)
Maps the size of all the child directories

USAGE:
    volu[.EXE] [OPTIONS] [DIR]

ARGS:
    <DIR>    The parent directory [default: .]

OPTIONS:
    -a, --all              Print all the parent directories, no limit
    -h, --help             Print help information
    -l, --limit <LIMIT>    Sort and limit the parent directories [default: 25]
    -p, --print            Print the parent directories
    -s, --sort             Sort the parent directories
    -V, --version          Print version information
```
