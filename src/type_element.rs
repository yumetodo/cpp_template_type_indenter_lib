// Forked from https://github.com/SimonSapin/rust-forest/blob/master/rctree/lib.rs
// because the crate is not available anymore on crates.io. MIT License Simon Sapin
// pub use rctree::NodeRef as NodeRef;
pub use tmp_type_element::TmpTypeElement;
#[derive(Debug)]
#[allow(dead_code)]
struct TypeElement<'a> {
    before: Option<&'a str>,
    after: Option<&'a str>,
    value: Option<&'a str>,
}


impl<'a> PartialEq<TypeElement<'a>> for TypeElement<'a> {
    fn eq<'b>(&self, other: &TypeElement<'b>) -> bool {
        if self.value.is_some() {
            self.value == other.value
        } else {
            self.before.is_some() && self.before == other.before && self.after == other.after
        }
    }
}

#[allow(dead_code)]
impl<'a> TypeElement<'a> {
    pub fn new_value(value: &'a str) -> TypeElement<'a> {
        TypeElement { before: None, after: None, value: Some(value) }
    }
    pub fn new(before: &'a str, after: &'a str) -> TypeElement<'a> {
        TypeElement { before: Some(before), after: Some(after), value: None }
    }
}
#[cfg(test)]
mod tests {
    use type_element::*;
    #[test]
    fn new_value() {
        let s1 = "std::vector";
        let e1_1 = TypeElement { before: None, after: None, value: Some(&s1) };
        let e1_2 = TypeElement::new_value(&s1);
        assert_eq!(e1_1, e1_2);
    }
    #[test]
    fn new() {
        let s2 = "std::is_arithmetic<T>::value";
        let s2_splitted: Vec<&str> = s2.split(|c: char| c == '<' || c == '>').collect();
        let e2_1 = TypeElement { before: Some(s2_splitted[0]), after: Some(s2_splitted[2]), value: None };
        let e2_2 = TypeElement::new(s2_splitted[0], s2_splitted[2]);
        assert_eq!(e2_1, e2_2);
    }
}