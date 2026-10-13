#!/bin/bash
g++ -std=c++17 solution.cpp \
    -I/opt/homebrew/include \
    -L/opt/homebrew/lib \
    -g \
    -fsanitize=address,undefined \
    -lgtest -lgtest_main -pthread \
    -o solution && ./solution

