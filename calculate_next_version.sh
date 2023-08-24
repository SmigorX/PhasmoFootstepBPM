#!/bin/bash

# Get the latest tag
latest_tag=$(git describe --tags --abbrev=0)

# Use semver to increment the patch version
next_version=$(semver bump patch $latest_tag)

echo $next_version
