use super::{LogicType, KeyValue, Logical, LogicNode};

#[derive(PartialEq, Debug)]
pub struct LogicChain<K> {
    pub(crate) first: KeyValue<K>,
    pub(crate) rest: Vec<(Logical, KeyValue<K>)>,
}

impl<K> LogicType for LogicChain<K> {
    type Key = K;
    type Gats<G> = LogicChain<G>;
    fn with_keys<K2, E, F: Copy+Fn(&Self::Key)->Result<K2, E>>(&self, f: F) -> Result<Self::Gats<K2>, E> {
        let first = self.first.convert_with(&f)?;
        let rest: Result<_, E> = self.rest.iter().map(|(logic, kv)| Ok((*logic, kv.convert_with(&f)?))).collect();
        Ok(LogicChain {
            first,
            rest: rest?, 
        })
    }
}

impl<K: Clone> LogicChain<K> {
    pub fn logic_nodes<K2: From<K>>(self) -> LogicNode<K2> {
        if self.rest.is_empty() {
            LogicNode::KeyValue(self.first.convert())
        } else {
            fn flush_or_section<K>(or_section: &mut Vec<KeyValue<K>>) -> Option<LogicNode<K>> {
                Some(match or_section.len() {
                    1 => LogicNode::KeyValue(or_section.remove(0)),
                    2..=usize::MAX => {
                        LogicNode::Or(or_section.drain(..).map(LogicNode::KeyValue).collect())
                    }
                    _ => return None,
                })
            }
            let mut or_section = vec![self.first.convert()];
            let mut and_section: Vec<LogicNode<K2>> = Vec::new();
            for (logical, kv) in self.rest {
                match logical {
                    Logical::Or => {}
                    Logical::And => {
                        if let Some(node) = flush_or_section(&mut or_section) {
                            and_section.push(node);
                        }
                    }
                }
                or_section.push(kv.convert());
            }
            if and_section.is_empty() {
                flush_or_section(&mut or_section).unwrap()
            } else {
                if let Some(node) = flush_or_section(&mut or_section) {
                    and_section.push(node);
                }
                LogicNode::And(and_section)
            }
        }
    }
}
