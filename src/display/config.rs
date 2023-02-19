use std::borrow::Cow;


#[derive(Clone)]
pub struct LogicFmtConfig {
    pub(super) or: Cow<'static, str>,
    pub(super) and: Cow<'static, str>,
    pub(super) pre_value: Cow<'static, str>,
    pub(super) post_value: Cow<'static, str>,
    pub(super) show_value: fn(u32)->bool,
    pub(super) top_level_sep: Cow<'static, str>,
    pub(super) brackets: (Cow<'static, str>, Cow<'static, str>),
}

impl Default for LogicFmtConfig {
    fn default() -> Self {
        Self::basic()
    }
}
impl LogicFmtConfig {
    pub const fn basic() -> Self {
        Self{and: Cow::Borrowed(" AND "), or: Cow::Borrowed(" OR "), pre_value: Cow::Borrowed(": "), post_value: Cow::Borrowed(""), show_value: |_| true, top_level_sep: Cow::Borrowed(""), brackets: (Cow::Borrowed("("), Cow::Borrowed(")"))}
    } 
    pub fn new(and: String, or: String) -> Self {
        Self{and: Cow::Owned(and), or: Cow::Owned(or), ..Default::default()}
    }
    pub fn value_prefix(mut self, pre: String) -> Self {
        self.pre_value = Cow::Owned(pre);
        self
    }
    pub fn value_postfix(mut self, pre: String) -> Self {
        self.post_value = Cow::Owned(pre);
        self
    }
    pub fn show_value_if(mut self, confition: fn(u32)->bool) -> Self {
        self.show_value = confition;
        self
    }
    pub fn separate_top_level_with(mut self, sep: String) -> Self {
        self.top_level_sep = Cow::Owned(sep);
        self
    }
    pub fn no_brackets(mut self) -> Self {
        self.brackets = (Cow::Borrowed(""), Cow::Borrowed(""));
        self
    }
}