∏#/bin/bash

normal=$'\e[0m'
red=$'\e[1;31m'
green=$'\e[1;32m'
blue=$'\e[0;34m'
purple=$'\e[0;35m'
cyan=$'\e[0;36m'
bold=$'\e[0m'
dim=$'\e[2m'
fmt_end=$'\033[0m'

title_fmt="\n$bold     %-10s   %-10s   %-15s$fmt_end\n"
pos_rows_fmt="$green %1s   $cyan%-10s   $blue%-10s   $purple%-15s$fmt_end\n"
neg_rows_fmt="$red %1s   $normal$dim%-10s   $dim%-10s   $dim%-15s$fmt_end\n"
TableWidth=50

dih () {
    local cmd=$1
    local platform=$cmd

    if ! [ -z "$2" ]; then
        local platform=$2
    fi

    local version_regex="((?:\d+\.)+(?:\d+))(p\d+)?(?:.*?)"
    local version=$($cmd --version 2>&1 |& grep -oP -m1 $version_regex | sed -n '1p')

    if [ "$version" == "" ]; then
        version=$($cmd -version 2>&1 |& grep -oP -m1 $version_regex | sed -n '1p')
    fi

    if [ "$version" == "" ]; then
        version=$($cmd version 2>&1 |& grep -oP -m1 $version_regex | sed -n '1p')
    fi
    
    if [ "$version" == "" ]; then
        version=$($cmd --help 2>&1 |& grep -oP -m1 $version_regex | sed -n '1p')
    fi
    
    if [ "$version" == "" ]; then
        version=$($cmd -help 2>&1 |& grep -oP -m1 $version_regex | sed -n '1p')
    fi
    
    if [ "$version" == "" ]; then
        version=$($cmd help 2>&1 |& grep -oP -m1 $version_regex | sed -n '1p')
    fi

    if [ "$version" == "" ]; then
        printf "$neg_rows_fmt" "X" "$platform" "$cmd" "---"
    else
        printf "$pos_rows_fmt" "✓" "$platform" "$cmd" "$version"
    fi
}

if ! [ -z "$1" ]; then
    dih $1
else 
    printf "$title_fmt" "Language" "Command" "Version"
    dih python Python
    dih python3 Python
    dih ruby
    dih perl
    dih awk
    dih julia
    dih lua

    dih npm JavaScript
    dih php

    dih gcc C
    dih gdb C
    dih cpp C++
    dih clang C/C++
    dih cargo Rust
    dih rustc Rust
    dih go
    dih zig
    dih mono C#
    dih mono F#

    dih java JVM

    dih ghc haskell
    # dih racket # do raco

    # printf "$title_fmt" "Utility" "Command" "Version"
    # dih docker Docker

    echo
fi
