//! Items relating to defining individual step of the cli
//!
//! The goal here is to give the user feedback as to how long a step is taking and how many are
//! left.
use std::{
    sync::{Arc, OnceLock, atomic::AtomicBool},
    thread,
    time::Instant,
    vec,
};

/// If anything should actually be printed by the steps
pub static INTERACTIVE: OnceLock<bool> = OnceLock::new();

/// A builder to conveniently create multiple [`Step`]s
///
/// This type implements [`Iterator`]. To proceed to the next step call next.
pub struct StepBuilder {
    /// The current step
    current_num: u8,
    /// The maximum amount of steps
    ///
    /// This is equal to the len of [`Self::names`]
    max_num: u8,
    /// An iterator of the names of the different steps
    names: vec::IntoIter<String>,
    /// The bool to signal to the last step that it is finished
    last_step_bool: Option<Arc<AtomicBool>>,
}

impl StepBuilder {
    /// Create a new [`Self`] taking the names for each step.
    pub fn new(names: Vec<String>) -> Self {
        Self {
            max_num: u8::try_from(names.len()).unwrap(),
            current_num: u8::default(),
            names: names.into_iter(),
            last_step_bool: None,
        }
    }
}

impl Iterator for StepBuilder {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(bool) = self.last_step_bool.as_ref() {
            bool.store(true, std::sync::atomic::Ordering::Relaxed);
        } else {
            debug_assert_eq!(self.current_num, 0);
        }

        if self.current_num >= self.max_num {
            return None;
        }

        self.current_num += 1;

        self.last_step_bool = Some(
            Step(Created(StepData {
                text: self.names.next().expect(
                    "Next should never fail here since the constructor ensures sufficient length.",
                ),
                num: self.current_num,
                max_num: self.max_num,
            }))
            .start()
            .0
            .0,
        );

        Some(())
    }
}

/// A single step in the cli process
///
/// It uses a background thread to print to stdout the time taken for the current step.
///
/// To create a step see [`StepBuilder`]
#[derive(Debug)]
pub struct Step<T>(T);

/// State and data of [`Step`] when it is created
///
/// See:
///
/// - [`StepData`]
#[derive(Debug)]
pub struct Created(StepData);
/// State and data of [`Step`] when it is started
///
/// The internal bool is used to signal to the background thread when to stop.
///
/// See:
///
/// - [`Step<Started>::finish`]
#[derive(Debug)]
pub struct Started(Arc<AtomicBool>);

/// Data for the step
///
/// This is passed to the thread on running [`Step<Created>::start`]
#[derive(Debug)]
struct StepData {
    /// Text describing what the step is
    text: String,
    /// The number of the step
    num: u8,
    /// The maximum number of any step
    max_num: u8,
}

impl Step<Created> {
    /// Start this step
    pub fn start(self) -> Step<Started> {
        let done = Arc::new(AtomicBool::default());

        let thread_done = done.clone();
        thread::spawn(move || {
            if !INTERACTIVE.get_or_init(|| true) {
                return;
            }

            let start_time = Instant::now();

            print!(
                "({}/{}) {}:  ",
                self.0.0.num, self.0.0.max_num, self.0.0.text
            );

            let mut last_time_len = 0;
            while !thread_done.load(std::sync::atomic::Ordering::Relaxed) {
                let time =
                    ((start_time.elapsed().as_secs_f32() * 100.0).round() / 100.0).to_string();

                let backspaces = "\x08".repeat(last_time_len + 1);

                last_time_len = time.len();

                print!("{backspaces}{time}s");
            }

            println!(
                "{} (Finished){}",
                anstyle::AnsiColor::Green.render_fg(),
                anstyle::Reset.render()
            );
        });

        Step(Started(done))
    }
}
