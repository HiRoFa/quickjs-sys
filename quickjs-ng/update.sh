rm -r -f ./quickjs
#git clone https://github.com/quickjs-ng/quickjs.git --branch v0.4.1 --single-branch
git clone https://github.com/quickjs-ng/quickjs.git --single-branch
rm -r -f quickjs/.git
bindgen ../wrapper.h -o bindings.rs -- -I ./
echo "quickjs-ng-0.4.1" > ./quickjs/VERSION