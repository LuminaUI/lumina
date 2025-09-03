use console::{Emoji, style};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle, style::TemplateError};
use std::time::Duration;

pub static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
pub static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
pub static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
pub static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "");

pub struct Step {
    pb: ProgressBar,
    step: u64,
    steps: u64,
}

impl Step {
    pub fn new(multi: &MultiProgress, len: u64, steps: u64) -> Result<Self, TemplateError> {
        let pb = multi.add(ProgressBar::new(len));

        pb.set_style(
            ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")?
                .progress_chars("=>-"),
        );

        Ok(Self { pb, step: 1, steps })
    }

    #[inline(always)]
    pub fn inc(&mut self) {
        self.step += 1;
    }

    pub fn step_before<M>(&self, emoji: Emoji, message: M)
    where
        M: AsRef<str>,
    {
        let message = format!(
            "{} {emoji} {}",
            style(format!("{}/{}", self.step, self.steps)).bold().dim(),
            message.as_ref()
        );

        self.pb.set_message(message);
        self.pb.enable_steady_tick(Duration::from_millis(80));
        self.pb.tick();
    }

    pub fn step_before_no_tick<M>(&self, emoji: Emoji, message: M)
    where
        M: AsRef<str>,
    {
        let message = format!(
            "{} {emoji} {}",
            style(format!("{}/{}", self.step, self.steps)).bold().dim(),
            message.as_ref()
        );

        self.pb.set_message(message);
        self.pb.tick();
    }

    pub fn step_after(&self) {
        self.pb.disable_steady_tick();
        self.pb.inc(1);
    }

    pub fn finish_with<M>(&self, emoji: Emoji, message: M)
    where
        M: AsRef<str>,
    {
        let message = format!(
            "{} {emoji} {}",
            style(format!("{}/{}", self.step, self.steps)).bold().dim(),
            message.as_ref()
        );

        self.pb.finish_with_message(message);
    }

    #[inline(always)]
    pub fn abandon(&self) {
        self.pb.abandon();
    }
}

#[macro_export]
macro_rules! step {
    ($step:expr,$emoji:expr,$message:expr) => {
        $step.step_before_no_tick($emoji, $message);
        $step.step_after();
    };

    ($step:expr,$emoji:expr,$message:expr,$work:expr) => {
        $step.step_before($emoji, $message);
        $work;
        $step.step_after();
    };
}

pub use step;

#[macro_export]
macro_rules! inc_step {
    ($step:expr,$emoji:expr,$message:expr) => {
        $step.inc();
        $step.step_before_no_tick($emoji, $message);
        $step.step_after();
    };

    ($step:expr,$emoji:expr,$message:expr,$work:expr) => {
        $step.inc();
        $step.step_before($emoji, $message);
        $work;
        $step.step_after();
    };
}
