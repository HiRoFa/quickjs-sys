rm -r -f ./quickjs
git clone https://github.com/quickjs-ng/quickjs.git --branch v0.3.0 --single-branch
rm -r -f quickjs/.git
echo "quickjs-ng-0.3.0" > ./quickjs/VERSION