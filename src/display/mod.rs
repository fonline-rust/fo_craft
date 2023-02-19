mod chain;
mod node;
mod config;

use std::fmt::Display;

pub use config::LogicFmtConfig;

use crate::logic::KeyValue;

pub struct LogicFormatter<'a, L> {
    logic: &'a L,
    config: &'a LogicFmtConfig,
}
impl<'a, L> LogicFormatter<'a, L> {
    pub(crate) fn new(logic: &'a L, config: &'a LogicFmtConfig) -> Self {
        Self{config, logic}
    }
}

impl<'a, K: Display> Display for LogicFormatter<'a, KeyValue<K>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self{logic, config} = *self;
        if (config.show_value)(logic.value) {
            write!(f, "{}{}{}{}", logic.key, config.pre_value, logic.value, config.post_value)
        } else {
            write!(f, "{}", logic.key)
        }
    }
}

pub trait LogicDisplay: Sized {
    fn format<'d>(&'d self, config: &'d LogicFmtConfig) -> LogicFormatter<'d, Self>;
    fn display(&self, config: &LogicFmtConfig) -> String
    where 
        for<'a> LogicFormatter<'a, Self>: Display
    {
        self.format(config).to_string()
    }
}

impl<L> LogicDisplay for L
where
    for<'a> LogicFormatter<'a, Self>: Display
{
    fn format<'d>(&'d self, config: &'d LogicFmtConfig) -> LogicFormatter<'d, Self> {
        LogicFormatter::new(self, config)
    }
}
