#!/usr/bin/env bash

set -euo pipefail

version=$1

echo "update version field"
sed -i -E 's/"version": ".+minswap.+"/"version": "'"$version"'"/' package.json
sed -i -E 's/"version": ".+minswap.+"/"version": "'"$version"'"/' package-lock.json

sed -i -E 's/version = ".+minswap.+"/version = "'"$version"'"/' rust/Cargo.toml
sed -i -E 's/version = ".+minswap.+"/version = "'"$version"'"/' rust/Cargo.lock

echo "publish"
npm run js:publish-nodejs:prod
npm run js:publish-browser:prod

echo "commit new version"
git add package.json package-lock.json rust/Cargo.toml rust/Cargo.lock
git commit -m "publish version $version"

echo "tag version"
git tag "$version"

echo "push to remote"
git push origin minswap-11
git push origin "$version"
