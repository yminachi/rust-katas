struct Pencil {
    max_durability: u32,
    durability: u32,
    eraser_durability: u32,
    page: String
}

impl Pencil {
    pub fn new(max_durability: u32, eraser_durability: u32) -> Pencil {
        return Pencil { max_durability, durability: max_durability, eraser_durability, page: "".to_string() }
    }

    pub fn write(&mut self, string: String) {
        self.durability -= string.chars().filter(|c| ![' ', '\n', '\t'].contains(c)).count() as u32;
        self.page = string
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn given_string_with_no_spaces_when_writes_then_lowers_durability() {
        let mut pencil = Pencil::new(4, 4);
        pencil.write("test".to_string());

        assert_eq!(pencil.durability, 0)
    }

    #[test]
    fn given_string_with_whitespace_when_writes_then_lowers_durability_for_only_text() {
        let mut pencil = Pencil::new(8, 4);
        pencil.write("test test\n\t\t".to_string());

        assert_eq!(pencil.durability, 0)
    }

    #[test]
    fn given_string_with_writes_then_writes_to_page() {
        let string = "test test\n\t\t";
        let mut pencil = Pencil::new(8, 4);

        pencil.write(string.to_string());

        assert_eq!(pencil.page, string.to_string())
    }
}