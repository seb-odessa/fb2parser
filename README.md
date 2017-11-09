# fb2parser 

This library was developed for parsing [FictionBook2 format](http://fictionbook.org/) into the Rust's structs.
Used in main project [fb2lib](https://github.com/seb-odessa/fb2lib)

```
#[cfg(test)]
mod tests {
    use super::*;
    use xmltree::Element;
    use util::from;
    const TEST_DATA: &'static str = "<root><book-title>value</book-title></root>";

    #[test]
    fn from_trait_impl() {
        let root = Element::parse(TEST_DATA.as_bytes()).unwrap();
        assert_eq!(
            BookTitle { text: "value".to_owned() },
            from(&root, "book-title").unwrap()
        );
    }
}
```


