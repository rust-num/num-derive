# Release 0.1.43

- [The derived code now explicitly allows `trivial_numeric_casts`][7], so users
  that globally deny that lint don't encounter an error.

[7]: https://github.com/rust-num/num-derive/pull/7

# Release 0.1.42

- [num-derive now has its own source repository][num-356] at [rust-num/num-derive][home].
- [The derivation macros have been updated][3] to using `syn` 0.12.  Support for complex
  expressions in enum values can be enabled with the `full-syntax` feature.

Thanks to @cuviper and @hcpl for their contributions!

[home]: https://github.com/rust-num/num-derive
[num-356]: https://github.com/rust-num/num/pull/356
[3]: https://github.com/rust-num/num-derive/pull/3


# Prior releases

No prior release notes were kept.  Thanks all the same to the many
contributors that have made this crate what it is!

