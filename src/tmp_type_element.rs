#[derive(Debug)]
#[allow(dead_code)]
pub struct TmpTypeElement<'a> {
    pub t: &'a str,
    pub level: usize,
}

impl<'a> PartialEq<TmpTypeElement<'a>> for TmpTypeElement<'a> {
    fn eq<'b>(&self, other: &TmpTypeElement<'b>) -> bool {
        return self.t.eq(other.t) && self.level == other.level;
    }
}

#[allow(dead_code)]
impl<'a> TmpTypeElement<'a> {
    pub fn new(t: &'a str, level: usize) -> TmpTypeElement<'a> {
        TmpTypeElement { t: t, level: level }
    }
    pub fn from_str(s: &'a str) -> Vec<TmpTypeElement<'a>> {
        let mut re: Vec<TmpTypeElement<'a>> = Vec::new();
        for (i, s) in s.split('<').enumerate() {
            re.push(TmpTypeElement::new(s, i));
        }
        return re;
    }
}


#[cfg(test)]
mod tests {
    use tmp_type_element::TmpTypeElement;
    #[test]
    fn new() {
        let s1 = "std::vector";
        let t1 = TmpTypeElement{ t: &s1, level: 0};
        let t2 = TmpTypeElement::new(&s1, 0);
        assert_eq!(t1, t2);
    }
    #[test]
    fn from_str() {
        let s = "std::vector<std::basic_string<char16_t>>";
        assert_eq!(
            vec![TmpTypeElement::new("std::vector", 0), TmpTypeElement::new("std::basic_string", 1), TmpTypeElement::new("char16_t>>", 2)],
            TmpTypeElement::from_str(s)
        );
    }
    #[test]
    fn eq() {
        let s1 = "std::vector";
        let s2 = "std::basic_string";
        let t1 = TmpTypeElement::new(&s1, 0);
        let t2 = TmpTypeElement::new(&s1, 0);
        assert_eq!(t1, t2);
        let t3 = TmpTypeElement::new(&s1, 1);
        assert!(t1.ne(&t3));
        let t4 = TmpTypeElement::new(&s2, 0);
        assert!(t1.ne(&t4));
    }
}
