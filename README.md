# kdam

<p align="center">
  <a href="LICENSE" title="License: MIT"><img src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
  <a href="https://github.com/clitic/kdam"><img src="https://img.shields.io/github/repo-size/clitic/kdam.svg" alt="Repository Size"></a>
</p>

This library is inspired by [tqdm](https://github.com/tqdm/tqdm) but 10 times faster than it.

Instantly make your loops show a smart progress meter - just wrap any iterator with tqdm!(iterator), and you're done!

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

Another example without a total value. This only shows basic stats.

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

## Examples

### Description and additional stats

Custom information can be displayed and updated dynamically on `kdam` bars with the `desc` and `postfix`.

```rust
use kdam::tqdm;

fn main() {
    let mut pb = tqdm!(total = 10);
    pb.refresh();

    for i in 0..10 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.5));

        pb.set_description(format!("GEN {}", i));
        pb.set_postfix(format!("str={}, lst={:?}", "h", [1, 2]));
        pb.update(1);
    }
}
```

```
GEN 4:  50%|█████████▎        | 5/10 [00:02<00:02, 1.95it/s, str=h, lst=[1, 2]]
```

### Nested progress bars

`kdam` supports nested progress bars. For manual control over positioning (e.g. for multi-processing use), you
may specify `position=n` where `n=0` for the outermost bar, `n=1` for
the next, and so on.

```rust
use kdam::tqdm;

fn main() {
    for _ in tqdm!(0..4, desc = "1st loop".to_string(), position = 0) {
        for _ in tqdm!(0..5, desc = "2nd loop".to_string(), position = 1) {
            for _ in tqdm!(0..50, desc = "3rd loop".to_string(), position = 2) {
                std::thread::sleep(std::time::Duration::from_secs_f32(0.0001));
            }
        }
    }
    print!("{}", "\n".repeat(3));
    println!("completed!");
}
```

```
1st loop:  50%|███████▎      | 2/4 [00:08<00:08, 0.25it/s]
2nd loop:  60%|████████▌     | 3/5 [00:02<00:01, 1.25it/s]
3rd loop:   0%|▎               | 0/50 [00:00<00:00, ?it/s]
```

### Writing Messages

Since `kdam` uses a simple printing mechanism to display progress bars, you should not write any message in the terminal using `println!()` while a progressbar is open.

To write messages in the terminal without any collision with `kdam` bar display, a `.write()` method is provided.

```rust
use kdam::tqdm;

fn main() {
    let mut pb = tqdm!(total = 10);

    for i in 0..10 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.1));

        pb.update(1);
        pb.write(format!("Done task {}", i));
    }
}
```

By default, this will print to standard output.

```
Done task 0
Done task 1
Done task 2
Done task 3
Done task 4
Done task 5
Done task 6
Done task 7
Done task 8
Done task 9
100%|███████████████████████████| 10/10 [00:02<00:00, 4.31it/s]
```

## License

&copy; 2022 clitic

This repository is licensed under the MIT license. See LICENSE for details.