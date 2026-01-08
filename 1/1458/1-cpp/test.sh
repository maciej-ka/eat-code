#!/bin/bash
g++ -std=c++17 solution.cpp \
    -I/opt/homebrew/include \
    -L/opt/homebrew/lib \
    -lgtest -lgtest_main -pthread \
    -o solution && ./solution
