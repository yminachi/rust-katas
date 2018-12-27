use std::cmp::min;

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
            } else if character.is_uppercase() {
                self.page.push(character);
                self.durability -= 2
            } else {
                self.page.push(character);
                self.durability -= 1
            }
        }
    }

    pub fn sharpen(&mut self) {
        if self.length != 0 {
            self.durability = self.max_durability;
            self.length -= 1
        }
    }

    pub fn erase(&mut self, word: String) {
        let last_match = self.page.match_indices(&word).last();

        if let Some(matched) = last_match {
            let mut new_string = "".to_string();

            let start_index = matched.0 + word.len() - min(word.len(), self.eraser_durability as usize);
            let end_index = matched.0 + word.len();

            for (i, c) in self.page.chars().enumerate() {
                if i >= start_index && i < end_index {
                    new_string.push(' ');
                    self.eraser_durability -= 1;
                } else {
                    new_string.push(c)
                }
            }

            self.page = new_string
        }
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
    fn given_string_with_capital_letters_when_writes_then_lowers_durability_by_two() {
        let mut pencil = Pencil::new(8, 4, 4);
        pencil.write("TeSt".to_string());

        assert_eq!(pencil.durability, 2)
    }

    #[test]
    fn given_string_with_whitespace_when_writes_then_lowers_durability_for_only_text() {
        let mut pencil = Pencil::new(8, 4, 4);
        pencil.write("test test\n\t\t".to_string());

        assert_eq!(pencil.durability, 0)
    }

    #[test]
    fn given_string_then_writes_to_page() {
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
        let mut pencil = Pencil::new(7, 1, 4);

        pencil.write("unicorn".to_string());
        pencil.sharpen();

        assert_eq!(pencil.durability, 7);
    }

    #[test]
    fn when_sharpened_then_reduces_length() {
        let mut pencil = Pencil::new(7, 4, 4);

        pencil.sharpen();

        assert_eq!(pencil.length, 3);
    }

    #[test]
    fn given_pencil_with_no_length_when_sharpened_then_does_not_restore_durability() {
        let mut pencil = Pencil::new(7, 0, 4);

        pencil.write("unicorn".to_string());
        pencil.sharpen();

        assert_eq!(pencil.durability, 0);
    }

    #[test]
    fn given_existing_text_when_erasing_replaces_last_instance_with_spaces_word() {
        let mut pencil = Pencil::new(100, 7, 5);

        pencil.write("one two three four three two one".to_string());
        pencil.erase("three".to_string());

        assert_eq!(pencil.page, "one two three four       two one".to_string());
    }

    #[test]
    fn when_erasing_depletes_eraser_durability() {
        let mut pencil = Pencil::new(100, 7, 5);

        pencil.write("one two three four three two one".to_string());
        pencil.erase("three".to_string());

        assert_eq!(pencil.eraser_durability, 0);
    }

    #[test]
    fn given_not_enough_durability_when_erasing_starts_from_end_of_word() {
        let mut pencil = Pencil::new(100, 7, 3);

        pencil.write("one two three four three two one".to_string());
        pencil.erase("three".to_string());

        assert_eq!(pencil.page, "one two three four th    two one".to_string());
    }
}