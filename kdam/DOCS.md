A console progress bar library for Rust. (inspired by [tqdm](https://github.com/tqdm/tqdm) & [rich.progress](https://rich.readthedocs.io/en/latest/progress.html))

## Optional Features

The following are a list of [Cargo features](https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-features-section) that can be enabled or disabled:

- **derive**: Adds a derive macro for deriving [BarExt](crate::BarExt) trait.
- **notebook**: Enables support for jupyter notebooks. It can be useful when you are building [pyo3](https://crates.io/crates/pyo3) python extension modules. Also, make sure that you have [ipython](https://pypi.org/project/ipython) and [ipywidgets](https://pypi.org/project/ipywidgets) python libraries installed on your system.
- **gradient**: Enables gradient colours support for progress bars and printing text.
- **rayon**: Adds support for [rayon's](https://crates.io/crates/rayon) parallel iterators.
- **rich**: Enables [rich](https://rich.readthedocs.io/en/latest/progress.html) style progress bar.
- **spinner**: Enables support for using spinners. 
- **template**: Enables templating capabilities for [Bar](crate::Bar).
- **unicode**: Enables unicode support.

## Usage

### Iterator Based

```
use kdam::tqdm;

fn main() {
    let chars = ["a", "b", "c", "d"];
    let mut charset = String::new();

    for i in tqdm!(chars.iter()) {
        charset += i;
    }

    eprintln!();
    assert_eq!(charset, "abcd");
}
```

### Manual

```
use kdam::{tqdm, BarExt};
use std::io::Result;

fn main() -> Result<()> {
    let mut pb = tqdm!(total = 100);

    for _ in 0..100 {
        pb.update(1)?;
    }

    eprintln!();
    Ok(())
}
```

Another example without a total value. This only shows basic stats.

```
use kdam::{tqdm, BarExt};
use std::io::Result;

fn main() -> Result<()> {
    let mut pb = tqdm!();

    for _ in 0..10000000 {
        pb.update(1)?;
    }

    pb.refresh()?;
    eprintln!();

    Ok(())
}
```

```text
10000000 [00:03, 2998660.35it/s]
```

## Examples

### Description And Additional Stats

Custom information can be displayed and updated dynamically on `kdam` bars with the `desc` and `postfix`.

```
use kdam::{tqdm, BarExt};
use std::io::Result;

fn main() -> Result<()> {
    let mut pb = tqdm!(total = 10);
    pb.set_postfix(format!("str={}, lst={:?}", "h", [1, 2]));
    pb.refresh()?;

    for i in 0..10 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
        pb.set_description(format!("GEN {}", i));
        pb.update(1)?;
    }
    
    eprintln!();
    Ok(())
}
```

```text
GEN 4:  50%|█████████▎        | 5/10 [00:02<00:02, 1.95it/s, str=h, lst=[1, 2]]
```

### Nested Progress Bars

`kdam` supports nested progress bars. For manual control over positioning (e.g. for multi-processing use), you may specify `position=n` where `n=0` for the outermost bar, `n=1` for the next, and so on.

```
use kdam::{term, tqdm};
use std::io::Result;

fn main() -> Result<()> {
    term::init(false);
    term::hide_cursor()?;

    for _ in tqdm!(0..4, desc = "1st loop", position = 0) {
        for _ in tqdm!(0..5, desc = "2nd loop", position = 1) {
            for _ in tqdm!(0..50, desc = "3rd loop", position = 2) {
                std::thread::sleep(std::time::Duration::from_secs_f32(0.0001));
            }
        }
    }

    eprint!("{}", "\n".repeat(3));
    println!("completed!");

    Ok(())
}
```

```text
1st loop:  50%|███████▎      | 2/4 [00:08<00:08, 0.25it/s]
2nd loop:  60%|████████▌     | 3/5 [00:02<00:01, 1.25it/s]
3rd loop:   0%|▎               | 0/50 [00:00<00:00, ?it/s]
```

### Writing Messages And Inputs

Since `kdam` uses a simple printing mechanism to display progress bars, you should not write any message in the terminal using `println!()` while a progressbar is open.

To write messages in the terminal without any collision with `kdam` bar display, a `.write()` method is provided. This message will print at bar output location, which is stderr by default.

```
use kdam::{tqdm, BarExt};
use std::io::Result;

fn main() -> Result<()> {
    let mut pb = tqdm!(total = 10);

    for i in 0..10 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.1));

        pb.update(1)?;
        pb.write(format!("Done task {}", i))?;
    }

    eprintln!();
    Ok(())
}
```

```text
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

```
use kdam::{tqdm, BarExt};
use std::io::Result;

fn main() -> Result<()> {
    let mut pb = tqdm!(total = 10);

    for i in 0..10 {
        if i == 5 {
            if pb.input("Break Loop [y/n]: ")?.trim() == "y" {
                break;
            }
        }

        pb.update(1);
    }

    eprintln!();
    Ok(())
}
```

```text
Break Loop [y/n]: y
 50%|███████████████▎              | 5/10 [00:01<00:01, 3.83it/s]
```

### Terminal Colorization

kdam also provides a text colorization trait for printing coloured text in terminal. It can be used as an alternative to existing [colored](https://github.com/mackwic/colored) crate.

```
use kdam::term::Colorizer;

kdam::term::init(true);

println!("{}", "hello world!".colorize("bold red"));
println!("{}", "hello world!".colorize("bright white on blue"));
```
