#!/usr/bin/env bash
#source libziyy.sh
[[ -f ../lib/ziyy/libziyy.sh ]] && source ../lib/ziyy/libziyy.sh
[[ -f libziyy.sh ]] && source libziyy.sh

function usage {
    style "Convenient Terminal Output Styler.

[c:green][b]Usage: [c:cyan]ziyy[/b] [c:cyan][OPTION] [TEXT]

[b][c:green]Options:[/0]
  [c:cyan][b]-V[/0], [c:cyan][b]--version[/0]
          Print version info and exit
  [c:cyan][b]-f[/0], [c:cyan][b]--file[/b] <FILENAME>[/c]
          Read input from file.
  [c:cyan][b]-n[/0], [c:cyan][b]--no-newline[/0]
          Do not print newline after text.
  [c:cyan][b]-h[/0], [c:cyan][b]--help[/0]
          Print help
"
}

function main {
    args=(${@:1})
    if [[ ${#args} -lt 1 ]]; then
        usage
        builtin printf "$return"
        exit 1
    fi
    first=$1
    if [[ $first == "-n" ]] || [[ $first == "--no-newline" ]]; then
        style "$2"
        builtin printf "$return"
    elif [[ $first == "-f" ]] || [[ $first == "--filename" ]]; then
        if [[ ${#args} -eq 1 ]]; then
            exit 1
        fi
        if ! [[ -f $2 ]]; then
            exit 1
        fi
        file=$(<$2)
        style "$file"
        builtin printf "$return"
    elif [[ $first == "-V" ]] || [[ $first == "--version" ]]; then
        builtin printf "ziyy 1.0.6\n"
    elif [[ $first == "-h" ]] || [[ $first == "--help" ]]; then
        usage
        builtin printf "$return"
        exit 0
    fi
}
main $@
