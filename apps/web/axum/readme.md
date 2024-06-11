```shell
# Terminal 1 - To run the server
RUST_LOG=debug cargo watch -q -c -w src/ -x  run
# Terminal 2 - To run the quick_dev
RUST_LOG=debug cargo watch -q -c -w examples/ -x  "run --example quick_dev"
```
