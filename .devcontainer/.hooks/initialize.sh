#!/bin/bash

set -ex

BASH_HISTORY_PATH=".devcontainer/workspace/root/.bash_history"
if [ ! -f $BASH_HISTORY_PATH ]; then
  touch $BASH_HISTORY_PATH
fi

BASHRC_EXAMPLE_PATH=".devcontainer/workspace/root/.bashrc.example"
BASHRC_PATH=".devcontainer/workspace/root/.bashrc"
if [ ! -f $BASHRC_PATH ]; then
  cp $BASHRC_EXAMPLE_PATH $BASHRC_PATH
fi
