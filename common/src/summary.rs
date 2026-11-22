use crate::{
    DayFn,
    data::{Answers, Input},
};
use alloc::{borrow::ToOwned, boxed::Box};
use core::{
    fmt::{Display, Formatter, Result},
    time::Duration,
};
use derive_more::{Deref, DerefMut};
use std::time::SystemTime;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DaySummary {
    pub day: usize,
    pub time: Duration,
    pub provided_answers: Answers,
    pub correct_answers: Answers,
}

impl DaySummary {
    #[must_use]
    pub fn is_correct(&self) -> bool {
        self.provided_answers == self.correct_answers
    }
}

impl Display for DaySummary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.is_correct() {
            write!(
                f,
                "Day {} - {:?} - {}",
                self.day, self.time, self.correct_answers
            )
        } else {
            write!(
                f,
                "Incorrect answer for Day {}! Expected {}, received {}",
                self.day, self.correct_answers, self.provided_answers
            )
        }
    }
}

#[derive(Clone, Debug, Deref, DerefMut, Eq, Hash, PartialEq)]
pub struct YearSummary(pub Box<[DaySummary]>);

impl YearSummary {
    pub fn generate(data_dir: &str, day_fns: &[DayFn]) -> Self {
        Self(
            day_fns
                .iter()
                .enumerate()
                .map(|(day_index, day_fn)| {
                    let day = day_index + 1;

                    let input = Input::load(data_dir, day);
                    let correct_answers = Answers::load_correct(data_dir, day);

                    let timer = SystemTime::now();
                    let output_raw = day_fn(&input);
                    let time = timer.elapsed().unwrap();

                    let provided_answers = Answers(
                        output_raw.map(|o| str::from_utf8(o.as_slice()).unwrap().to_owned()),
                    );

                    DaySummary {
                        day,
                        time,
                        provided_answers,
                        correct_answers,
                    }
                })
                .collect(),
        )
    }

    #[must_use]
    pub fn total_time(&self) -> Duration {
        self.iter().map(|d| d.time).sum()
    }
}

impl Display for YearSummary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for day in self.iter() {
            writeln!(f, "{day}")?;
        }

        write!(f, "Total - {:?}", self.total_time())
    }
}
