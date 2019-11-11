This use `nightly` feature of `rust`

We need to install it using `rustup`

```
$ rustup toolchain install nightly
```

Build:

```
$ cargo build --bin req --release
```

Run test:

```
$cargo run --bin req -- -e <email> -p <password>
```
