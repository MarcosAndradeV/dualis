/*
   Binding  =
   Type     ::
   Function () {

   }
*/

use std::collections::{HashMap, HashSet};

pub struct TypeCheckState {
    map_of_unions: HashMap<Ty, HashSet<Ty>>,
    map_of_lists: HashMap<Ty, Ty>,
    map_of_records: HashMap<Ty, HashMap<&'static str, Ty>>,
}

#[allow(non_camel_case_types)]
#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Ty {
    int,
    float,
    string,
    Union(u64),
    List(u64),
    Record(u64),
}

#[cfg(test)]
mod tests {
    use std::hash::{DefaultHasher, Hash, Hasher};

    use super::*;

    #[test]
    fn test_name() {
        let mut h = DefaultHasher::new();
        Ty::int.hash(&mut h);
        Ty::float.hash(&mut h);
        let number_ty = Ty::Union(h.finish());

        let mut h = DefaultHasher::new();
        Ty::int.hash(&mut h);
        Ty::float.hash(&mut h);
        assert_eq!(number_ty, Ty::Union(h.finish()));
    }
}
