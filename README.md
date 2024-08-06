# `unwind-unsafe`

This Rust crate provides zero-sized marker types which do not implement
`UnwindSafe` or `RefUnwindSafe`. They can be added to types to prevent the
respective auto traits from being derived.

It has no dependencies, no unsafe code, and is `no_std` by default.

## Documentation

See [API docs on docs.rs](https://docs.rs/unwind-unsafe).

## License

This project is licensed under the [MIT No Attribution
License](https://spdx.org/licenses/MIT-0.html).  Unless you explicitly state
otherwise, any contribution intentionally submitted for inclusion in
`unwind-unsafe` by you, shall be licensed as MIT No Attribution, without any
additional terms or conditions.