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
    pub fn get_genres(&self) -> Vec<String> {
        let mut result = Vec::new();
        if let Some(ref description) = self.description {
            if let Some(ref title_info) = description.title_info {                
                for genre in &title_info.genres {
                    result.push(genre.text.clone());
                }
            }
        }
        return result;
    }    

    #[allow(dead_code)]
    pub fn get_book_authors(&self) -> Vec<(String,String,String,String)> {
        let mut result = Vec::new();
        if let Some(ref description) = self.description {            
            if let Some(ref title_info) = description.title_info {                
                for author in &title_info.authors {
                    let first_name = author.first_name.clone().unwrap_or_default().text;
                    let middle_name = author.middle_name.clone().unwrap_or_default().text;
                    let last_name = author.last_name.clone().unwrap_or_default().text;
                    let nickname = author.nickname.clone().unwrap_or_default().text;
                    result.push((first_name, middle_name, last_name, nickname));
                }                
            }
        }
        return result;
    }    

    #[allow(dead_code)]
    pub fn get_book_title(&self) -> String {
        let mut result = String::new();
        if let Some(ref description) = self.description {
            if let Some(ref title_info) = description.title_info {
                if let Some(ref book_title) = title_info.book_title {
                    result = book_title.text.clone();
                }
            }
        }
        return result;
    }

    #[allow(dead_code)]
    pub fn get_book_lang(&self) -> String {
        let mut result = String::new();
        if let Some(ref description) = self.description {
            if let Some(ref title_info) = description.title_info {
                if let Some(ref book_lang) = title_info.lang {
                    result = book_lang.text.clone();
                }
            }
        }
        return result;
    }

    #[allow(dead_code)]
    pub fn get_book_src_lang(&self) -> String {
        let mut result = String::new();
        if let Some(ref description) = self.description {
            if let Some(ref title_info) = description.title_info {
                if let Some(ref book_src_lang) = title_info.src_lang {
                    result = book_src_lang.text.clone();
                }
            }
        }
        return result;
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

    fn convert<S: Into<String>>(input: Vec<S>) -> Vec<String>
    {
        input.into_iter().map(|s| s.into()).collect::<Vec<_>>()
    }

    #[test]
    fn new() {
        let fb = FictionBook::new(XML.as_bytes()).unwrap();
        assert!(fb.description.is_some());
    }

    #[test]
    fn test_get_book_title() {
        let fb = FictionBook::new(XML.as_bytes()).unwrap();
        assert_eq!(
            "Название книги",
            fb.get_book_title().as_str()
        );
    }
    #[test]
    fn test_get_genres() {
        let fb = FictionBook::new(XML.as_bytes()).unwrap();
        assert_eq!(
            convert(vec!["sf_space", "sf_epic"]),
            fb.get_genres()
        );
    }

    #[test]
    fn test_get_book_authors() {
        let fb = FictionBook::new(XML.as_bytes()).unwrap();
        assert_eq!(
            vec![
                (String::from("Иван"), String::from("Иванович"), String::from("Иванов"), String::from("ivan")), 
                (String::from("Пётр"), String::from("Петрович"), String::from("Петров"), String::from("piter"))],
            fb.get_book_authors()
        );
    }
    
    #[test]
    fn test_get_book_langs() {
        let fb = FictionBook::new(XML.as_bytes()).unwrap();
        assert_eq!("ru", fb.get_book_lang().as_str());
        assert_eq!("ua", fb.get_book_src_lang().as_str());
    }
    
}
