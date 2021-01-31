#/bin/bash

dig () {
    local cmd=$1
    local version_regex="((?:\d+\.)+(?:\d+))(p\d+)?(?:.*?)"
    local version=$($cmd --version 2>&1 |& grep -oP -m1 $version_regex |  sed -n '1p')

    if [ "$version" == "" ]; then
        version=$($cmd version 2>&1 |& grep -oP -m1 $version_regex |  sed -n '1p')
    fi

    if [ "$version" == "" ]; then
        echo $cmd "Not installed"
    else
        echo $cmd $version
    fi
}

echo "=== Scripting ==="
dig npm
dig python
dig python3
dig ruby
dig perl
dig awk
echo

echo "=== Systems ==="
dig rustc
dig cargo
dig gcc
dig cpp
dig go
echo

echo "=== JVM ==="
dig java
echo

echo "=== Fucntional ==="
dig haskell
dig racket # do raco
echo
