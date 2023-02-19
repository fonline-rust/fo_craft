use super::{KeyValue, LogicType};

#[derive(Debug, PartialEq)]
pub enum LogicNode<K> {
    And(Vec<LogicNode<K>>),
    Or(Vec<LogicNode<K>>),
    KeyValue(KeyValue<K>),
}

impl<K> LogicType for LogicNode<K> {
    type Key = K;
    type Gats<G> = LogicNode<G>;

    fn with_keys<K2, E, F: Copy+Fn(&Self::Key)->Result<K2, E>>(&self, f: F) -> Result<Self::Gats<K2>, E> {
        let with_nodes = |nodes: &[LogicNode<K>]| nodes.iter().map(|node| node.with_keys(f)).collect::<Result<_, E>>();
        Ok(match self {
            LogicNode::And(nodes) => LogicNode::And(with_nodes(nodes)?),
            LogicNode::Or(nodes) => LogicNode::Or(with_nodes(nodes)?),
            LogicNode::KeyValue(kv) => LogicNode::KeyValue(kv.convert_with(&f)?),
        })
    }
}
