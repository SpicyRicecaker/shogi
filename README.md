## Running

For building the files for web, run
```
wasm-bindgen --out-dir ./server/out/ --target web target/wasm32-unknown-unknown/debug/shogi-rs.wasm
```

To optimize bundles, run
```
wasm-opt -Os ./server/out/shogi-rs_bg.wasm -o ./server/out/shogi-rs_bg.wasm
```

To run the webserver cd into `server` and run
```
bun dev
```