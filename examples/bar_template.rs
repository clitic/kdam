use kdam::formatx::Template;
use kdam::prelude::*;

// Template::new("{desc}{percentage precision=2} |{animation}| {count}/{total} [{elapsed}<{remaining}, {rate}{postfix}]").unwrap()
fn main() {
    let mut pb = tqdm!(
        total = 300,
        bar_format = Template::new("{@desc suffix=' '}|{animation}| {alive-spinner} {@count}/{@total} [{@percentage:.0}%] in {@elapsed:.0}s ({@rate:.1}/s, eta: {@remaining:.0}s)").unwrap(),
        force_refresh = true
    );

    for _ in 0..300 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
        pb.update(1);
    }

    pb.set_bar_format(Template::new("{@desc suffix=' '}|{animation}| {@count}/{@total} [{@percentage:.0}%] in {@elapsed:.0}s ({@rate:.1}/s)").unwrap());
    pb.refresh();

    eprint!("\n");
}
