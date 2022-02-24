# kdam

<p align="center">
  <a href="LICENSE" title="License: MIT"><img src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
  <a href="https://github.com/clitic/kdam"><img src="https://img.shields.io/github/repo-size/clitic/kdam.svg" alt="Repository Size"></a>
</p>

Instantly make your loops show a smart progress meter.

```rust
use kdam::tqdm;

fn main() {
    let mut pb = tqdm!(total = 100);
    for _ in 0..100 {
        pb.update(1);
    }
}
```

```
100%|█████████████████████████████| 100/100 [00:00<00:00, 728862.97it/s]
```

## Installations

```toml
kdam = "0.1.0"
```

## Usage

```rust
use kdam::tqdm;

fn main() {
    let pb = tqdm!(["Earth", "Mars", "Jupiter", "Saturn"].iter());

    for _ in pb {
    }
}
```

- Alternative Way

```rust
use kdam::BarIter;

fn main() {
    let pb = ["Earth", "Mars", "Jupiter", "Saturn"].iter().progress();

    for _ in pb {
    }
}
```

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

## License

&copy; 2022 clitic

This repository is licensed under the MIT license. See LICENSE for details.
