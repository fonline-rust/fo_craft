pub(crate) trait Key<'a>: Sized {
    fn key_from(str: &'a str) -> Option<Self>;
}

impl<'a> Key<'a> for u32 {
    fn key_from(str: &'a str) -> Option<Self> {
        str.parse().ok()
    }
}
impl<'a> Key<'a> for &'a str {
    fn key_from(str: &'a str) -> Option<Self>{
        Some(str)
    }
}
impl<'a> Key<'a> for String {
    fn key_from(str: &str) -> Option<Self> {
        Some(str.to_owned())
    }
}

#[derive(Clone, Copy)]
pub enum KeyMeaning {
    Param,
    Item,
}
impl KeyMeaning {
    pub fn lst_file_name(self) -> &'static str {
        match self {
            KeyMeaning::Item => "ItemNames",
            KeyMeaning::Param => "ParamNames",
        }
    }
}
