#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

mod solutions {
    pub(crate) struct Solution;
    include!("../solutions/3.longest-substring-without-repeating-characters.rs");
    include!("../solutions/5.longest-palindromic-substring.rs");
    include!("../solutions/51.n-queens.rs");
}

#[cfg(test)]
mod tests;
