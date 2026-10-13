#!/bin/zsh
export MallocNanoZone=0

g++ -std=c++17 solution.cpp \
    -I/opt/homebrew/include \
    -L/opt/homebrew/lib \
    -g \
    -fsanitize=address,undefined \
    -o solution || exit 1

test() {
    ./solution <<< "$2" | diff -u <(echo "$3") - | tail -n +4
    if (( pipestatus[2] )); then
        echo "test $1 ... FAILED"
        echo
    else
        echo "test $1 ... ok"
    fi
}

test 1 0 "NO"
test 2 8 "YES"
test 3 1 "NO"
test 4 2 "NO"
test 5 4 "YES"
