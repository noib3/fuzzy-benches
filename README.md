# Benchmarks of fuzzy-matching libraries

The following are a series of benchmarks comparing the performance of various
fuzzy-matching libraries written in C and Rust. We're comparing
[`fuzzy-matcher`][fuzzy-matcher], [`norm`][norm], [`nucleo-matcher`][nucleo],
[`sublime_fuzzy`][sublime-fuzzy] and
[`telescope-fzf-native`][telescope-fzf-native] on 4 benchmarks:

- `emacs`: matches the word `"emacs"` against a list of 95655 possible
  haystacks with a median length of 37 characters. This dataset was taken
  from [this][emacs-completion-bench] repo;

- `e(macs)`: same as `emacs`, but here the needle is simply `"e"` instead of
  `"emacs"`. This was added because some libraries have fast paths for
  single-character needles;

- `medium-middle` matches a needle of length 5 that occurs in the middle of a
  haystack of length 200;

- `long-middle` matches a needle of length 5 that occurs in the middle of a
  haystack of length 501;

All the benchmarks were run on a 2018 MacBook Pro with a 2.2 GHz 6-Core i7.

## `emacs`

Run with `cargo run --release -- --bench 'emacs/[^/]*$'`.

| Metric                  | Time (ms) | Relative |
| ----------------------- | --------- | -------- |
| `fuzzy-matcher-clangd`  | 34.24     | 5.08     |
| `fuzzy-matcher-skim-v2` | 26.53     | 3.94     |
| `norm-fzf-v1`           | 5.15      | 0.76     |
| `norm-fzf-v2`           | 6.74      | 1        |
| `nucleo-matcher`        | 7.43      | 1.10     |
| `sublime_fuzzy`         | 242.8     | 36.02    |
| `telescope-fzf-native`  | 9.10      | 1.35     |

## `e(macs)`

Run with `cargo run --release -- --bench 'e-macs/[^/]*$'`.

| Metric                  | Time (ms) | Relative |
| ----------------------- | --------- | -------- |
| `fuzzy-matcher-clangd`  | 86.48     | 12.7     |
| `fuzzy-matcher-skim-v2` | 62.99     | 9.25     |
| `norm-fzf-v1`           | 3.20      | 0.47     |
| `norm-fzf-v2`           | 6.81      | 1        |
| `nucleo-matcher`        | 6.74      | 0.99     |
| `sublime_fuzzy`         | 170.82    | 25.08    |
| `telescope-fzf-native`  | 15.05     | 2.21     |

## `medium-middle`

Run with `cargo run --release -- --bench 'medium-middle/[^/]*$'`.

| Metric                  | Time (µs) | Relative |
| ----------------------- | --------- | -------- |
| `fuzzy-matcher-clangd`  | 9.12      | 14.71    |
| `fuzzy-matcher-skim-v2` | 4.48      | 7.23     |
| `norm-fzf-v1`           | 0.26      | 0.42     |
| `norm-fzf-v2`           | 0.62      | 1        |
| `nucleo-matcher`        | 0.98      | 1.58     |
| `sublime_fuzzy`         | 16.55     | 26.69    |
| `telescope-fzf-native`  | 1.34      | 2.16     |

## `long-middle`

Run with `cargo run --release -- --bench 'long-middle/[^/]*$'`.

| Metric                  | Time (µs) | Relative |
| ----------------------- | --------- | -------- |
| `fuzzy-matcher-clangd`  | 21.49     | 13.6     |
| `fuzzy-matcher-skim-v2` | 9.31      | 5.89     |
| `norm-fzf-v1`           | 0.10      | 0.06     |
| `norm-fzf-v2`           | 1.58      | 1        |
| `nucleo-matcher`        | 3.02      | 1.91     |
| `sublime_fuzzy`         | 105.32    | 66.66    |
| `telescope-fzf-native`  | 3.40      | 2.15     |

[emacs-completion-bench]: https://github.com/axelf4/emacs-completion-bench
[fuzzy-matcher]: https://github.com/lotabout/fuzzy-matcher
[norm]: https://github.com/nomad/norm
[nucleo]: https://github.com/helix-editor/nucleo
[sublime-fuzzy]: https://github.com/Schlechtwetterfront/fuzzy-rs
[telescope-fzf-native]: https://github.com/nvim-telescope/telescope-fzf-native.nvim
