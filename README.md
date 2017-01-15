# `rocket-test`

A static file server written by following [the Rocket getting started guide](https://rocket.rs/guide/requests/).

### License

MIT

### Usage:

```
$ cargo run
```

Then in your browser navigate to `localhost:8000/files/foo.txt` -- notice that you can read the text.

Now navigate to `localhost:8000/files/bar.json`.

This file _does_ exist but it is a secret!

This is achieved by explicitly returning `None` in the `files` function.

### Other information

Built with:
  * Rust 1.16.0-nightly (1a2ed98d3 2017-01-13)
  * Rocket 0.1.5
