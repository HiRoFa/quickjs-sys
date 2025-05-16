rm -r -f ./quickjs
git clone https://github.com/quickjs-ng/quickjs.git --branch v0.10.1 --single-branch
rm -r -f quickjs/.git
bindgen ./wrapper.h --blocklist-item FP_NORMAL --blocklist-item FP_NAN --blocklist-item FP_INFINITE --blocklist-item FP_ZERO --blocklist-item FP_SUBNORMAL -o bindings.rs -- -I ./
echo "quickjs-ng-0.10.1" > ./quickjs/VERSION