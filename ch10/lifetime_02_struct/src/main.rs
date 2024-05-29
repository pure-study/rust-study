fn main() {}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level() {
        let ie = ImportantExcerpt {
            part: "whatever",
        };

        assert_eq!(3, ie.level());
    }

    #[test]
    fn test_announce_and_return_part() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };

        assert_eq!("Call me Ishmael", i.part);
        assert_eq!("Call me Ishmael", i.announce_and_return_part("announcement test"));
    }
}