
wget https://bellard.org/quickjs/quickjs-2023-12-09.tar.xz
r, -r -f ./quickjs
tar xJv -C ./quickjs --strip-components 1 -f quickjs-2023-12-09.tar.xz
rm quickjs-2023-12-09.tar.xz
bindgen ../wrapper.h -o bindings.rs -- -I ./
echo "2023-12-09" > ./quickjs/VERSION