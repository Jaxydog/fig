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

Fig is free software:
you can redistribute it and/or modify it under the terms of
the GNU Lesser General Public License as published by the Free Software Foundation,
either version 3 of the License, or (at your option) any later version.

Fig is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
See the GNU Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public License along with Fig
(located within [LICENSE](./LICENSE)).
If not,
see <[https://www.gnu.org/licenses/](https://www.gnu.org/licenses/)>.
