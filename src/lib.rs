#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

mod helper;
pub mod fb;

pub fn create(xml: String) -> Result<fb::FictionBook, fb::Error> {
    return try_fast(xml).
        or_else(try_escaped).
        or_else(try_fix_lang).
        or_else(try_fix_title_info_double_last_name).
        or_else(try_fix_doc_info_double_nickname).
        or_else(done);
}

fn try_fast(xml: String) -> Result<fb::FictionBook, String> {
    // Skip heading bytes until '<' will found
    let clean:String = xml.chars().skip_while(|c| *c != '<').collect();

    match fb::deserialize(clean.as_bytes()) {
        Ok(result) => Ok(result),
        Err(_) => Err(clean),
    }
}

fn try_escaped(xml: String) -> Result<fb::FictionBook, String> {
    let fixed_xml = helper::escape(&xml);
    match fb::deserialize(fixed_xml.as_bytes()) {
        Ok(result) => Ok(result),
        Err(_) => Err(fixed_xml),
    }
}

fn try_fix_lang(xml: String) -> Result<fb::FictionBook, String> {
    let fixed_xml = helper::deduplicate_tags(&xml, "title-info", "lang");
    match fb::deserialize(fixed_xml.as_bytes()) {
        Ok(result) => Ok(result),
        Err(_) => Err(fixed_xml),
    }
}

fn try_fix_title_info_double_last_name(xml: String) -> Result<fb::FictionBook, String> {
    let fixed_xml = helper::deduplicate_tags(&xml, "title-info", "last-name");
    match fb::deserialize(fixed_xml.as_bytes()) {
        Ok(result) => Ok(result),
        Err(_) => Err(fixed_xml),
    }
}

fn try_fix_doc_info_double_nickname(xml: String) -> Result<fb::FictionBook, String> {
    let fixed_xml = helper::deduplicate_tags(&xml, "document-info", "nickname");
    match fb::deserialize(fixed_xml.as_bytes()) {
        Ok(result) => Ok(result),
        Err(_) => Err(fixed_xml),
    }
}

fn done(xml: String) -> Result<fb::FictionBook, fb::Error> {
    match fb::deserialize(xml.as_bytes()) {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}


#[cfg(test)]
mod tests {
    use super::create;

    use std::fs::File;
    use fb::*;
    use std::io::{Read, Result};

    fn load_xml(file_name: &str) -> Result<String> {

        let mut file = File::open(file_name)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }

    #[test]
    fn parse_double_last_name_tag() {
        let xml = load_xml("test_data/double_last_name.xml");
        assert!(xml.is_ok());
        let obj = self::create(xml.unwrap());
        assert!(obj.is_ok());

    }

    #[test]
    fn parse_description_xml() {
        let xml = load_xml("test_data/description.xml");
        assert!(xml.is_ok());
        let obj = self::create(xml.unwrap());
        assert!(obj.is_ok());
        let fb: FictionBook = obj.unwrap();
        assert_eq!(
            fb,
            FictionBook {
                description: Description {
                    title_info: TitleInfo {
                        genre: vec![
                            "sf".to_owned(),
                            "sf_history".to_owned()
                            ],
                        author: vec![
                            Author {
                                first_name: "Константин".to_owned(),
                                middle_name: "Георгиевич".to_owned(),
                                last_name: "Калбанов".to_owned(),
                                nick_name: "".to_owned(),
                                home_page: "http://samlib.ru/k/kalbazow_k_g/".to_owned(),
                                email: "mahoni928@yandex.ru".to_owned(),
                            },
                        ],
                        book_title: "Робинзоны".to_owned(),
                        date: "".to_owned(),
                        lang: "ru".to_owned(),
                        src_lang: "".to_owned(),
                        translator: vec![],
                        sequence: vec![
                            Sequence {
                                name: "Робинзоны".to_owned(),
                                number: "1".to_owned(),
                                lang: "".to_owned(),
                            },
                        ],
                    },
                },
            }
        );
    }
}