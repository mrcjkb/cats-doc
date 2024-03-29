use chumsky::{prelude::choice, select, Parser};

use crate::{
    lexer::{Member, TagType, Ty},
    parser::{impl_parse, Prefix},
    Accept, Visitor,
};

#[derive(Debug, Clone)]
pub enum AliasKind {
    Type(Ty),
    Enum(Vec<(Member, Option<String>)>),
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub name: String,
    pub desc: Vec<String>,
    pub kind: AliasKind,
    pub prefix: Prefix,
}

impl_parse!(Alias, {
    select! {
        TagType::Comment(x) => x,
    }
    .repeated()
    .then(choice((
        select! {
            TagType::Alias(name, Some(ty)) => (name, AliasKind::Type(ty))
        },
        select! { TagType::Alias(name, ..) => name }.then(
            select! {
                TagType::Variant(ty, desc) => (ty, desc)
            }
            .repeated()
            .map(AliasKind::Enum),
        ),
    )))
    .map(|(desc, (name, kind))| Self {
        name,
        desc,
        kind,
        prefix: Prefix::default(),
    })
});

impl<T: Visitor> Accept<T> for Alias {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.alias(self, s)
    }
}
