

cd www
npm run build
cd ../
cp www/dist/* .
cp pkg/wasmlife_bg.js pkg/wasmlife_bg.wasm .
vim -c ":%s/.\/pkg//g | :wq" bootstrap.js
vim -c ":%s/.\/pkg//g | :wq" "0.bootstrap.js"
