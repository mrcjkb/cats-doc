use chumsky::{select, Parser};

use crate::{
    lexer::{Scope, TagType, Ty},
    parser::{impl_parse, Prefix, See},
};

#[derive(Debug, Clone)]
pub struct Field {
    pub scope: Scope,
    pub name: String,
    pub ty: Ty,
    pub desc: Vec<String>,
}

impl_parse!(Field, {
    select! {
        TagType::Comment(x) => x,
    }
    .repeated()
    .then(select! {
        TagType::Field { scope, name, ty, desc } => (scope, name, ty, desc)
    })
    .map(|(header, (scope, name, ty, desc))| {
        let desc = match desc {
            Some(d) => vec![d],
            None => header,
        };

        Self {
            scope,
            name,
            ty,
            desc,
        }
    })
});

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub desc: Vec<String>,
    pub fields: Vec<Field>,
    pub see: See,
    pub prefix: Prefix,
}

impl_parse!(Class, {
    select! { TagType::Comment(c) => c }
        .repeated()
        .then(select! { TagType::Class(name) => name })
        .then(Field::parse().repeated())
        .then(See::parse())
        .map(|(((desc, name), fields), see)| Self {
            name,
            desc,
            fields,
            see,
            prefix: Prefix::default(),
        })
});

impl Class {
    pub fn rename_tag(&mut self, tag: String) {
        self.prefix.right = Some(tag);
    }
}
