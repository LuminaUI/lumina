use indicatif::ProgressBar;
use std::time::Duration;

pub struct Spinner {
    pub text: String,
}

impl Spinner {
    pub fn spinner(text: &'static str) -> ProgressBar {
        let spinner = ProgressBar::new_spinner();

        spinner.set_message(text);

        spinner.enable_steady_tick(Duration::from_millis(80));

        spinner
    }
}
