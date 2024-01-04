rm -r -f ./quickjs
git clone https://github.com/openwebf/quickjs.git --single-branch
rm -r -f quickjs/.git
echo "openwebf-2024-01-04" > ./quickjs/VERSION