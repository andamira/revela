## Features

Features are grouped in the following categories:
- *Miscellaneous*
- *Environment*
- *Modules*
- *OS*
- *Safety*
- *Nightly*
- *Capability*
- *Dependencies*

Features from different categories are designed to be mostly independent from
each other, and composable, except from the miscellaneous features.

### Miscellaneous features

In this category there are features with varied purposes mostly for internal use.

<!-- - `__dbg`: for debugging purposes, shows enabled features and reflection flags. -->
- `__notest`: allows excluding examples from being tested.


### Environment features

By default the crate is `no_std` compatible without allocation.

- `std`: disables `no_std` compatibility and enables `std` functionality.
- `alloc`: enables `alloc` functionality.
- `no_std`: enables functionality incompatible with or substitute of `std`:
  - enables the `libm` optional dependency.


### Modules features

Modules can be enabled independently of *environment*, *dependencies* or *safety*.

- `all`: enables all the root modules and extra submodules:

Single modules:
- [`audio`]
- [`color`]
- [`font`]
- [`image`]
- [`ui`]
- [`video`]

[`audio`]: crate::audio
[`color`]: crate::color
[`font`]: crate::font
[`image`]: crate::image
[`ui`]: crate::ui
[`video`]: crate::video


### OS features

- [`linux`]
- [`web`]
- [`windows`]
- [`term`]


### Safety features

In order to use any unsafe functionality:
1. enable the corresponding `unsafe` feature.
2. don't enable the `safe` feature for that module.

- `safe`: forbids `unsafe` (and overrides unsafe features), includes:
  - `safe_audio`
  - `safe_color`
  - `safe_font`
  - `safe_image`
  - `safe_ui`
  - `safe_video`

- `unsafe`: enables `unsafe` (as long as it isn't forbidden in the module).

  It also enables the corresponding features from `devela` with the same name:
	- `unsafe_array`: faster array initialization.
	- `unsafe_async`: custom task waker, coroutine impls.
	- `unsafe_const`: extra const methods.
	- `unsafe_dyn` DSTs in the stack, `no_std` Error dyn impls, `ExtAny::downcast*`.
	- `unsafe_niche`: unchecked niche constructors.
	- `unsafe_ptr`: `Pinned`, pop methods without `Clone`.
	- `unsafe_slice`: extra slice methods, avoid bound checks.
	- `unsafe_str`: unchecked utf-8 char and &str conversions.
	- `unsafe_thread`: `Logging::set_logger_racy`.

- `safest`: forbids unsafe recursively.
- `unsafest`: enables unsafe recursively.


### Nightly features

<!-- Enabling any of them sets the `_some_nightly` flag. -->

- `nightly`: enables the nightly features.

  It also enables the corresponding features from `devela` with the same name:
  - `nightly_coro`: enables `coroutines`, `coroutine_trait`, `iter_from_coroutine`.
  - `nightly_doc`: enables `doc_cfg`.
  - `nightly_simd`: enables `portable_simd`.
  - `nightly_stabilized`: enables stabilized features to be released soon.


### Capability features

Documentation capabilities:
- `_docsrs`: enables the most complete version of the documentation for [docs.rs](https://docs.rs).
- `_docsrs_stable`: like `_docsrs` but without enabling `nightly`.


### Dependencies features

<!-- Enabling any of them sets the `_some_dep` flag. -->

- `dep_std`: enables all backends compatible with `std`.
- `dep_alloc`: enables all backends compatible with `alloc`.
- `dep_no_std`: enables all backends compatible with `no_std`.
