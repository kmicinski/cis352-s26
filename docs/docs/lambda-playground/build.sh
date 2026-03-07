#!/bin/bash
set -e

cd "$(dirname "$0")"

echo "Building lambda-viz WASM..."
wasm-pack build --target web --out-dir www/pkg

echo ""
echo "Build complete!"
echo "To serve: cd www && python3 -m http.server 8080"
echo "Then open: http://localhost:8080"
