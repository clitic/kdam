# kdam

<p align="center">
  <img src="https://img.shields.io/crates/d/kdam?style=flat-square">
  <img src="https://img.shields.io/crates/v/kdam?style=flat-square">
  <img src="https://img.shields.io/github/license/clitic/kdam?style=flat-square">
  <img src="https://img.shields.io/github/repo-size/clitic/kdam?style=flat-square">
  <img src="https://img.shields.io/tokei/lines/github/clitic/kdam?style=flat-square">
</p>

kdam is a port of [tqdm](https://github.com/tqdm/tqdm) library which is written in python. kdam has almost same features as tqdm with extra features included. Some features couldn't be ported due to language barriers. kdam is also 8-10 times faster than tqdm. kdam has only one external dependency which [terminal-size](https://github.com/eminence/terminal-size).

Instantly make your loops show a smart progress meter. Just wrap any iterator with tqdm!(iterator) macro and you're done!

```rust
use kdam::tqdm;

fn main() {
    for _ in tqdm!(0..100) {}
}
```

```
100%|█████████████████████████████| 100/100 [00:00<00:00, 25854.49it/s]
```

kdam also supports different animation styles. All available animations styles are:

[examples/showcase_animations.rs](https://github.com/clitic/kdam/blob/main/examples/showcase_animations.rs)

![showcase_animations](https://raw.githubusercontent.com/clitic/kdam/main/images/animations.gif)

[Fira Code](https://github.com/tonsky/FiraCode) is the first programming font to offer dedicated glyphs to render progress bars. kdam has an animation style to support it.

[examples/misc_fira_code.rs](https://github.com/clitic/kdam/blob/main/examples/misc_fira_code.rs)

![showcase_fira_code_animation](https://raw.githubusercontent.com/clitic/kdam/main/images/fira_code.gif)

## Installations

Add this to your Cargo.toml file.

```toml
[dependencies]
kdam = "0.1.2"
```

Or add from github main branch.

```toml
[dependencies]
kdam = { git = "https://github.com/clitic/kdam.git", branch = "main" }
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
/ 10000000 [00:03, 2998660.35it/s]
```

## Examples

### Description And Additional Stats

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

### Nested Progress Bars

`kdam` supports nested progress bars. For manual control over positioning (e.g. for multi-processing use), you may specify `position=n` where `n=0` for the outermost bar, `n=1` for the next, and so on.

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

### Writing Messages And Inputs

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

Similarly `.input()` method can be called to store an user input.

```rust
use kdam::tqdm;

fn main() {
    let mut pb = tqdm!(total = 10);

    for i in 0..10 {
        if i == 5 {
            if pb.input("Break Loop [y/n]: ").unwrap() == "y\r\n" {
                break;
            }
        }

        pb.update(1);
    }
}
```

```
Break Loop [y/n]: y
 50%|███████████████▎              | 5/10 [00:01<00:01, 3.83it/s]
```

## License

&copy; 2022 clitic

This repository is licensed under the MIT license. See LICENSE for details.
