use crate::{Recipe, RecipeError, NodeRecipe, typed::{ParamLogic, ItemLogic}, logic::{LogicType, LogicChain}, key::KeyMeaning};

impl<'a> TryFrom<AnyRecipe<&'a str>> for Recipe<&'a str, &'a str> {
    type Error = RecipeError;

    fn try_from(recipe: AnyRecipe<&'a str>) -> Result<Self, Self::Error> {
        match recipe {
            AnyRecipe::Numeric(recipe) => Err(format!("Unexpected numeric recipe: {:?}", recipe)),
            AnyRecipe::Textual(recipe) => Ok(recipe),
        }
    }
}

impl<'a> TryFrom<AnyRecipe<&'a str>> for Recipe<&'a str, u32> {
    type Error = RecipeError;

    fn try_from(recipe: AnyRecipe<&'a str>) -> Result<Self, Self::Error> {
        match recipe {
            AnyRecipe::Numeric(recipe) => Ok(recipe),
            AnyRecipe::Textual(recipe) => Err(format!("Unexpected textual recipe: {:?}", recipe)),
        }    
    }
}

impl<'a, K: Clone> TryFrom<AnyRecipe<&'a str>> for NodeRecipe<String, K>
    where Recipe<&'a str, K>: TryFrom<AnyRecipe<&'a str>, Error = RecipeError>
{
    type Error = RecipeError;

    fn try_from(recipe: AnyRecipe<&'a str>) -> Result<Self, Self::Error> {
        Ok((Recipe::try_from(recipe)?).into())
    }
}

impl<'a> TryFrom<AnyRecipe<&'a str>> for NodeRecipe<String, String>
    where Recipe<&'a str, &'a str>: TryFrom<AnyRecipe<&'a str>, Error = RecipeError>
{
    type Error = RecipeError;

    fn try_from(recipe: AnyRecipe<&'a str>) -> Result<Self, Self::Error> {
        Ok((Recipe::try_from(recipe)?).into())
    }
}

#[derive(Debug)]
pub enum AnyRecipe<S> {
    Textual(Recipe<S, S>),
    Numeric(Recipe<S, u32>),
}

#[derive(PartialEq, Debug)]
pub struct GenericRecipe<S, L> {
    pub(crate) name: S,
    pub(crate) description: Option<S>,
    pub(crate) params_to_see: Option<L>,
    pub(crate) params_to_craft: Option<L>,
    pub(crate) ingredients: L,
    pub(crate) tools: Option<L>,
    pub(crate) output: L,
    pub(crate) side_effect: SideEffect<S>,
}

impl<S, L> GenericRecipe<S, L> {
    pub fn internal_name(&self) -> &S {
        &self.name
    }
    pub fn description(&self) -> Option<&S> {
        self.description.as_ref()
    }
    pub fn params_to_see(&self) -> Option<ParamLogic<L>> {
        ParamLogic::map(&self.params_to_see)
    }
    pub fn params_to_craft(&self) -> Option<ParamLogic<L>> {
        ParamLogic::map(&self.params_to_craft)
    }
    pub fn ingredients(&self) -> ItemLogic<L> {
        ItemLogic::new(&self.ingredients)
    }
    pub fn tools(&self) -> Option<ItemLogic<L>> {
        ItemLogic::map(&self.tools)
    }
    pub fn output(&self) -> ItemLogic<L> {
        ItemLogic::new(&self.output)
    }
}

impl<K, S: Clone, L: LogicType<Key = K>> GenericRecipe<S, L> {
    pub fn with_keys<K2, E, F: Fn(&K, KeyMeaning)->Result<K2, E>>(&self, f: F) -> Result<GenericRecipe<S, L::Gats<K2>>, E> {
        let convert = |logic: &L, meaning: KeyMeaning| logic.with_keys(|key| f(key, meaning));
        let convert_opt = |logic: &Option<L>, meaning: KeyMeaning| logic.as_ref().map(|logic| convert(logic, meaning)).transpose();
        Ok(GenericRecipe{
            name: self.name.clone(),
            description: self.description.clone(),
            params_to_see: convert_opt(&self.params_to_see, KeyMeaning::Param)?,
            params_to_craft: convert_opt(&self.params_to_craft, KeyMeaning::Param)?,
            ingredients: convert(&self.ingredients, KeyMeaning::Item)?,
            tools: convert_opt(&self.tools, KeyMeaning::Item)?,
            output: convert(&self.output, KeyMeaning::Item)?,
            side_effect: self.side_effect.clone(),
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
pub(crate) enum SideEffect<S> {
    Script { module: S, function: S },
    Experience(u32),
}
impl<S: Default> SideEffect<S> {
    #[allow(dead_code)]
    pub(crate) fn truncated(&self) -> Self {
        match self {
            SideEffect::Script { .. } => SideEffect::Script { module: Default::default(), function: Default::default() },
            SideEffect::Experience(_) => SideEffect::Experience(Default::default()),
        }
    }
}
impl<S: Clone> SideEffect<S> {
    fn convert<S2: From<S>>(&self) -> SideEffect<S2> {
        match self {
            SideEffect::Script { module, function } => SideEffect::Script { module: module.clone().into(), function: function.clone().into() },
            SideEffect::Experience(exp) => SideEffect::Experience(*exp),
        }
    }
}

impl<'a, K: Clone, K2: From<K>> From<Recipe<&'a str, K>> for NodeRecipe<String, K2> {
    fn from(value: Recipe<&'a str, K>) -> Self {
        NodeRecipe {
            name: value.name.into(),
            description: value.description.map(Into::into),
            params_to_see: value.params_to_see.map(LogicChain::logic_nodes),
            params_to_craft: value.params_to_craft.map(LogicChain::logic_nodes),
            ingredients: value.ingredients.logic_nodes(),
            tools: value.tools.map(LogicChain::logic_nodes),
            output: value.output.logic_nodes(),
            side_effect: value.side_effect.convert(),
        }
    }
}

impl<'a, K: Clone, K2: From<K>> From<Recipe<&'a str, K>> for NodeRecipe<&'a str, K2> {
    fn from(value: Recipe<&'a str, K>) -> Self {
        NodeRecipe {
            name: value.name,
            description: value.description,
            params_to_see: value.params_to_see.map(LogicChain::logic_nodes),
            params_to_craft: value.params_to_craft.map(LogicChain::logic_nodes),
            ingredients: value.ingredients.logic_nodes(),
            tools: value.tools.map(LogicChain::logic_nodes),
            output: value.output.logic_nodes(),
            side_effect: value.side_effect.convert(),
        }
    }
}