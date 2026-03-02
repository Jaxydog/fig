# Fig

Provides a simple API for declaring custom `cfg` predicates at compile-time.

### Usage

```rs
use fig::{Cfg, CheckedCfg};

fn main() {
    // Will create a new `cfg` predicate that can be set to either `"foo"` or `"bar"`.
    // Usable like `#[cfg(custom_cfg = "foo")]`.
    Cfg::new("custom_cfg").assigned_one_of(&["foo", "bar"]).set("foo");
}
```

### License

Fig is subject to the terms of the Mozilla Public License, v. 2.0.
If a copy of the MPL was not distributed with this project,
you can obtain one at <http://mozilla.org/MPL/2.0/>.
