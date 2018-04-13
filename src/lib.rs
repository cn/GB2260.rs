extern crate phf;

mod data;

use data::DIVISIONS;

const CURRENT_REVISION: &str = "201607";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Division {
    pub code: &'static str,
    pub name: &'static str,
    pub revision: &'static str,
}

impl Division {

    pub fn get(code: &str) -> Option<Self> {
        let current_data = DIVISIONS[CURRENT_REVISION];
        current_data.get_entry(code).map(|(key, name)| {
            Division {
                code: key,
                name: name,
                revision: CURRENT_REVISION,
            }
        })
    }

    pub fn revisions() -> Vec<&'static str> {
        DIVISIONS.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Division;

    #[test]
    fn test_get() {
        let division = Division::get("110000").unwrap();
        assert_eq!(division.code, "110000");
        assert_eq!(division.name, "北京市");
        assert_eq!(division.revision, "201607");
    }
}
