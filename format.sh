#!/bin/sh

find $(dirname "$0")/CrossChronox -name *pp | xargs clang-format -i
