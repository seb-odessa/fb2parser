/*********************************************************************************************
Элемент <FictionBook>
Описание

Корневой элемент документа.
Версия FB

2.0 и выше
Поддерживается

По своей природе поддерживается любой программой, претендующей на минимальную поддержку FB.
Атрибуты

Атрибутов нет.

Поскольку является корневым элементом, то здесь задаются пространства имен (что выглядит похоже на аттрибуты).
Подчиненные элементы

Должен содержать в перечисленном порядке:

    <stylesheet> - 0..n (любое число, опционально);
    <description> - 1 (один, обязателен);
    <body> - 1..n (любое число, один обязaтелен);
    <binary> - 0..n (любое число, опционально).

Подчинен

Поскольку является корневым элементом, то никому не подчинен.
*********************************************************************************************/

use std::fmt;
use xmltree::{Element, ParseError};
use Description;
use util::from;

#[derive(Debug, PartialEq)]
pub struct FictionBook {
    pub description: Option<Description>,
}
impl FictionBook {
    pub fn new(xml: &[u8]) -> Result<Self, ParseError> {
        match Element::parse(xml) {
            Ok(node) => Ok(FictionBook { description: from(&node, "description") }),
            Err(e) => Err(e),
        }
    }

    #[allow(dead_code)]
    pub fn get_book_title(&self) -> Option<String> {
        if let Some(ref description) = self.description {
            if let Some(ref title_info) = description.title_info {
                if let Some(ref book_title) = title_info.book_title {
                    return Some(book_title.text.clone());
                }
            }
        }
        None
    }
}
impl fmt::Display for FictionBook {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if let Some(ref description) = self.description {
            write!(fmt, "{}", description)
        } else {
            Ok(())
        }
    }
}


#[cfg(test)]
mod tests {
    use tests::XML;
    use FictionBook;

    #[test]
    fn new() {
        let fb = FictionBook::new(XML.as_bytes()).unwrap();
        assert!(fb.description.is_some());
    }

    #[test]
    fn get_book_title() {
        let fb = FictionBook::new(XML.as_bytes()).unwrap();
        assert_eq!(
            "Тень его мыслей",
            &fb.get_book_title().unwrap()
        );
    }
}
