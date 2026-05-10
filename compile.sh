#!/usr/bin/env bash
set -euo pipefail

BUILD=false

for arg in "$@"; do
  case "$arg" in
    --build)
      BUILD=true
      shift
      ;;
    *)
      ;;
  esac
done

if [ "$BUILD" = true ]; then
  docker build -t penumbra .
fi

docker run --rm \
  -v "$PWD":/app \
  -w /app \
  penumbra \
  cargo build --release --target x86_64-pc-windows-msvc
