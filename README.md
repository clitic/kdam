# kdam

<p align="center">
  <a href="LICENSE" title="License: MIT"><img src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
  <a href="https://github.com/clitic/kdam"><img src="https://img.shields.io/github/repo-size/clitic/kdam.svg" alt="Repository Size"></a>
</p>

This library is inspired by [tqdm](https://github.com/tqdm/tqdm).

Instantly make your loops show a smart progress meter - just wrap any iterator with tqdm!(iterable), and you're done!

```rust
use kdam::tqdm;

fn main() {
    for _ in tqdm!(0..100) {}
}
```

```
100%|█████████████████████████████| 100/100 [00:00<00:00, 25854.49it/s]
```

## Installations

```toml
kdam = "0.1.0"
```

## Usage

### Iterator Based

```rust
use kdam::tqdm;

fn main() {
    let chars = ["a", "b", "c", "d"];
    let mut charset = String::new();

    for i in tqdm!(chars.iter()) {
        charset += i;
    }

    assert_eq!(charset, "abcd");
}
```

<details>
  <summary>Alternative Way</summary>

```rust
use kdam::BarIter;

fn main() {
    let chars = ["a", "b", "c", "d"];
    let mut charset = String::new();

    for i in chars.iter().progress() {
        charset += i;
    }

    assert_eq!(charset, "abcd");
}
```

</details>

### Manual

```rust
use kdam::tqdm;

fn main() {
    let mut pb = tqdm!(total = 100);
    for _ in 0..100 {
        pb.update(1);
    }
}
```

> If total is unknown progress bar will not be displayed.

```rust
use kdam::tqdm;

fn main() {
    let mut pb = tqdm!();

    for _ in 0..10000000 {
        pb.update(1);
    }
    pb.refresh();
}
```

```
10000000 [00:03, 2998660.35it/s]
```

## Examples And Advance Usage

- [Description and additional stats](https://github.com/clitic/kdam/blob/main/examples/desc_stats.rs)
- [Nested progress bar](https://github.com/clitic/kdam/blob/main/examples/nested.rs)
- [Multiple progress bar](https://github.com/clitic/kdam/blob/main/examples/multiple.rs)

- [Download a file](https://github.com/clitic/kdam/blob/main/examples/file_download/src/main.rs)
- [Copy a file](https://github.com/clitic/kdam/blob/main/examples/file_io.rs)

## License

&copy; 2022 clitic

This repository is licensed under the MIT license. See LICENSE for details.
