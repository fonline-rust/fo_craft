use crate::{recipe::AnyRecipe, RecipeError, book::RecipeBook};

mod lexer;

impl<'a, R: TryFrom<AnyRecipe<&'a str>, Error=RecipeError>> RecipeBook<R> {
    pub fn try_from_iter(iter: impl Iterator<Item = (u32, &'a str)>) -> Result<Self, RecipeError> {
        let mut book = Self{recipes: Default::default()};
        for (index, str) in iter {
            let recipe = lexer::any_recipe::<nom_prelude::nom::error::VerboseError<&'a str>>(str).map_err(|err| format!("Recipe #{} has err: {:?}", index, err))?.1;
            let recipe = recipe.try_into()?;
            book.recipes.insert(index, recipe);
        }
        Ok(book)
    }
}
