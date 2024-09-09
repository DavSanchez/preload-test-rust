## Reproduce quickly with `docker` (`aarch64`)

```console
$ rm -rf target ; docker run --rm --volume $(pwd):/app rust:1.80.1 bash -c "cd app ; cargo test"
Should succeed
```

```console
$ rm -rf target ; docker run --rm --volume $(pwd):/app rust:1.81.0 bash -c "cd app ; cargo test"
Should fail
```