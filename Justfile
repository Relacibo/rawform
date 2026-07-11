set shell := ["bash", "-uc"]

default:
    @just --list

# Build & run locally
run:
    cargo run

# Build the Docker image locally
docker-build:
    docker build -t rawform:local .

# Release: `just release` bumps patch; `just release 0.2.0` sets that version.
# Updates Cargo.toml, commits, tags v<version> and pushes (triggers CI image build).
release version="":
    #!/usr/bin/env bash
    set -euo pipefail

    if [[ -n "$(git status --porcelain)" ]]; then
        echo "error: working tree is dirty; commit or stash first" >&2
        exit 1
    fi

    current=$(cargo metadata --no-deps --format-version 1 \
        | grep -o '"version":"[^"]*"' | head -n1 | cut -d'"' -f4)

    input="{{ version }}"
    input="${input#v}"

    if [[ -z "$input" ]]; then
        IFS='.' read -r major minor patch <<< "$current"
        new="${major}.${minor}.$((patch + 1))"
    else
        if [[ ! "$input" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
            echo "error: version must be MAJOR.MINOR.PATCH (got '$input')" >&2
            exit 1
        fi
        new="$input"
    fi

    tag="v${new}"
    if git rev-parse "$tag" >/dev/null 2>&1; then
        echo "error: tag $tag already exists" >&2
        exit 1
    fi

    echo "Releasing ${current} -> ${new}"

    # Update version in Cargo.toml [package] section, then sync Cargo.lock
    sed -i -E "0,/^version = \"[^\"]*\"/s//version = \"${new}\"/" Cargo.toml
    cargo update -p rawform

    git add Cargo.toml Cargo.lock
    git commit -m "Release ${tag}"
    git tag -a "$tag" -m "Release ${tag}"
    git push origin HEAD
    git push origin "$tag"

    echo "Pushed ${tag} -> CI will build & push the Docker image."
