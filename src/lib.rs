#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

pub type XmlParseError = serde_xml_rs::Error;
pub mod fb;

#[cfg(test)]
mod tests {

    use std::io::Result;
    use std::io::prelude::*;
    use std::fs::File;
    use fb::*;
    use std::io::Read;
    use serde_xml_rs::deserialize;
    

    fn load_xml(file_name: &str) -> Result<String> {

        let mut file = File::open(file_name)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }

    #[test]
    fn parse_description_xml() {
        let xml = load_xml("test_data/description.xml");
        assert!(xml.is_ok());
        let obj = FictionBook::new(&xml.unwrap());
        assert!(obj.is_ok());
        let fb: FictionBook = obj.unwrap();
        assert_eq!(
            fb,
            FictionBook {
                description: Description {
                    title_info: TitleInfo {
                        author: vec![
                            Author {
                                first_name: "Константин".to_string(),
                                middle_name: "Георгиевич".to_string(),
                                last_name: "Калбанов".to_string(),
                                nick_name: "".to_string(),
                                home_page: "http://samlib.ru/k/kalbazow_k_g/".to_string(),
                                email: "mahoni928@yandex.ru".to_string(),
                            },
                        ],
                        book_title: "Робинзоны".to_string(),
                        lang: "ru".to_string(),
                        src_lang: "".to_string(),
                        translator: vec![],
                        sequence: vec![
                            Sequence {
                                name: "Робинзоны".to_string(),
                                number: "1".to_string(),
                                lang: "".to_string(),
                            },
                        ],
                    },
                },
            }
        );
    }
}