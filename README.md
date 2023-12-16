[![dependency status](https://deps.rs/repo/github/baretuna/visheetsql/status.svg)](https://deps.rs/repo/github/baretuna/visheetsql)
[![Build Status](https://github.com/baretuna/visheetsql/workflows/CI/badge.svg)](https://github.com/baretuna/visheetsql/actions?workflow=CI)

pretty unsure of the direction and design but i wanted to make a prototype. i
think a retained gui (i.e. iced) makes more sense for this than egui, but the
web export is not mature enough for iced, plus egui has a built-in table widget
which will work good enough for a prototype.

---

### Testing locally

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

### Web Locally

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page.

[Trunk](https://trunkrs.dev/) is used to build for web target.
1. Install the required target with `rustup target add wasm32-unknown-unknown`.
2. Install Trunk with `cargo install --locked trunk`.
3. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
4. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.

> `assets/sw.js` script will try to cache our app, and loads the cached version when it cannot connect to server allowing your app to work offline (like PWA).
> appending `#dev` to `index.html` will skip this caching, allowing us to load the latest builds during development.