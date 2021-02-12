#/bin/bash

case $1
SHOW_LOCATION=


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

frisk () {
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
        printf "$neg_rows_fmt" "X" "$platform" "$cmd" "---"
    else
        printf "$pos_rows_fmt" "✓" "$platform" "$cmd" "$version"
    fi
}

printf "$title_fmt" "Language" "Command" "Version"
frisk python Python
frisk python3 Python
frisk ruby
frisk perl
frisk awk
frisk julia

frisk npm JavaScript
frisk php

frisk gcc C
frisk gdb C
frisk cpp C++
frisk clang C/C++
frisk cargo Rust
frisk rustc Rust
frisk go
frisk zig

frisk java JVM

frisk ghc haskell
frisk racket # do raco

printf "$title_fmt" "Utility" "Command" "Version"
frisk docker Docker

echo
