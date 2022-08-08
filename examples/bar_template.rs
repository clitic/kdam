use kdam::formatx::Template;
use kdam::prelude::*;
use kdam::Spinner;

// https://github.com/rsalmei/alive-progress
// Template::new("{desc}{percentage precision=2} |{animation}| {count}/{total} [{elapsed}<{remaining}, {rate}{postfix}]").unwrap()
fn main() {
    let mut pb = tqdm!(
        total = 300,
		ncols = 40 as i16,
        force_refresh = true,
        bar_format = "{desc suffix=' '}|{animation}| {spinner} {@count}/{@total} [{@percentage:.0}%] in {elapsed human=true} ({@rate:.1}/s, eta: {remaining human=true})".parse::<Template>().unwrap(),
        spinner = Spinner::new(
            &["▁▂▃", "▂▃▄", "▃▄▅", "▄▅▆", "▅▆▇", "▆▇█", "▇█▇", "█▇▆", "▇▆▅", "▆▅▄", "▅▄▃", "▄▃▂", "▃▂▁"],
            30.0,
            1.0,
        )
    );

    for _ in 0..300 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
        pb.update(1);
    }

    pb.set_bar_format("{desc suffix=' '}|{animation}| {@count}/{@total} [{@percentage:.0}%] in {elapsed human=true} ({@rate:.1}/s)".parse::<Template>().unwrap());
    pb.clear();
    pb.refresh();

    eprint!("\n");
}
