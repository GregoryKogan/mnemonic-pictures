#!/usr/bin/env sh

# abort on errors
set -e

yarn build

cd dist

cp index.html 404.html

echo > .nojekyll

# if you are deploying to a custom domain
# echo 'www.example.com' > CNAME

git init
git checkout -B main
git add -A
git commit -m 'deploy'

# if you are deploying to https://<USERNAME>.github.io
# git push -f git@github.com:<USERNAME>/<USERNAME>.github.io.git master

# if you are deploying to https://<USERNAME>.github.io/<REPO>
git push -f git@github.com:GregoryKogan/mnemonic-pictures.git main:gh-pages

cd -