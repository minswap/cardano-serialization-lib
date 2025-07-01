#!/usr/bin/env bash

set -euo pipefail

version=$1

echo "update version field"
sed -i -E 's/"version": ".+minswap.+"/"version": "'"$version"'"/' package.json
sed -i -E 's/"version": ".+minswap.+"/"version": "'"$version"'"/' package-lock.json

sed -i -E 's/version = ".+minswap.+"/version = "'"$version"'"/' rust/Cargo.toml
sed -i -E 's/version = ".+minswap.+"/version = "'"$version"'"/' rust/Cargo.lock

echo "Authenticate with NPM"
echo "//registry.npmjs.org/:_authToken=$NPM_TOKEN" > ~/.npmrc

echo "publish"

npm run js:publish-nodejs:prod
npm run js:publish-browser:prod
