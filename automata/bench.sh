#!/usr/bin/env bash

valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes --simulate-cache=yes "$@"
