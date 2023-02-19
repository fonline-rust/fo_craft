use std::{collections::BTreeMap, ops::Deref};

#[derive(Debug)]
pub struct RecipeBook<R> {
    pub(crate) recipes: BTreeMap<u32, R>,
}

impl<R> Default for RecipeBook<R> {
    fn default() -> Self {
        Self { recipes: Default::default() }
    }
}

impl<R> Deref for RecipeBook<R> {
    type Target = BTreeMap<u32, R>;

    fn deref(&self) -> &Self::Target {
        &self.recipes
    }
}

/*
impl<S, K> RecipeBook<S, K> {
    fn from_iter_inner<'a>(iter: impl Iterator<Item = (u32, &'a str)>, and_then: impl Fn(AnyRecipe<S>) -> Result<Recipe<S, K>, RecipeError>) -> Result<Self, RecipeError> {
        let mut book = Self{recipes: Default::default()};
        for (index, str) in iter {
            let recipe = lexer::any_recipe(str).map_err(|err| format!("Recipe #{} has err: {:?}", index, err))?.1;
            let recipe = and_then(recipe)?;
            book.recipes.insert(index, recipe);
        }
        Ok(book)
    }
}
*/

impl<R> RecipeBook<R> {
    pub fn map_recipes<R2, E, F: Fn(R)->Result<R2, E>>(self, map: F) -> Result<RecipeBook<R2>, E> {
        let res: Result<_, E> = self.recipes.into_iter().map(|(num, recipe)| Ok((num, map(recipe)?))).collect();
        Ok(RecipeBook { recipes: res? })
    }
}
/*
impl<K, S: Clone, L: LogicType<Key = K>> RecipeBook<GenericRecipe<S, L>> {
    pub fn with_keys<K2, E, F: Fn(&K, KeyMeaning)->Result<K2, E>>(&self, f: F) -> Result<RecipeBook<GenericRecipe<S, L::Gats<K2>>>, E> {
        let res: Result<_, E> =  self.recipes.iter().map(|(num, recipe)| Ok((*num, recipe.with_keys(&f)?))).collect();
        Ok(RecipeBook { recipes: res? })
    }
}
*/