#!/bin/bash

# set -e

if [ $# -ne 1 ]; then
    echo "Usage: ./setup.sh 1"
    exit 1
fi

PROBLEM_NUMBER=$1
DIRECTORY_SUFFIX="problem-"
DIRECTORY_NAME="${DIRECTORY_SUFFIX}${PROBLEM_NUMBER}"

if [ -d "$DIRECTORY_NAME" ]; then
    echo "ディレクトリが既に存在しています。"
    exit 1
fi

rlr d "https://recursionist.io/dashboard/problems/${PROBLEM_NUMBER}"


# copy template file
cp ./template/main.py "$DIRECTORY_NAME/main.py"

# copy shell script to execute judge
cp ./template/run.sh "$DIRECTORY_NAME/"
