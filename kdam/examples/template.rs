use kdam::{tqdm, BarExt, Spinner};
use std::io::Result;

fn main() -> Result<()> {
    let mut pb = tqdm!(
        total = 300,
        ncols = 40,
        force_refresh = true,
        bar_format = "{desc suffix=' '}|{animation}| {spinner} {count}/{total} [{percentage:.0}%] in {elapsed human=true} ({rate:.1}/s, eta: {remaining human=true})",
        spinner = Spinner::new(
            &["▁▂▃", "▂▃▄", "▃▄▅", "▄▅▆", "▅▆▇", "▆▇█", "▇█▇", "█▇▆", "▇▆▅", "▆▅▄", "▅▄▃", "▄▃▂", "▃▂▁"],
            30.0,
            1.0,
        )
    );

    for _ in 0..300 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
        pb.update(1)?;
    }

    pb.set_bar_format("{desc suffix=' '}|{animation}| {count}/{total} [{percentage:.0}%] in {elapsed human=true} ({rate:.1}/s)").unwrap();
    pb.clear()?;
    pb.refresh()?;
    eprintln!();

    Ok(())
}
