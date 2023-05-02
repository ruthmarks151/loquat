#!/usr/bin/env bash

find . \( -name target -o -name server/static -o -name dist \) -type d | xargs rm -rf
