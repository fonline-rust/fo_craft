use std::fmt::Display;

use crate::logic::{LogicChain, KeyValue, Logical};

use super::{LogicFormatter, LogicDisplay};

impl<'a, K: Display> Display for LogicFormatter<'a, LogicChain<K>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self{logic, config} = *self;
        let mut or_section = vec![&logic.first];
        let mut multi_and = false;
        let mut first_flush = true;
        let mut flush = |or_section: &[&KeyValue<K>], in_and: bool| -> std::fmt::Result {
            if first_flush {
                first_flush = false;
            } else if !or_section.is_empty() {
                write!(f, "{}", config.top_level_sep)?;
            }
            if in_and {
                multi_and = true;
            }
            if or_section.len() > 1 {
                if multi_and {
                    write!(f, "{}", config.brackets.0)?;
                }
                write!(f, "{}", or_section[0].format(config))?;
                for &kv in &or_section[1..] {
                    write!(f, "{}{}", config.or, kv.format(config))?;
                }
                if multi_and {
                    write!(f, "{}", config.brackets.1)?;
                }
            } else if or_section.len() > 0 {
                write!(f, "{}", or_section[0].format(config))?;
            }
            if in_and {
                write!(f, "{}", config.and)?;
            }
            Ok(())
        };

        for (logical, kv) in &logic.rest {
            match logical {
                Logical::Or => {
                    or_section.push(kv);
                }
                Logical::And => {
                    flush(&or_section, true)?;
                    or_section.clear();
                    or_section.push(kv);
                }
            }
        }
        flush(&or_section, false)?;
        Ok(())
    }
}
