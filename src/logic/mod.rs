mod node;
mod chain;

pub use self::{
    node::LogicNode,
    chain::LogicChain,
};

pub trait LogicType: Sized {
    type Key;
    type Gats<G>;
    fn with_keys<K2, E, F: Copy+Fn(&Self::Key)->Result<K2, E>>(&self, f: F) -> Result<Self::Gats<K2>, E>;
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Logical {
    And,
    Or,
}

#[derive(PartialEq, Debug, Clone)]
pub struct KeyValue<K> {
    pub(crate) key: K,
    pub(crate) value: u32,
}
impl<K: Clone> KeyValue<K> {
    fn convert<K2: From<K>>(self) -> KeyValue<K2> {
        KeyValue { key: self.key.into(), value: self.value }
    }
}
impl<K> KeyValue<K> {
    fn convert_with<K2, E, F: Fn(&K)->Result<K2, E>>(&self, f: F) -> Result<KeyValue<K2>, E> {
        Ok(KeyValue { key: f(&self.key)?, value: self.value })
    }
}
