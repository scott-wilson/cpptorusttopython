This is a test of going from C++ to Rust to Python

To run
======

Dependencies
------------

- Rust
- Python 2.7 `sudo apt install python2-dev`

Build/Run
---------

```bash
clear && cargo build --release && PYTHONPATH=$PYTHONPATH:$(pwd)/target/release python -c 'import cpptorusttopython; a = cpptorusttopython.Something(); print(a.get_value()); a.set_value(10); print(a.get_value())'
```
