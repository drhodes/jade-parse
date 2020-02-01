#!/bin/sh

set -ex
python3 -m http.server&

cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build --target web"
#wasm-pack build --target web
