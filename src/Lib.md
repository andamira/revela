
# Features

- `std` (default): enables functionality that depends on the standard library.
  Disabling it makes the crate `no_std` compatible.
- `alloc`: enables functionality that depends on allocation. Included in `std`.
- `no_std`: enables functionality incompatible with `std`.
---
- `safe`: forbids unsafe at the crate level.
- `safest`: forbids unsafe recursively.
- `unsafe`: enables all the unsafe features:
- `unsafest`: enables unsafe recursively.
---
- `all_std`: enables all backends compatible with `std`.
- `all_alloc`: enables all backends compatible with `alloc`.
- `all_no_std`: enables all backends compatible with `no_std`.
