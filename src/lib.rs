#[cfg(feature = "display")]
pub mod display;
#[cfg(feature = "parse")]
mod parse;
pub mod logic;
pub mod book;
pub mod recipe;
mod key;
pub mod typed;

type RecipeError = String;

pub type Recipe<S, K> = recipe::GenericRecipe<S, logic::LogicChain<K>>;
pub type NodeRecipe<S, K> = recipe::GenericRecipe<S, logic::LogicNode<K>>;
pub type ReadableOwnedNodeRecipe = recipe::GenericRecipe<String, logic::LogicNode<String>>;
pub type UserFriendlyRecipeBook = book::RecipeBook<ReadableOwnedNodeRecipe>;
pub type OwnedNodeRecipe = recipe::GenericRecipe<String, logic::LogicNode<u32>>;
pub type OwnedRecipeBook = book::RecipeBook<OwnedNodeRecipe>;

#[cfg(test)]
mod tests {
    use crate::{UserFriendlyRecipeBook, book::RecipeBook, Recipe, key::KeyMeaning, NodeRecipe};

    fn _readable_local_recipes<'a, I: Iterator<Item = (u32, &'a str)>>(lines: I) -> Result<UserFriendlyRecipeBook, String> {
        let lst = fo_lst_format::parse_dir("../../FO4RP/data").map_err(|err| format!("Can't parse LST files: {err}"))?;
        let book = RecipeBook::<Recipe<&str, u32>>::try_from_iter(lines).map_err(|err| format!("Can't parse craft book: {err}"))?;
        let lst_mapper = |key: &u32, meaning: KeyMeaning| lst.index_to_string_in_file(*key, meaning.lst_file_name()).map(String::from).ok_or_else(|| format!("Key {key} not found in dictionary"));
        let readable_book = book.map_recipes(|recipe| {
            let node_recipe: NodeRecipe<String, u32> = recipe.into();
            node_recipe.with_keys(lst_mapper)
            //Ok::<_, String>(recipe.with_keys(lst_mapper)?.into())
        })?;
        Ok(readable_book)
    }
}
