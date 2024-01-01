#!/bin/sh

  git checkout -b ${1}

  date > ./README.md

  git add ./README.md

  git commit -m ${1}

