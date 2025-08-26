use indicatif::ProgressBar;
use std::time::Duration;

pub fn step<F, T>(pb: &ProgressBar, label: String, work: F) -> T
where
    F: FnOnce() -> T,
{
    pb.println(label.clone());
    pb.set_message(label.clone());
    pb.enable_steady_tick(Duration::from_millis(80));
    let out = work();
    pb.disable_steady_tick();

    pb.inc(1);
    out
}
