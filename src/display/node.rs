use std::fmt::Display;

use crate::logic::LogicNode;

use super::{LogicFormatter, LogicFmtConfig, LogicDisplay};

impl<'a, K: Display> Display for LogicFormatter<'a, LogicNode<K>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        display_logic_node(self.logic, self.config, f, true)
    }
}

fn display_logic_node<K: Display>(
    node: &LogicNode<K>,
    config: &LogicFmtConfig,
    f: &mut std::fmt::Formatter<'_>,
    root: bool,
) -> std::fmt::Result {
    match node {
        LogicNode::And(nodes) => display_logic_vec(nodes, f, &config, true, root),
        LogicNode::Or(nodes) => display_logic_vec(nodes, f, &config, false, root),
        LogicNode::KeyValue(node) => write!(f, "{}", node.format(config)),
    }
}

fn display_logic_vec<K: Display>(
    nodes: &[LogicNode<K>],
    f: &mut std::fmt::Formatter<'_>,
    config: &LogicFmtConfig,
    and: bool,
    root: bool,
) -> std::fmt::Result {
    if !root && nodes.len() != 1 {
        write!(f, "{}", config.brackets.0)?;
    }
    if let Some(first) = nodes.first() {
        display_logic_node(first, config, f, false)?;
        for rest in nodes.iter().skip(1) {
            let top_level_sep = if root {&config.top_level_sep} else {""};
            let logic_sep = if and {&config.and} else {&config.or};
            write!(f, "{}{}", top_level_sep, logic_sep)?;
            display_logic_node(rest, config, f, false)?;
        }
    }
    if !root && nodes.len() != 1 {
        write!(f, "{}", config.brackets.1)?;
    }
    Ok(())
}
