# kdam

<p align="center">
  <a href="LICENSE" title="License: MIT"><img src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
  <a href="https://github.com/clitic/kdam"><img src="https://img.shields.io/github/repo-size/clitic/kdam.svg" alt="Repository Size"></a>
</p>

Instantly make your loops show a smart progress meter.

```rust
use kdam::Bar;

fn main() {
    let mut pb = Bar::new(100);
    
    for _ in 0..100 {
        pb.update(1);
    }
}
```

```
100%|███████████████████████████████| 100/100 [00:00<00:00, 728862.97it/s]
```

## Installations

```bash
[dependencies]
kdam = "0.1.0"
```

## Usage

```rust
use kdam::BarIter;

fn main() {
    let pb = ["Earth", "Mars", "Saturn", "Jupiter"].iter().progress();

    for _ in pb {
    }
}
```

## License

&copy; 2022 clitic

This repository is licensed under the MIT license. See LICENSE for details.
