# CommonLibSSE-NG rust bindings

It probably will crash if used it does not pass the layout test.

See [test results](./test_results.txt)

```toml
commonlibsse_ng_sys = { git = "https://github.com/SARDONYX-sard/commonlibsse_ng", rev = "6891f79" } # Fixed by rev(commit sha)
```

## Licenses

- [SkyrimOutfitSystemSE](https://gitlab.com/metricexpansion/SkyrimOutfitSystemSE): `commonlibsse_ng` Rust code

  This code was originally under the CC-BY-NC-SA-4.0 license, but the author gave us permission to use the Rust parts they wrote under the MIT license as well.

  See [this issue](https://gitlab.com/metricexpansion/SkyrimOutfitSystemSE/-/issues/2#note_2332635556)

  - C++ & Papyrus: [CC-BY-NC-SA-4.0](https://gitlab.com/metricexpansion/SkyrimOutfitSystemSE/-/blob/master/LICENSE.md?ref_type=heads)

  - Rust code section: [MIT OR CC-BY-NC-SA-4.0](https://gitlab.com/metricexpansion/SkyrimOutfitSystemSE/-/blob/master/LICENSE.md?ref_type=heads)

- [CommonLibSSE-NG](https://github.com/CharmedBaryon/CommonLibSSE-NG): Generated code by binding, inherited docs.(src/bindings.rs)

  - [License: MIT](https://github.com/CharmedBaryon/CommonLibSSE-NG/blob/main/LICENSE)

- Other code: [MIT License](./LICENSE-MIT)

The code I wrote, such as `build.ps1`, is licensed under .

```rust
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT
```
