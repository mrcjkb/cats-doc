use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TagType {
    /// ```lua
    /// ---@toc <name>
    /// ```
    Toc(String),
    /// ```lua
    /// ---@mod <name> [desc]
    /// ```
    Module(String, Option<String>),
    /// ```lua
    /// ---@divider <char>
    /// ```
    Divider(char),
    /// ```lua
    /// function one.two() end
    /// one.two = function() end
    /// ```
    Func {
        prefix: Option<String>,
        name: String,
        kind: Kind,
    },
    /// ```lua
    /// one = 1
    /// one.two = 12
    /// ```
    Expr {
        prefix: Option<String>,
        name: String,
        kind: Kind,
    },
    /// ```lua
    /// ---@export <module>
    /// or
    /// return <module>\eof
    /// ```
    Export(String),
    /// ```lua
    /// ---@brief [[
    /// ```
    BriefStart,
    /// ```lua
    /// ---@brief ]]
    /// ```
    BriefEnd,
    /// ```lua
    /// ---@param <name[?]> <type[|type...]> [description]
    /// ```
    Param(TypeVal, Option<String>),
    /// ```lua
    /// ---@return <type> [<name> [comment] | [name] #<comment>]
    /// ```
    Return {
        ty: Ty,
        name: Option<String>,
        desc: Option<String>,
    },
    /// ```lua
    /// ---@class <name>
    /// ```
    Class(String),
    /// ```lua
    /// ---@field [public|private|protected] <name> <type> [description]
    /// ```
    Field {
        scope: Scope,
        name: String,
        ty: Ty,
        desc: Option<String>,
    },
    /// ```lua
    /// -- Simple Alias
    /// ---@alias <name> <type>
    ///
    /// -- Enum alias
    /// ---@alias <name>
    /// ```
    Alias(String, Option<Ty>),
    /// ```lua
    /// ---| '<value>' [# description]
    /// ```
    Variant(String, Option<String>),
    /// ```lua
    /// ---@type <type> [desc]
    /// ```
    Type(Ty, Option<String>),
    /// ```lua
    /// ---@tag <name>
    /// ```
    Tag(String),
    /// ```lua
    /// ---@see <name>
    /// ```
    See(String),
    /// ```lua
    /// ---@usage `<code>`
    /// ```
    Usage(String),
    /// ```lua
    /// ---@usage [[
    /// ```
    UsageStart,
    /// ```lua
    /// ---@usage ]]
    /// ```
    UsageEnd,
    /// ```lua
    /// ---TEXT
    /// ```
    Comment(String),
    /// Text nodes which are not needed
    Skip,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Kind {
    Dot,
    Colon,
    Local,
}

impl Kind {
    pub fn as_char(&self) -> char {
        match self {
            Self::Dot => '.',
            Self::Colon => ':',
            Self::Local => '#',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Scope {
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeVal {
    Req(String, Ty),
    Opt(String, Ty),
}

impl Display for TypeVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Req(n, t) => write!(f, "{n}:{t}"),
            Self::Opt(n, t) => write!(f, "{n}?:{t}"),
        }
    }
}

// Source: https://github.com/sumneko/lua-language-server/wiki/Annotations#documenting-types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Nil,
    Any,
    Unknown,
    Boolean,
    String,
    Number,
    Integer,
    Function,
    Thread,
    Userdata,
    Lightuserdata,
    Ref(String),
    Array(Box<Ty>),
    Table(Option<(Box<Ty>, Box<Ty>)>),
    Fun(Vec<TypeVal>, Option<Vec<Ty>>),
    Dict(Vec<TypeVal>),
    Union(Box<Ty>, Box<Ty>),
}

impl Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn list_like(args: &[TypeVal]) -> String {
            args.iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(",")
        }

        match self {
            Self::Nil => f.write_str("nil"),
            Self::Any => f.write_str("any"),
            Self::Unknown => f.write_str("unknown"),
            Self::Boolean => f.write_str("boolean"),
            Self::String => f.write_str("string"),
            Self::Number => f.write_str("number"),
            Self::Integer => f.write_str("integer"),
            Self::Function => f.write_str("function"),
            Self::Thread => f.write_str("thread"),
            Self::Userdata => f.write_str("userdata"),
            Self::Lightuserdata => f.write_str("lightuserdata"),
            Self::Ref(id) => f.write_str(id),
            Self::Array(ty) => write!(f, "{ty}[]"),
            Self::Table(kv) => match kv {
                Some((k, v)) => write!(f, "table<{k},{v}>"),
                None => f.write_str("table"),
            },
            Self::Fun(args, ret) => {
                f.write_str("fun(")?;
                f.write_str(&list_like(args))?;
                f.write_str(")")?;
                if let Some(ret) = ret {
                    f.write_str(":")?;
                    f.write_str(
                        &ret.iter()
                            .map(|r| r.to_string())
                            .collect::<Vec<String>>()
                            .join(","),
                    )?;
                }
                Ok(())
            }
            Self::Dict(kv) => {
                f.write_str("{")?;
                f.write_str(&list_like(kv))?;
                f.write_str("}")
            }
            Self::Union(rhs, lhs) => write!(f, "{rhs}|{lhs}"),
        }
    }
}