
wget https://bellard.org/quickjs/quickjs-2026-06-04.tar.xz
rm -r -f ./quickjs
tar -xvf quickjs-2026-06-04.tar.xz
rm quickjs-2026-06-04.tar.xz
mv quickjs-2026-06-04 quickjs
bindgen ./wrapper.h -o bindings.rs -- -I ./
echo "2026-06-04" > ./quickjs/VERSION
