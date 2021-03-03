extern crate phf;

mod data;

use data::DIVISIONS;

const CURRENT_REVISION: &str = "202011";

/// The administrative division
#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Division {
    /// The six-digit number of the specific administrative division
    pub code: &'static str,
    /// The Chinese name of the specific administrative division
    pub name: &'static str,
    /// The revision year (month)
    pub revision: &'static str,
}

impl Division {
    /// Return the division of the given code
    pub fn get(code: &str) -> Option<Self> {
        Self::get_by_revision(code, CURRENT_REVISION)
    }

    /// Return the division of the given code of the given revision
    pub fn get_by_revision(code: &str, revision: &str) -> Option<Self> {
        DIVISIONS.get_entry(revision).and_then(|(rev, data)| {
            data.get_entry(code).map(|(key, name)| Division {
                code: key,
                name: name,
                revision: rev,
            })
        })
    }

    /// Searches administrative division by its code in all revision
    pub fn search(code: &str) -> Option<Self> {
        fn parse_rev_year(rev: &str) -> u32 {
            let parts: Vec<&str> = rev.split('-').collect();
            if parts.len() == 2 {
                parts[1].parse().unwrap()
            } else {
                rev.parse().unwrap()
            }
        }

        let mut revisions = Self::revisions();
        revisions.sort_unstable_by(|a, b| {
            let year_a = parse_rev_year(a);
            let year_b = parse_rev_year(b);
            year_b.cmp(&year_a)
        });
        for rev in revisions {
            let division = Self::get_by_revision(code, rev);
            if division.is_some() {
                return division;
            }
        }
        None
    }

    /// List all revisions supported by GB2260
    pub fn revisions() -> Vec<&'static str> {
        DIVISIONS.keys().cloned().collect()
    }

    /// Return province level division of current division
    pub fn province(&self) -> Self {
        let code = format!("{}0000", &self.code[..2]);
        Self::get_by_revision(&code, self.revision).unwrap()
    }

    pub fn is_province(&self) -> bool {
        *self == self.province()
    }

    /// Return prefecture level division of current division
    pub fn prefecture(&self) -> Option<Self> {
        if self.is_province() {
            return None;
        }
        let code = format!("{}00", &self.code[..4]);
        Self::get_by_revision(&code, self.revision)
    }

    pub fn is_prefecture(&self) -> bool {
        if let Some(pref) = self.prefecture() {
            pref == *self
        } else {
            false
        }
    }

    /// Return county level division of current division
    pub fn county(&self) -> Option<&Division> {
        if self.is_province() || self.is_prefecture() {
            return None;
        }
        Some(self)
    }

    pub fn is_county(&self) -> bool {
        self.county().is_some()
    }

    pub fn stack(&self) -> Vec<Self> {
        let mut res = Vec::with_capacity(3);
        res.push(self.province());
        if self.is_prefecture() || self.is_county() {
            res.push(self.prefecture().unwrap());
        }
        if self.is_county() {
            res.push(self.clone());
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Division;

    #[test]
    fn test_division() {
        let division = Division::get("440000").unwrap();
        assert_eq!(division.code, "440000");
        assert_eq!(division.name, "广东省");
        assert_eq!(division.revision, "202011");
        assert!(division.is_province());
        assert!(!division.is_prefecture());
        assert!(!division.is_county());
        assert_eq!(division.stack().len(), 1);

        let division = Division::get("440100").unwrap();
        assert_eq!(division.code, "440100");
        assert_eq!(division.name, "广州市");
        assert_eq!(division.revision, "202011");
        assert!(!division.is_province());
        assert!(division.is_prefecture());
        assert!(!division.is_county());
        assert_eq!(division.stack().len(), 2);

        let division = Division::get("440115").unwrap();
        assert_eq!(division.code, "440115");
        assert_eq!(division.name, "南沙区");
        assert_eq!(division.revision, "202011");
        assert!(!division.is_province());
        assert!(!division.is_prefecture());
        assert!(division.is_county());
        assert_eq!(division.stack().len(), 3);

        let division_search = Division::search("110000").unwrap();
        let division_get = Division::get("110000").unwrap();
        assert_eq!(division_search, division_get);
    }
}
