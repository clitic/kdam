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

Note that in Python 3, `tqdm.write` is thread-safe.

```python
from time import sleep
from tqdm import tqdm, trange
from concurrent.futures import ThreadPoolExecutor

L = list(range(9))

def progresser(n):
    interval = 0.001 / (n + 2)
    total = 5000
    text = "#{}, est. {:<04.2}s".format(n, interval * total)
    for _ in trange(total, desc=text):
        sleep(interval)
    if n == 6:
        tqdm.write("n == 6 completed.")
        tqdm.write("`tqdm.write()` is thread-safe in py3!")

if __name__ == '__main__':
    with ThreadPoolExecutor() as p:
        p.map(progresser, L)
```

### Dynamic Monitor / Meter

You can use a `kdam` as a meter which is not monotonically increasing. This could be because `n` decreases (e.g. a CPU usage monitor) or `total` changes.

One example would be recursively searching for files. The `total` is the number of objects found so far, while `n` is the number of those objects which are files (rather than folders).

```python
from tqdm import tqdm
import os.path

def find_files_recursively(path, show_progress=True):
    files = []
    # total=1 assumes `path` is a file
    t = tqdm(total=1, unit="file", disable=not show_progress)
    if not os.path.exists(path):
        raise IOError("Cannot find:" + path)

    def append_found_file(f):
        files.append(f)
        t.update()

    def list_found_dir(path):
        """returns os.listdir(path) assuming os.path.isdir(path)"""
        listing = os.listdir(path)
        # subtract 1 since a "file" we found was actually this directory
        t.total += len(listing) - 1
        # fancy way to give info without forcing a refresh
        t.set_postfix(dir=path[-10:], refresh=False)
        t.update(0)  # may trigger a refresh
        return listing

    def recursively_search(path):
        if os.path.isdir(path):
            for f in list_found_dir(path):
                recursively_search(os.path.join(path, f))
        else:
            append_found_file(path)

    recursively_search(path)
    t.set_postfix(dir=path)
    t.close()
    return files
```

Using `update(0)` is a handy way to let `kdam` decide when to trigger a display refresh to avoid console spamming.

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
