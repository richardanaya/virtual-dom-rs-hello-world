# Hello World with virtual-dom-rs

This was one of the coolest libraries i've seen doing virtual DOM in Rust. I was disappointed though it didn't have any simple examples nor is the crate on cargo up to date. So I made this project to show the most simplest utilization of a client side app using entirely wasm-bindgen, web-sys, and virtual-dom-rs.

No javascript required (asside what wasm-bindgen auto creates ;) )!

You are going to need some things unfortunately:
* cargo with nightly and wasm target
* nodejs + npm ( this is a dependency of wasm-bindgen)
* make

How to run:

```console
make setup
make
make serve
http://localhost:8080
```

if you want to edit it after its running, just open up a new tab and run `make` to generate new wasm binary, you shouldn't have to restart the server.
