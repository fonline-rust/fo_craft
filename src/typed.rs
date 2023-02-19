use crate::logic::LogicType;

#[derive(Default)]
pub struct ParamKey;

#[derive(Default)]
pub struct ItemKey;

pub struct TypedLogic<'a, L, T> {
    logic: &'a L,
    _ty: T,
}
impl<'a, L, T> TypedLogic<'a, L, T> {
    pub fn logic(&self) -> &L {
        self.logic
    }
}
impl<'a, L, T: Default> TypedLogic<'a, L, T> {
    pub(crate) fn new(logic: &'a L) -> Self {
        Self{logic, _ty: Default::default()}
    }
    pub(crate) fn map(logic: &'a Option<L>) -> Option<Self> {
        Some(Self::new(logic.as_ref()?))
    }
}
pub type ParamLogic<'a, L> = TypedLogic<'a, L, ParamKey>;
pub type ItemLogic<'a, L> = TypedLogic<'a, L, ItemKey>;

impl<'a, L: LogicType, T> TypedLogic<'a, L, T> {
    pub fn map_keys<'o, M: TypedKeyMapper<T, L::Key>>(&self, key_mapper: &'o M) -> Result<L::Gats<M::Output<'o>>, M::Error> {
        self.logic.with_keys(|key| key_mapper.key_map(key))
    }
}

pub trait TypedKeyMapper<T, K> {
    type Output<'o> where Self: 'o;
    type Error;
    fn key_map<'o>(&'o self, from: &K) -> Result<Self::Output<'o>, Self::Error>;
}
