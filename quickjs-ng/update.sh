rm -r -f ./quickjs
git clone https://github.com/quickjs-ng/quickjs.git --branch v0.8.0 --single-branch
rm -r -f quickjs/.git
bindgen ../wrapper.h -o bindings.rs -- -I ./
echo "quickjs-ng-0.8.0" > ./quickjs/VERSION