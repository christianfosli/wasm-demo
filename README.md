# wasm-demo

Super simple demo of using Rust and web assembly in the browser

### Prereq

* Install wasm-pack
  (See https://rustwasm.github.io/wasm-pack/installer/)

* Install simple-http-server (`cargo install simple-http-server`)

### Build

```bash
wasm-pack build --target web
```

### Run

simple-http-server .