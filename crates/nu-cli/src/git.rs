use crate::prelude::*;
use git2::{Repository, RepositoryOpenFlags};
use std::ffi::OsString;

pub fn current_branch() -> Option<String> {
    if let Ok(config) = crate::data::config::config(Tag::unknown()) {
        let use_starship = match config.get("use_starship") {
            Some(b) => match b.as_bool() {
                Ok(b) => b,
                _ => false,
            },
            _ => false,
        };

        if !use_starship {
            let v: Vec<OsString> = vec![];
            match Repository::open_ext(".", RepositoryOpenFlags::empty(), v) {
                Ok(repo) => {
                    let r = repo.head();
                    match r {
                        Ok(r) => match r.shorthand() {
                            Some(s) => Some(s.to_string()),
                            None => None,
                        },
                        _ => None,
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}
