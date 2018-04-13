extern crate phf;

mod data;

use data::DIVISIONS;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Division<'a> {
    pub code: &'a str,
    pub name: &'a str,
    pub revision: &'a str,
}

impl<'a> Division<'a> {

    pub fn revisions() -> Vec<&'static str> {
        DIVISIONS.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
