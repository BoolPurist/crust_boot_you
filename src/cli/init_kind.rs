use std::{fmt::Display, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InitKind {
    OnlyEmpty,
    NoNameConflicts,
    Override,
    Purge,
}

impl Default for InitKind {
    fn default() -> Self {
        Self::OnlyEmpty
    }
}
use crate::constants::{
    INIT_KIND_NO_NAME_CONFLICT, INIT_KIND_ONLY_EMPTY, INIT_KIND_OVERRIDE, INIT_KIND_PURGE,
};

pub const fn valid_values_init_kind_msg() -> &'static str {
    const_format::formatcp!(
        "Valid values are ({}, {}, {}, {})",
        INIT_KIND_ONLY_EMPTY,
        INIT_KIND_NO_NAME_CONFLICT,
        INIT_KIND_OVERRIDE,
        INIT_KIND_PURGE
    )
}

impl Display for InitKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::OnlyEmpty => INIT_KIND_ONLY_EMPTY,
            Self::NoNameConflicts => INIT_KIND_NO_NAME_CONFLICT,
            Self::Override => INIT_KIND_OVERRIDE,
            Self::Purge => INIT_KIND_PURGE,
        };
        write!(f, "{}", output)
    }
}
impl FromStr for InitKind {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            INIT_KIND_ONLY_EMPTY => Ok(Self::OnlyEmpty),
            INIT_KIND_NO_NAME_CONFLICT => Ok(Self::NoNameConflicts),
            INIT_KIND_OVERRIDE => Ok(Self::Override),
            INIT_KIND_PURGE => Ok(Self::Purge),
            _ => Err(format!(
                "{} is not one of the valid words. {}",
                s,
                valid_values_init_kind_msg()
            )),
        }
    }
}
