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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
    pub fn get_book_genres(&self) -> Vec<String> {
        let mut result = Vec::new();
        if let Some(ref description) = self.description {
            if let Some(ref title_info) = description.title_info {                
                for value in &title_info.genres {
                    result.push(value.get());
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
                    let first_name = author.first_name.clone().unwrap_or_default().get();
                    let middle_name = author.middle_name.clone().unwrap_or_default().get();
                    let last_name = author.last_name.clone().unwrap_or_default().get();
                    let nickname = author.nickname.clone().unwrap_or_default().get();
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
                if let Some(ref value) = title_info.book_title {
                    result = value.get();
                }
            }
        }
        return result;
    }
    #[allow(dead_code)]
    pub fn get_book_date(&self) -> String {
        let mut result = String::new();
        if let Some(ref description) = self.description {
            if let Some(ref title_info) = description.title_info {
                if let Some(ref value) = title_info.date {
                    result = value.value.clone();
                    if result.is_empty() {
                        result = value.text.clone();
                    }
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
                if let Some(ref value) = title_info.lang {
                    result = value.get();
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
                if let Some(ref value) = title_info.src_lang {
                    result = value.get();
                }
            }
        }
        return result;
    }

    #[allow(dead_code)]
    pub fn get_book_translators(&self) -> Vec<(String,String,String,String)> {
        let mut result = Vec::new();
        if let Some(ref description) = self.description {            
            if let Some(ref title_info) = description.title_info {
                for author in &title_info.translators {
                    let first_name = author.first_name.clone().unwrap_or_default().get();
                    let middle_name = author.middle_name.clone().unwrap_or_default().get();
                    let last_name = author.last_name.clone().unwrap_or_default().get();
                    let nickname = author.nickname.clone().unwrap_or_default().get();
                    result.push((first_name, middle_name, last_name, nickname));
                }                
            }
        }
        return result;
    }    

    #[allow(dead_code)]
    pub fn get_book_sequences(&self) -> Vec<(String, u32)> {
        let mut result = Vec::new();
        if let Some(ref description) = self.description {
            if let Some(ref title_info) = description.title_info {
                for value in &title_info.sequences {
                    result.push((value.get_name(), value.get_number()));
                }
            }
        }
        return result;
    }   

    #[allow(dead_code)]
    pub fn get_doc_authors(&self) -> Vec<(String,String,String,String)> {
        let mut result = Vec::new();
        if let Some(ref description) = self.description {            
            if let Some(ref document_info) = description.document_info {
                for author in &document_info.authors {
                    let first_name = author.first_name.clone().unwrap_or_default().get();
                    let middle_name = author.middle_name.clone().unwrap_or_default().get();
                    let last_name = author.last_name.clone().unwrap_or_default().get();
                    let nickname = author.nickname.clone().unwrap_or_default().get();
                    result.push((first_name, middle_name, last_name, nickname));
                }                
            }
        }
        return result;
    }

    #[allow(dead_code)]
    pub fn get_doc_program_used(&self) -> String {
        let mut result = String::new();
        if let Some(ref description) = self.description {
            if let Some(ref document_info) = description.document_info {
                if let Some(ref value) = document_info.program_used {
                    result = value.text.clone();
                }
            }
        }
        return result;
    }

    #[allow(dead_code)]
    pub fn get_doc_date(&self) -> String {
        let mut result = String::new();
        if let Some(ref description) = self.description {
            if let Some(ref document_info) = description.document_info {
                if let Some(ref value) = document_info.date {
                    result = value.value.clone();
                    if result.is_empty() {
                        result = value.text.clone();
                    }
                }
            }
        }
        return result;
    }

    #[allow(dead_code)]
    pub fn get_doc_publishers(&self) -> Vec<String> {
        let mut result = Vec::new();
        if let Some(ref description) = self.description {
            if let Some(ref document_info) = description.document_info {                
                for value in &document_info.publishers {
                    result.push(value.get_value());
                }
            }
        }
        return result;
    }   

    #[allow(dead_code)]
    pub fn get_publish_book_name(&self) -> String {
        let mut result = String::new();
        if let Some(ref description) = self.description {
            if let Some(ref publish_info) = description.publish_info {
                if let Some(ref value) = publish_info.book_name {
                    result = value.text.clone();
                }
            }
        }
        return result;
    }

    #[allow(dead_code)]
    pub fn get_publish_publishers(&self) -> Vec<String> {
        let mut result = Vec::new();
        if let Some(ref description) = self.description {
            if let Some(ref publish_info) = description.publish_info {
                if let Some(ref value) = publish_info.publisher {
                    result.push(value.get_value());
                }
            }
        }
        return result;
    }

    #[allow(dead_code)]
    pub fn get_publish_city(&self) -> String {
        let mut result = String::new();
        if let Some(ref description) = self.description {
            if let Some(ref publish_info) = description.publish_info {
                if let Some(ref value) = publish_info.city {
                    result = value.text.clone();
                }
            }
        }
        return result;
    }

    #[allow(dead_code)]
    pub fn get_publish_year(&self) -> String {
        let mut result = String::new();
        if let Some(ref description) = self.description {
            if let Some(ref publish_info) = description.publish_info {
                if let Some(ref value) = publish_info.year {
                    result = value.text.clone();
                }
            }
        }
        return result;
    }

    #[allow(dead_code)]
    pub fn get_publish_isbn(&self) -> String {
        let mut result = String::new();
        if let Some(ref description) = self.description {
            if let Some(ref publish_info) = description.publish_info {
                if let Some(ref value) = publish_info.isbn {
                    result = value.text.clone();
                }
            }
        }
        return result;
    }    

    #[allow(dead_code)]
    pub fn get_publish_sequences(&self) -> Vec<(String, u32)> {
        let mut result = Vec::new();
        if let Some(ref description) = self.description {
            if let Some(ref publish_info) = description.publish_info {
                for value in &publish_info.sequences {
                    result.push((value.name.clone(), value.number));
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
    use bincode::{serialize, deserialize};

    #[test]
    fn new() {
        let fb = FictionBook::new(XML.as_bytes()).unwrap();
        assert!(fb.description.is_some());
    }

    #[test]
    fn test_title_info() {
        let fb = FictionBook::new(XML.as_bytes()).unwrap();
        assert_eq!(
            ["sf_space","sf_epic"].into_iter().map(|&s| String::from(s)).collect::<Vec<_>>(),
            fb.get_book_genres()
        );
        assert_eq!(
            [("Иван", "Иванович", "Иванов", "ivan"), ("Пётр", "Петрович", "Петров", "piter")]
                .iter().map(|&(fname, mname, lname, nick)| (String::from(fname), String::from(mname), String::from(lname), String::from(nick)))
                .collect::<Vec<(_,_,_,_)>>(),
            fb.get_book_authors()
        );
        assert_eq!("Название книги",fb.get_book_title().as_str());
        assert_eq!("1999",fb.get_book_date());
        assert_eq!("ru", fb.get_book_lang().as_str());
        assert_eq!("ua", fb.get_book_src_lang().as_str());
        assert_eq!(
            vec![("Сидор", "Сидорович", "Сидоров", "sidorov")]
                .iter().map(|&(fname, mname, lname, nick)| (String::from(fname), String::from(mname), String::from(lname), String::from(nick)))
                .collect::<Vec<(_,_,_,_)>>(),
            fb.get_book_translators()
        );
        assert_eq!(vec![(String::from("Вавилон"), 5)],fb.get_book_sequences());
    }


    #[test]
    fn test_document_info() {
        let fb = FictionBook::new(XML.as_bytes()).unwrap();
        assert_eq!(
            vec![("Николай", "", "Никулин", "")]
                .iter().map(|&(fname, mname, lname, nick)| (String::from(fname), String::from(mname), String::from(lname), String::from(nick)))
                .collect::<Vec<(_,_,_,_)>>(),
            fb.get_doc_authors()
        );
        assert_eq!("hand made",fb.get_doc_program_used().as_str());
        assert_eq!("2008-03-06",fb.get_doc_date());
        assert_eq!(
            ["Домашняя Библиотека", "Сам себе Гуттенберг"].into_iter().map(|&s| String::from(s)).collect::<Vec<_>>(),
            fb.get_doc_publishers()
        );
    }

    #[test]
    fn test_publish_info() {
        let fb = FictionBook::new(XML.as_bytes()).unwrap();
        assert_eq!("Фатаморгана",fb.get_publish_book_name());
        assert_eq!(
            ["Сам себе Гуттенберг"].into_iter().map(|&s| String::from(s)).collect::<Vec<_>>(),
            fb.get_publish_publishers()
        );
        assert_eq!("Москва",fb.get_publish_city());
        assert_eq!("2018",fb.get_publish_year());
        assert_eq!("ISBN 1-58182-008-9",fb.get_publish_isbn());
        assert_eq!(vec![(String::from("Серия Вавилон"), 5)],fb.get_publish_sequences());    
    }

    #[test]
    fn test_serialize() {
        let fb = FictionBook::new(XML.as_bytes()).unwrap();
        let encoded: Vec<u8> = serialize(&fb).unwrap();
        assert_eq!(812, encoded.len());
        let restored: FictionBook = deserialize(&encoded[..]).unwrap();
        assert_eq!(fb, restored);
    }

}
