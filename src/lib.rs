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
        Self::get_by_revision(code, CURRENT_REVISION)
    }

    pub fn get_by_revision(code: &str, revision: &str) -> Option<Self> {
        DIVISIONS.get_entry(revision).and_then(|(rev, data)| {
            data.get_entry(code).map(|(key, name)| {
                Division {
                    code: key,
                    name: name,
                    revision: rev,
                }
            })
        })
    }

    pub fn revisions() -> Vec<&'static str> {
        DIVISIONS.keys().cloned().collect()
    }

    pub fn province(&self) -> Self {
        let code = format!("{}0000", &self.code[..2]);
        Self::get_by_revision(&code, self.revision).unwrap()
    }

    pub fn is_province(&self) -> bool {
        *self == self.province()
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
        assert!(division.is_province());
    }
}
