# hirofa-quickjs-sys

Low level rust bindings for quickjs, used by [quickjs_runtime](https://github.com/HiRoFa/quickjs_es_runtime).

uses either
* The original by Fabrice Bellard (```features=['bellard']```) see: https://bellard.org/quickjs/
* quickjs-ng, Friendly QuickJS fork focused on reigniting the project. (```features=['quickjs-ng']```) see: https://github.com/quickjs-ng/quickjs
* openwebf, Optimized quickjs mantained by OpenWebF team. (```features=['openwebf']```) see: https://github.com/openwebf/quickjs

status: 
* bellard version is working, updated to 2024-01-13
* quickjs-ng compiles, have not tested yet
* openwebf does not compile as build scripts needs some mods, will look into that later

