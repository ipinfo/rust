#!/bin/bash

DIR=`dirname $0`
ROOT=$DIR/..

# Format the project.

rustfmt \
    --config-path $ROOT/.rustfmt.toml \
    $ROOT/src/*
