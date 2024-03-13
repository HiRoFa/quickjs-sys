# hirofa-quickjs-sys

Low level rust bindings for quickjs, used by [quickjs_runtime](https://github.com/HiRoFa/quickjs_es_runtime).

uses either
* The original by Fabrice Bellard (```features=['bellard']```) see: https://bellard.org/quickjs/
* quickjs-ng, Friendly QuickJS fork focused on reigniting the project. (```features=['quickjs-ng']```) see: https://github.com/quickjs-ng/quickjs
* later
  * ~~openwebf, Optimized quickjs mantained by OpenWebF team. (```features=['openwebf']```) see: https://github.com/openwebf/quickjs~~

status: 
* bellard version is working, updated to 2024-01-13
* quickjs-ng compiles, have not tested yet
* openwebf, future

# getting started

Cargo.toml
```toml
libquickjs-sys = {package="hirofa-quickjs-sys", version="0.4.0", features=["bellard"]}
```
or
```toml
libquickjs-sys = {package="hirofa-quickjs-sys", version="0.4.0", features=["quickjs-ng"]}
```