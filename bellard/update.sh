
wget https://bellard.org/quickjs/quickjs-2025-04-26.tar.xz
rm -r -f ./quickjs
tar -xvf quickjs-2025-04-26.tar.xz
rm quickjs-2025-04-26.tar.xz
mv quickjs-2025-04-26 quickjs
bindgen ../wrapper.h -o bindings.rs -- -I ./
echo "2025-04-26" > ./quickjs/VERSION
