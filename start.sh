#curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

git reset --hard
git pull origin master
wasm-pack build --target web
