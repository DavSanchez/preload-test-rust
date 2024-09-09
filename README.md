## Reproduce quickly with `docker` (`aarch64`)

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