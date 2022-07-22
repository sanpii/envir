pub(crate) struct Symbol(&'static str);

pub(crate) const DEFAULT: Symbol = Symbol("default");
pub(crate) const ENVIR: Symbol = Symbol("envir");
pub(crate) const EXPORT_WITH: Symbol = Symbol("export_with");
pub(crate) const INTERNAL: Symbol = Symbol("internal");
pub(crate) const LOAD_WITH: Symbol = Symbol("load_with");
pub(crate) const NAME: Symbol = Symbol("name");
pub(crate) const NOPREFIX: Symbol = Symbol("noprefix");
pub(crate) const NESTED: Symbol = Symbol("nested");
pub(crate) const PREFIX: Symbol = Symbol("prefix");

impl PartialEq<Symbol> for syn::Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a syn::Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
