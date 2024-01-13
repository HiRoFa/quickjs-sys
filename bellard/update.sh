#version=2023-12-09
#version=2024-01-13

wget https://bellard.org/quickjs/quickjs-2024-01-13.tar.xz
rm -r -f ./quickjs
tar -xvf quickjs-2024-01-13.tar.xz
rm quickjs-2024-01-13.tar.xz
mv quickjs-2024-01-13 quickjs
bindgen ../wrapper.h -o bindings.rs -- -I ./
echo "2024-01-13" > ./quickjs/VERSION