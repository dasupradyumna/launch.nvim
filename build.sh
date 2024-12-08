#!/usr/bin/env bash

set -e

cargo build
mkdir -p lua
mv target/debug/liblaunch.so lua/launch.so
