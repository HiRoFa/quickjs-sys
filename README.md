# hirofa-quickjs-sys

Low level rust bindings for quickjs, used by [quickjs_runtime](https://github.com/HiRoFa/quickjs_es_runtime).

uses either
* The original by Fabrice Bellard. see: https://bellard.org/quickjs/
  * works on linux, mac and windows(mingw)
* quickjs-ng, Friendly QuickJS fork focused on reigniting the project. (```default-features=false, features=["quickjs-ng"]```) see: https://github.com/quickjs-ng/quickjs
  * works on linux, mac, windows(mingw) and windows(msvc)

# getting started

Cargo.toml
* bellard version
```toml
libquickjs-sys = {package="hirofa-quickjs-sys", version="0.10.0"}
```
or quickjs-ng
```toml
libquickjs-sys = {package="hirofa-quickjs-sys", version="0.10.0", default-features=false, features=["quickjs-ng"]}
```
