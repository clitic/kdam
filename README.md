<h1 align="center">kdam</h1>

<p align="center">
  <a href="https://crates.io/crates/kdam">
    <img src="https://img.shields.io/crates/d/kdam?style=flat-square">
  </a>
  <a href="https://crates.io/crates/kdam">
    <img src="https://img.shields.io/crates/v/kdam?style=flat-square">
  </a>
  <a href="https://github.com/clitic/kdam">
    <img src="https://img.shields.io/github/actions/workflow/status/clitic/kdam/tests.yml?logo=github&style=flat-square">
  </a>
  <a href="https://docs.rs/kdam/latest/kdam">
    <img src="https://img.shields.io/docsrs/kdam?logo=docsdotrs&style=flat-square">
  </a>
  <a href="https://github.com/clitic/kdam#license">
    <img src="https://img.shields.io/crates/l/kdam?style=flat-square">
  </a>
  <a href="https://github.com/clitic/kdam">
    <img src="https://img.shields.io/github/repo-size/clitic/kdam?style=flat-square">
  </a>
  <a href="https://github.com/clitic/kdam">
    <img src="https://img.shields.io/tokei/lines/github/clitic/kdam?logo=github&style=flat-square">
  </a>
</p>

kdam is a console progress bar library for rust. It is port of [tqdm](https://github.com/tqdm/tqdm) library which is written in python. kdam supports all features of tqdm except few. Some features of tqdm can't be ported directly so they are implemented in different way like `RowManager` which manages multiple progress bars but in tqdm progress bars are automatically managed using `nrows`. In addition to tqdm existing features kdam also provides extra features such as spinners, charset with fill, gradient colours etc. Since kdam is written in rust its upto 4 times faster than tqdm.

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

kdam also supports different animation styles. All available animation styles are:

[examples/showcase/animations.rs](https://github.com/clitic/kdam/blob/main/examples/showcase/animations.rs)
![showcase_animations](https://raw.githubusercontent.com/clitic/kdam/main/images/animations.gif)

kdam also supports [rich.progress](https://rich.readthedocs.io/en/latest/progress.html) style bars with customizable columns.

[examples/rich.rs](https://github.com/clitic/kdam/blob/main/examples/rich.rs)
![showcase_rich_progress_animation](https://raw.githubusercontent.com/clitic/kdam/main/images/rich_progress.gif)

kdam doesn't restrict you to use default progress bar styles. You can create your own progress bar template using bar_format. Here is clone of [alive-progress](https://github.com/rsalmei/alive-progress) using kdam.

[examples/template.rs](https://github.com/clitic/kdam/blob/main/examples/template.rs)
![showcase_alive_progress_template](https://raw.githubusercontent.com/clitic/kdam/main/images/template.gif)

A gradient progress bar can also be created.

[examples/coloured/gradient.rs](https://github.com/clitic/kdam/blob/main/examples/coloured/gradient.rs)
![showcase_gradient](https://raw.githubusercontent.com/clitic/kdam/main/images/gradient.gif)

## Getting Started

Add this to your Cargo.toml file.

```toml
[dependencies]
kdam = "0.3.0"
```

Or add from command line.

```bash
$ cargo add kdam
```

See [docs](https://docs.rs/kdam/latest/kdam) and [examples](https://github.com/clitic/kdam/tree/main/examples) to 
know how to use it.

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

    eprint!("\n");
    assert_eq!(charset, "abcd");
}
```

### Manual

```rust
use kdam::{tqdm, BarExt};

fn main() {
    let mut pb = tqdm!(total = 100);

    for _ in 0..100 {
        pb.update(1);
    }

    eprint!("\n");
}
```

Another example without a total value. This only shows basic stats.

```rust
use kdam::{tqdm, BarExt};

fn main() {
    let mut pb = tqdm!();

    for _ in 0..10000000 {
        pb.update(1);
    }
    pb.refresh();

    eprint!("\n");
}
```

```
10000000 [00:03, 2998660.35it/s]
```

## Examples

### Description And Additional Stats

Custom information can be displayed and updated dynamically on `kdam` bars with the `desc` and `postfix`.

```rust
use kdam::{tqdm, BarExt};

fn main() {
    let mut pb = tqdm!(total = 10);
    pb.set_postfix(format!("str={}, lst={:?}", "h", [1, 2]));
    pb.refresh();

    for i in 0..10 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
        pb.set_description(format!("GEN {}", i));
        pb.update(1);
    }
    
    eprint!("\n");
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
    for _ in tqdm!(0..4, desc = "1st loop", position = 0) {
        for _ in tqdm!(0..5, desc = "2nd loop", position = 1) {
            for _ in tqdm!(0..50, desc = "3rd loop", position = 2) {
                std::thread::sleep(std::time::Duration::from_secs_f32(0.0001));
            }
        }
    }

    eprint!("{}", "\n".repeat(3));
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

To write messages in the terminal without any collision with `kdam` bar display, a `.write()` method is provided. This message will print at bar output location, which is stderr by default.

```rust
use kdam::{tqdm, BarExt};

fn main() {
    let mut pb = tqdm!(total = 10);

    for i in 0..10 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.1));

        pb.update(1);
        pb.write(format!("Done task {}", i));
    }

    eprint!("\n");
}
```

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
use kdam::{tqdm, BarExt};

fn main() {
    let mut pb = tqdm!(total = 10);

    for i in 0..10 {
        if i == 5 {
            if pb.input("Break Loop [y/n]: ").unwrap().trim() == "y" {
                break;
            }
        }

        pb.update(1);
    }

    eprint!("\n");
}
```

```
Break Loop [y/n]: y
 50%|███████████████▎              | 5/10 [00:01<00:01, 3.83it/s]
```

### Terminal Colorization

kdam also provides a text colorization trait for printing colored text in terminal. It can be used as an alternative for existing [colored](https://github.com/mackwic/colored) crate. Note that tty detection is not implemented, an alternative is to use [atty](https://github.com/softprops/atty) crate to detect if stream is tty and then use [set_colorize](https://docs.rs/kdam/latest/kdam/term/fn.set_colorize.html) function.

```rust
use kdam::term::Colorizer;

println!("{}", "hello world!".colorize("bold red"));
println!("{}", "hello world!".colorize("bright white on blue"));
```

## License

Dual Licensed

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](LICENSE-APACHE))
- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](LICENSE-MIT))
