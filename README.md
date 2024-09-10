## Reproduce quickly with `docker` (`aarch64`)

### `cargo test`

```console
$ rm -rf target ; docker run --rm --volume $(pwd):/app rust:1.80.1 bash -c "cd app ; cargo test"
   Compiling preload-test v0.1.0 (/app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.46s
     Running unittests src/lib.rs (target/debug/deps/preload_test-ff929676b3aef50e)
HOLA!
{"key3": "value3", "key1": "value1", "key2": "value2"}
Bye!

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```console
$ rm -rf target ; docker run --rm --volume $(pwd):/app rust:1.81.0 bash -c "cd app ; cargo test"
   Compiling preload-test v0.1.0 (/app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.49s
     Running unittests src/lib.rs (target/debug/deps/preload_test-0a654d8cd2033440)
HOLA!
{"key2": "value2", "key3": "value3", "key1": "value1"}
Bye!
fatal runtime error: thread::set_current should only be called once per thread
error: test failed, to rerun pass `--lib`

Caused by:
  process didn't exit successfully: `/app/target/debug/deps/preload_test-0a654d8cd2033440` (signal: 6, SIGABRT: process abort signal)
```

### `cargo build` and run the result directly

```console
$ rm -rf target ; docker run --rm --volume $(pwd):/app rust:1.80.1 bash -c "cd app ; cargo build ; LD_PRELOAD=target/debug/libpreload_test.so ls -lah"
   Compiling preload-test v0.1.0 (/app)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
HOLA FROM LD_PRELOAD!
total 492K
drwxr-xr-x 11 root root  352 Sep 10 21:50 .
drwxr-xr-x  1 root root 4.0K Sep 10 21:50 ..
drwxr-xr-x 16 root root  512 Sep  9 22:18 .git
-rw-r--r--  1 root root    8 Sep  9 21:05 .gitignore
-rw-r--r--  1 root root  156 Sep  9 22:17 Cargo.lock
-rw-r--r--  1 root root  113 Sep  9 22:18 Cargo.toml
-rw-r--r--  1 root root 1.2K Sep  9 21:05 README.md
-rw-------  1 root root 472K Sep 10 21:48 core
drwxr-xr-x  3 root root   96 Sep 10 21:50 src
drwxr-xr-x  5 root root  160 Sep 10 21:50 target
drwxr-xr-x  3 root root   96 Sep 10 21:48 tests
```

```console
$ rm -rf target ; docker run --rm --volume $(pwd):/app rust:1.81.0 bash -c "cd app ; cargo build ; LD_PRELOAD=target/debug/libpreload_test.so ls -lah"
   Compiling preload-test v0.1.0 (/app)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
HOLA FROM LD_PRELOAD!
total 492K
drwxr-xr-x 11 root root  352 Sep 10 21:50 .
drwxr-xr-x  1 root root 4.0K Sep 10 21:50 ..
drwxr-xr-x 16 root root  512 Sep  9 22:18 .git
-rw-r--r--  1 root root    8 Sep  9 21:05 .gitignore
-rw-r--r--  1 root root  156 Sep  9 22:17 Cargo.lock
-rw-r--r--  1 root root  113 Sep  9 22:18 Cargo.toml
-rw-r--r--  1 root root 1.2K Sep  9 21:05 README.md
-rw-------  1 root root 472K Sep 10 21:48 core
drwxr-xr-x  3 root root   96 Sep 10 21:50 src
drwxr-xr-x  5 root root  160 Sep 10 21:50 target
drwxr-xr-x  3 root root   96 Sep 10 21:48 tests
```
