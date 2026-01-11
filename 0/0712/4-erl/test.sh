#!/bin/bash
erlc solution.erl
erl -noshell -eval "eunit:test(solution, [verbose])" -s init stop
