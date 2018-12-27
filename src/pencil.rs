struct Pencil {
    max_durability: u32,
    durability: u32,
    eraser_durability: u32,
    length: u32,
    page: String
}

impl Pencil {
    pub fn new(max_durability: u32, length: u32, eraser_durability: u32) -> Pencil {
        return Pencil { max_durability, durability: max_durability, length, eraser_durability, page: "".to_string() }
    }

    pub fn write(&mut self, string: String) {
        let whitespace = [' ', '\n', '\t'];
        for character in string.chars() {
            let is_whitespace = whitespace.contains(&character);
            let out_of_lead = self.durability == 0;

            if is_whitespace {
                self.page.push(character)
            } else if out_of_lead {
                self.page.push(' ')
            } else {
                self.page.push(character);
                self.durability -= 1
            }
        }
    }

    pub fn sharpen(&mut self) {
        self.durability = self.max_durability
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn given_string_with_no_spaces_when_writes_then_lowers_durability() {
        let mut pencil = Pencil::new(4, 4, 4);
        pencil.write("test".to_string());

        assert_eq!(pencil.durability, 0)
    }

    #[test]
    fn given_string_with_whitespace_when_writes_then_lowers_durability_for_only_text() {
        let mut pencil = Pencil::new(8, 4, 4);
        pencil.write("test test\n\t\t".to_string());

        assert_eq!(pencil.durability, 0)
    }

    #[test]
    fn given_string_with_writes_then_writes_to_page() {
        let string = "test test\n\t\t";
        let mut pencil = Pencil::new(8, 4, 4);

        pencil.write(string.to_string());

        assert_eq!(pencil.page, string.to_string())
    }

    #[test]
    fn given_subsequent_writes_then_appends_string() {
        let mut pencil = Pencil::new(16, 4, 4);

        pencil.write("test test\n\t\t".to_string());
        pencil.write("test test".to_string());

        assert_eq!(pencil.page, "test test\n\t\ttest test".to_string())
    }

    #[test]
    fn given_durability_runs_out_writes_spaces_instead_of_text() {
        let mut pencil = Pencil::new(19, 4, 4);

        pencil.write("test: ".to_string());
        pencil.write("gonna test this kata".to_string());

        assert_eq!(pencil.page, "test: gonna test this k   ");
        assert_eq!(pencil.durability, 0);
    }


    #[test]
    fn given_durability_runs_out_then_can_sharpen_to_max() {
        let mut pencil = Pencil::new(7, 4, 4);

        pencil.write("unicorn".to_string());
        pencil.sharpen();

        assert_eq!(pencil.durability, 7);
    }
}