//! A simple library for parsing an FB2 data into a rust structure
//!
//! FictionBook 2.0 Specification: http://gribuser.ru/xml/fictionbook/index.html.en
//! Shema Comments: http://gribuser.ru/xml/fictionbook/shema_comments.html
//! Root Node Wiki: http://www.fictionbook.org/index.php/%D0%AD%D0%BB%D0%B5%D0%BC%D0%B5%D0%BD%D1%82_FictionBook
//!

#[cfg(bench)]
extern crate test;

#[macro_use]
extern crate serde_derive;
extern crate xmltree;

use xmltree::{Element, ParseError};
use std::convert::TryFrom;

trait Fb2Node {
    const NAME: &'static str;
    fn ok(element: &xmltree::Element) -> bool {
        element.name.as_str().eq_ignore_ascii_case(Self::NAME)
    }

    fn ignore(element: &xmltree::Element) -> bool {
        element.children.iter().fold(false, |ignore, node|{
            if !ignore {
                if let Some(comment) = node.as_comment() {
                    return comment.eq_ignore_ascii_case("IGNORE");
                }
            }
            return false;
        })
    }

    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            Some(Self::default())
        } else {
            None
        }
    }

    fn get_text(element: &xmltree::Element) -> String {
        element.children.iter()
            .map(|node| node.as_text())
            .filter(|text|text.is_some())
            .map(|text|
                text.unwrap_or_default()
                .trim()
                .replace("&amp;","&")
                .replace("&apos;","'")
                .replace("&lt;","<")
                .replace("&gt;",">")
                .replace("&quot;","\"")
            )
            .nth(0)
            .unwrap_or_default()
    }

    fn get_capitalized_text(element: &xmltree::Element) -> String {
        let text = Self::get_text(element).to_lowercase();
        let mut result = Vec::with_capacity(text.len());
        let mut capitalize = true;
        for ch in text.chars() {
            if capitalize {
                for c in ch.to_uppercase() {
                    result.push(c);
                }
            } else {
                result.push(ch);
            }
            capitalize = !ch.is_alphabetic();
        }
        result.into_iter().collect::<String>()
    }
}

/// query items of type T from the xmltree::Element
fn query_subitems<T: Fb2Node+Default>(element: &xmltree::Element) -> Vec<T>
{
    element.children.iter().filter_map(
        |node| {
            if let Some(element) = node.as_element() {
                if T::ok(element) {
                    if T::ignore(element) {
                        return Some(T::default());
                    } else {
                        return T::new(element);
                    }
                }
            }
            return None;
        }
    ).collect::<Vec<T>>()
}

fn query_one<T: Fb2Node+Default+Clone>(element: &xmltree::Element) -> Option<T> {

    let objs = query_subitems::<T>(element);
    if 1 == objs.len() {
        Some(objs[0].clone())
    } else {
        None
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct FictionBook {
    pub stylesheets: Vec<Stylesheet>,
    pub description: Description,
    pub bodies: Vec<Body>,
    pub binaries: Vec<Binary>,
}
impl TryFrom<&[u8]> for FictionBook {
    type Error = ParseError;
    fn try_from(xml: &[u8]) -> Result<Self, Self::Error> {
        Element::parse(xml).and_then(|element|FictionBook::new(&element).ok_or(ParseError::CannotParse))
    }
}
impl Fb2Node for FictionBook {
    const NAME: &'static str = "fictionbook";
    fn new(element: &xmltree::Element) -> Option<Self>
    {
        if let Some(description) = query_one(element) {
            return Some(Self{
                stylesheets: query_subitems(element),
                description: description,
                bodies: query_subitems(element),
                binaries: query_subitems(element),
            });
        }
        return None;
    }
}
impl FictionBook {
    pub fn get_title_ref(&self) -> &TitleInfo {
        &self.description.title_info
    }
    pub fn get_publish(&self) -> PublishInfo {
        self.description.publish_info.clone().unwrap_or_default()
    }
    pub fn get_genres(&self) -> Vec<Genre> {
        self.get_title_ref().genres.clone()
    }
    pub fn get_authors(&self) -> Vec<Author> {
        self.get_title_ref().authors.clone()
    }
    pub fn get_title(&self) -> String {
        self.get_title_ref().book_title.clone().unwrap_or_default().text
    }
    pub fn get_date(&self) -> Option<Date> {
        self.get_title_ref().date.clone()
    }
    pub fn get_lang(&self) -> Option<Lang> {
        self.get_title_ref().lang.clone()
    }
    pub fn get_translators(&self) -> Vec<Translator> {
        self.get_title_ref().translators.clone()
    }
    pub fn get_sequences(&self) -> Vec<Sequence> {
        self.get_title_ref().sequences.clone()
    }
    pub fn get_book_name(&self) -> Option<String> {
        self.get_publish().book_name.map(|a| a.text)
    }
    pub fn get_book_publisher(&self) ->Option<String> {
        self.get_publish().publisher.map(|a| a.text)
    }
    pub fn get_book_year(&self) ->Option<String> {
        self.get_publish().year.map(|a| a.text)
    }
    pub fn get_book_isbn(&self) ->Option<String> {
        self.get_publish().isbn.map(|a| a.text)
    }
    pub fn get_book_sequences(&self) -> Vec<Sequence> {
        self.get_publish().sequences
    }
}


#[cfg(test)]
mod fictionbook {
    use super::*;

    #[test]
    fn get_text() {
        {
            let xml = r#"<first-name>Ру &amp; беж</first-name>"#;
            let node = Element::parse(xml.as_bytes()).map_or(None, |e| FirstName::new(&e)).unwrap_or_default();
            assert_eq!("Ру & Беж", node.text);
        }
        {
            let xml = r#"<first-name>&quot;Рубеж&quot;</first-name>"#;
            let node = Element::parse(xml.as_bytes()).map_or(None, |e| FirstName::new(&e)).unwrap_or_default();
            assert_eq!("\"Рубеж\"", node.text);
        }
        {
            let xml = r#"<first-name>&lt;Рубеж&gt;</first-name>"#;
            let node = Element::parse(xml.as_bytes()).map_or(None, |e| FirstName::new(&e)).unwrap_or_default();
            assert_eq!("<Рубеж>", node.text);
        }
    }

    #[test]
    fn get_capitalized_text() {
        {
            let xml = r#"<first-name>Рубеж</first-name>"#;
            let node = Element::parse(xml.as_bytes()).map_or(None, |e| FirstName::new(&e)).unwrap_or_default();
            assert_eq!("Рубеж", node.text);
        }
        {
            let xml = r#"<first-name>рубеж</first-name>"#;
            let node = Element::parse(xml.as_bytes()).map_or(None, |e| FirstName::new(&e)).unwrap_or_default();
            assert_eq!("Рубеж", node.text);
        }
        {
            let xml = r#"<first-name>руб еж</first-name>"#;
            let node = Element::parse(xml.as_bytes()).map_or(None, |e| FirstName::new(&e)).unwrap_or_default();
            assert_eq!("Руб Еж", node.text);
        }
        {
            let xml = r#"<first-name>аль-каддафи</first-name>"#;
            let node = Element::parse(xml.as_bytes()).map_or(None, |e| FirstName::new(&e)).unwrap_or_default();
            assert_eq!("Аль-Каддафи", node.text);
        }
        {
            let xml = r#"<first-name>АЛЬ-КАДДАФИ</first-name>"#;
            let node = Element::parse(xml.as_bytes()).map_or(None, |e| FirstName::new(&e)).unwrap_or_default();
            assert_eq!("Аль-Каддафи", node.text);
        }

    }

    #[test]
    fn is_ok_element() {
        let xml = r##"
        <FictionBook xmlns = "http://www.gribuser.ru/xml/fictionbook/2.0" xmlns:xlink = "http://www.w3.org/1999/xlink">
        </FictionBook>
        "##;

        let element = Element::parse(xml.as_bytes());
        assert!(element.is_ok());
        assert!(FictionBook::ok(&element.unwrap()));
    }

    #[test]
    fn parse_all_synthetic() {
        let xml = r##"
        <FictionBook xmlns = "http://www.gribuser.ru/xml/fictionbook/2.0" xmlns:xlink = "http://www.w3.org/1999/xlink">
        <stylesheet> <!--IGNORE--> </stylesheet>
        <stylesheet> <!--IGNORE--> </stylesheet>
        <description> <!--IGNORE--> </description>
        <body> <!--IGNORE--> </body>
        <body> <!--IGNORE--> </body>
        <body> <!--IGNORE--> </body>
        <binary> <!--IGNORE--> </binary>
        <binary> <!--IGNORE--> </binary>
        <binary> <!--IGNORE--> </binary>
        <binary> <!--IGNORE--> </binary>
        </FictionBook>
        "##;
        let obj = FictionBook::try_from(xml.as_bytes());
        assert!(obj.is_ok());
        let payload = obj.unwrap();
        assert_eq!(2, payload.stylesheets.len());
        assert_eq!(3, payload.bodies.len());
        assert_eq!(4, payload.binaries.len());
    }

    #[test]
    fn parse_only_description() {
        let xml = r##"
        <FictionBook xmlns = "http://www.gribuser.ru/xml/fictionbook/2.0" xmlns:xlink = "http://www.w3.org/1999/xlink">
        <description> <!--IGNORE--> </description>
        </FictionBook>
        "##;
        assert!(FictionBook::try_from(xml.as_bytes()).is_ok());
    }

    #[test]
    fn parse_empty_element() {
        let xml = r##"
        <FictionBook xmlns = "http://www.gribuser.ru/xml/fictionbook/2.0" xmlns:xlink = "http://www.w3.org/1999/xlink">
        </FictionBook>
        "##;
        assert!(FictionBook::try_from(xml.as_bytes()).is_err());
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Description {
    pub title_info: TitleInfo,
    pub src_title_info: Option<SrcTitleInfo>,
    pub document_info: Option<DocumentInfo>,
    pub publish_info: Option<PublishInfo>,
    pub custom_info: Vec<CustomInfo>,
    pub output: Vec<Output>,
}
impl Fb2Node for Description {
    const NAME: &'static str = "description";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            if let Some(title_info) = query_one(element) {
                return Some(Description{
                    title_info: title_info,
                    src_title_info: query_one(element),
                    document_info: query_one(element),
                    publish_info: query_one(element),
                    custom_info: query_subitems(element),
                    output: query_subitems(element)
                });
            }
        }
        return None;
    }
}

#[cfg(test)]
mod description {
    use super::*;

    #[test]
    fn parse_all_synthetic() {
        let xml = r##"
        <FictionBook xmlns="http://www.gribuser.ru/xml/fictionbook/2.0" xmlns:xlink="http://www.w3.org/1999/xlink">
        <description>
            <title-info>    <!--IGNORE--></title-info>
            <src-title-info><!--IGNORE--></src-title-info>
            <document-info> <!--IGNORE--></document-info>
            <publish-info>  <!--IGNORE--></publish-info>
            <custom-info>   <!--IGNORE--></custom-info>
            <output>        <!--IGNORE--></output>
        </description>
        <body><!--IGNORE--></body>
        </FictionBook>
        "##;

        assert!(FictionBook::try_from(xml.as_bytes()).is_ok());
    }

    #[test]
    fn parse_wo_title_info() {
        let xml = r##"
        <FictionBook xmlns="http://www.gribuser.ru/xml/fictionbook/2.0" xmlns:xlink="http://www.w3.org/1999/xlink">
        <description>
            <src-title-info><!--IGNORE--></src-title-info>
            <document-info> <!--IGNORE--></document-info>
            <publish-info>  <!--IGNORE--></publish-info>
            <custom-info>   <!--IGNORE--></custom-info>
            <output>        <!--IGNORE--></output>
        </description>
        <body><!--IGNORE--></body>
        </FictionBook>
        "##;
        assert!(FictionBook::try_from(xml.as_bytes()).is_err());
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Body; // Ignored payload
impl Fb2Node for Body {
    const NAME: &'static str = "body";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Stylesheet; // Not Implemented
impl Fb2Node for Stylesheet {
    const NAME: &'static str = "stylesheet";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Binary; // Not Implemented
impl Fb2Node for Binary {
    const NAME: &'static str = "binary";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct TitleInfo{
    pub genres: Vec<Genre>,
    pub authors: Vec<Author>,
    pub book_title: Option<BookTitle>,
    pub annotations: Vec<Annotation>,
    pub keywords: Vec<Keywords>,
    pub date: Option<Date>,
    pub coverpage: Option<Coverpage>,
    pub lang: Option<Lang>,
    pub src_lang: Option<SrcLang>,
    pub translators: Vec<Translator>,
    pub sequences: Vec<Sequence>,
}
impl Fb2Node for TitleInfo {
    const NAME: &'static str = "title-info";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self {
                genres: query_subitems(element),
                authors: query_subitems(element),
                book_title: query_one(element),
                annotations: query_subitems(element),
                keywords: query_subitems(element),
                date: query_one(element),
                coverpage: query_one(element),
                lang: query_one(element),
                src_lang: query_one(element),
                translators: query_subitems(element),
                sequences: query_subitems(element),
            })
        }
        return None;
    }
}

#[cfg(test)]
mod title_info {
    use super::*;

    #[test]
    fn parse_all() {
        let xml = r#"
            <title-info>
                <genre match="42">western</genre>
                <genre match="20">detective</genre>
                <author>
                    <first-name>robert</first-name>
                    <middle-name>ANSON</middle-name>
                    <last-name>Heinlein</last-name>
                </author>
                <author>
                    <first-name>Ray</first-name>
                    <middle-name>Douglas</middle-name>
                    <last-name>Bradbury</last-name>
                </author>
                "<book-title>Fahrenheit 451</book-title>"
            </title-info>
            "#;
        let obj = Element::parse(xml.as_bytes()).map_or(None, |element| TitleInfo::new(&element));
        assert!(obj.is_some());
        let payload = obj.unwrap();
        assert_eq!(2, payload.genres.len());
        assert_eq!("western", payload.genres[0].text);
        assert_eq!(42, payload.genres[0].matching);
        assert_eq!("detective", payload.genres[1].text);
        assert_eq!(20, payload.genres[1].matching);
        assert_eq!(2, payload.authors.len());
        assert_eq!("Heinlein", payload.authors[0].get_last_name().unwrap_or_default());
        assert_eq!("Bradbury", payload.authors[1].get_last_name().unwrap_or_default());
    }

    #[test]
    fn parse_book_title() {
        let xml = r#"<book-title>Рубеж</book-title>"#;
        let obj = Element::parse(xml.as_bytes()).map_or(None, |element| BookTitle::new(&element));
        assert!(obj.is_some());
        let payload = obj.unwrap();
        assert_eq!("Рубеж", payload.text);
    }

}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct SrcTitleInfo{
    pub genres: Vec<Genre>,
    pub authors: Vec<Author>,
    pub book_title: BookTitle,
    pub annotations: Vec<Annotation>,
    pub keywords: Vec<Keywords>,
    pub date: Vec<Date>,
    pub coverpage: Vec<Coverpage>,
    pub lang: Option<Lang>,
    pub src_lang: Option<SrcLang>,
    pub translators: Vec<Translator>,
    pub sequences: Vec<Sequence>,
}
impl Fb2Node for SrcTitleInfo {
    const NAME: &'static str = "src-title-info";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            if let Some(book_title) = query_one(element) {
                return Some(Self {
                    genres: query_subitems(element),
                    authors: query_subitems(element),
                    book_title: book_title,
                    annotations: query_subitems(element),
                    keywords: query_subitems(element),
                    date: query_subitems(element),
                    coverpage: query_subitems(element),
                    lang: query_one(element),
                    src_lang: query_one(element),
                    translators: query_subitems(element),
                    sequences: query_subitems(element),
                })
            }
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct DocumentInfo{
    pub authors: Vec<Author>,
    pub progs: Vec<ProgramUsed>,
    pub date: Vec<Date>,
    pub src_url: Vec<SrcUrl>,
    pub src_ocr: Vec<SrcOcr>,
    pub id: Vec<Id>,
    pub version: Vec<Version>,
    pub history: Vec<History>,
    pub publisher: Vec<Publisher>,

}
impl Fb2Node for DocumentInfo {
    const NAME: &'static str = "document-info";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self {
                authors: query_subitems(element),
                progs: query_subitems(element),
                date: query_subitems(element),
                src_url: query_subitems(element),
                src_ocr: query_subitems(element),
                id: query_subitems(element),
                version: query_subitems(element),
                history: query_subitems(element),
                publisher: query_subitems(element),
            })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct PublishInfo{
    pub book_name: Option<BookName>,
    pub publisher: Option<Publisher>,
    pub city: Option<City>,
    pub year: Option<Year>,
    pub isbn: Option<Isbn>,
    pub sequences: Vec<Sequence>,
}
impl Fb2Node for PublishInfo {
    const NAME: &'static str = "publish-info";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self {
                book_name: query_one(element),
                publisher: query_one(element),
                city: query_one(element),
                year: query_one(element),
                isbn: query_one(element),
                sequences: query_subitems(element),
            })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct CustomInfo; // Not Implemented
impl Fb2Node for CustomInfo {
    const NAME: &'static str = "custom-info";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Output; // Not Implemented
impl Fb2Node for Output {
    const NAME: &'static str = "output";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Genre {
    pub matching: u8,
    pub text: String,
}
impl Fb2Node for Genre {
    const NAME: &'static str = "genre";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Genre {
                matching: element.attributes.get("match").map(|s| s.parse().unwrap_or(0)).unwrap_or(0),
                text: Self::get_text(element),
            })
        }
        return None;
    }
}

#[cfg(test)]
mod genre {
    use super::*;

    #[test]
    fn parse_trivial() {
        let xml = r#"<genre>adv_western</genre>"#;
        let obj = Element::parse(xml.as_bytes()).map_or(None, |element| Genre::new(&element));
        assert!(obj.is_some());
        assert_eq!("adv_western", obj.unwrap().text);
    }

    #[test]
    fn parse_with_match() {
        let xml = r#"<genre match="20">detective</genre>"#;
        let obj = Element::parse(xml.as_bytes()).map_or(None, |element| Genre::new(&element));
        assert!(obj.is_some());
        let payload = obj.unwrap();
        assert_eq!(20, payload.matching);
        assert_eq!("detective", payload.text);
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Author {
    pub first_name: Option<FirstName>,
    pub middle_name: Option<MiddleName>,
    pub last_name: Option<LastName>,
    pub nickname: Option<NickName>,
    pub homepages: Vec<HomePage>,
    pub emails: Vec<Email>,
    pub id: Option<Id>,
}
impl Fb2Node for Author {
    const NAME: &'static str = "author";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self {
                first_name: query_one(element),
                middle_name: query_one(element),
                last_name: query_one(element),
                nickname: query_one(element),
                homepages: query_subitems(element),
                emails: query_subitems(element),
                id: query_one(element),
            })
        }
        return None;
    }
}
impl Author {
    pub fn get_first_name(&self) -> Option<String> {
        self.first_name.clone().map(|v| v.text)
    }
    pub fn get_middle_name(&self) -> Option<String> {
        self.middle_name.clone().map(|v| v.text)
    }
    pub fn get_last_name(&self) -> Option<String> {
        self.last_name.clone().map(|v| v.text)
    }
    pub fn get_nickname(&self) -> Option<String> {
        self.nickname.clone().map(|v| v.text)
    }
    pub fn get_homepages(&self) -> Vec<String> {
        self.homepages.iter().map(|v| v.text.clone()).collect()
    }
    pub fn get_emails(&self) -> Vec<String> {
        self.emails.iter().map(|v| v.text.clone()).collect()
    }
    pub fn get_id(&self) -> Option<String> {
        self.id.clone().map(|v| v.text)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Translator {
    pub first_name: Option<FirstName>,
    pub middle_name: Option<MiddleName>,
    pub last_name: Option<LastName>,
    pub nickname: Option<NickName>,
    pub homepages: Vec<HomePage>,
    pub emails: Vec<Email>,
    pub id: Option<Id>,
}
impl Fb2Node for Translator {
    const NAME: &'static str = "translator";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self {
                first_name: query_one(element),
                middle_name: query_one(element),
                last_name: query_one(element),
                nickname: query_one(element),
                homepages: query_subitems(element),
                emails: query_subitems(element),
                id: query_one(element),
            })
        }
        return None;
    }
}
impl Translator {
    pub fn get_first_name(&self) -> Option<String> {
        self.first_name.clone().map(|v| v.text)
    }
    pub fn get_middle_name(&self) -> Option<String> {
        self.middle_name.clone().map(|v| v.text)
    }
    pub fn get_last_name(&self) -> Option<String> {
        self.last_name.clone().map(|v| v.text)
    }
    pub fn get_nickname(&self) -> Option<String> {
        self.nickname.clone().map(|v| v.text)
    }
    pub fn get_homepages(&self) -> Vec<String> {
        self.homepages.iter().map(|v| v.text.clone()).collect()
    }
    pub fn get_emails(&self) -> Vec<String> {
        self.emails.iter().map(|v| v.text.clone()).collect()
    }
    pub fn get_id(&self) -> Option<String> {
        self.id.clone().map(|v| v.text)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct FirstName{
    pub text: String,
}
impl Fb2Node for FirstName {
    const NAME: &'static str = "first-name";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_capitalized_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct MiddleName{
    pub text: String,
}
impl Fb2Node for MiddleName {
    const NAME: &'static str = "middle-name";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_capitalized_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct LastName{
    pub text: String,
}
impl Fb2Node for LastName {
    const NAME: &'static str = "last-name";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_capitalized_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct NickName{
    pub text: String,
}
impl Fb2Node for NickName {
    const NAME: &'static str = "nickname";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_capitalized_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct HomePage{
    pub text: String,
}
impl Fb2Node for HomePage {
    const NAME: &'static str = "home-page";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Email{
    pub text: String,
}
impl Fb2Node for Email {
    const NAME: &'static str = "email";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Id {
    pub text: String,
}
impl Fb2Node for Id {
    const NAME: &'static str = "id";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_text(element) })
        }
        return None;
    }
}

#[cfg(test)]
mod author {
    use super::*;

    #[test]
    fn parse_all() {
        let xml = r#"
        <author>
            <first-name>Robert</first-name>
            <middle-name>Anson</middle-name>
            <last-name>Heinlein</last-name>
            <nickname>Heinlein</nickname>
            <home-page>example.com</home-page>
            <email>example@example.com</email>
            <id>42</id>
        </author>
        "#;
        let obj = Element::parse(xml.as_bytes()).map_or(None, |element| Author::new(&element));
        assert!(obj.is_some());
        let payload = obj.unwrap();
        assert_eq!("Robert", payload.get_first_name().unwrap_or_default());
        assert_eq!("Anson", payload.get_middle_name().unwrap_or_default());
        assert_eq!("Heinlein", payload.get_last_name().unwrap_or_default());
        assert_eq!("Heinlein", payload.get_nickname().unwrap_or_default());
        assert!(!payload.get_homepages().is_empty());
        assert_eq!("example.com", payload.get_homepages()[0]);
        assert!(!payload.get_emails().is_empty());
        assert_eq!("example@example.com", payload.get_emails()[0]);
        assert_eq!("42", payload.get_id().unwrap_or_default());
    }

    #[test]
    fn parse_id() {
        let xml = r#"<id>C03EEC10-4053-4913-86D0-F379926F3487</id>"#;
        let obj = Element::parse(xml.as_bytes()).map_or(None, |element| Id::new(&element));
        assert!(obj.is_some());
        let payload = obj.unwrap();
        assert_eq!("C03EEC10-4053-4913-86D0-F379926F3487", payload.text);
    }

}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct BookTitle{
    pub text: String,
}
impl Fb2Node for BookTitle {
    const NAME: &'static str = "book-title";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct BookName{
    pub text: String,
}
impl Fb2Node for BookName {
    const NAME: &'static str = "book-name";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Annotation; // Not Implemented
impl Fb2Node for Annotation {
    const NAME: &'static str = "annotation";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Keywords{
    pub text: String,
}
impl Fb2Node for Keywords {
    const NAME: &'static str = "keywords";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Date {
    pub value: String,
    pub lang: Option<String>,
    pub text: String,
}
impl Fb2Node for Date {
    const NAME: &'static str = "date";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self {
                value: element.attributes.get("value").cloned().unwrap_or_default(),
                lang: element.attributes.get("xml:lang").cloned(),
                text: Self::get_text(element),
            })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Image;
impl Fb2Node for Image {
    const NAME: &'static str = "image";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Coverpage {
    pub images: Vec<Image>
}
impl Fb2Node for Coverpage {
    const NAME: &'static str = "coverpage";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self {
                images: query_subitems(element)
            })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Lang{
    pub text: String,
}
impl Fb2Node for Lang {
    const NAME: &'static str = "lang";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct SrcLang{
    pub text: String,
}
impl Fb2Node for SrcLang {
    const NAME: &'static str = "src-lang";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Sequence {
    pub name: String,
    pub number: String,
    pub lang: Option<String>,
}
impl Fb2Node for Sequence {
    const NAME: &'static str = "sequence";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self {
                name: element.attributes.get("name").cloned().unwrap_or_default(),
                number: element.attributes.get("number").cloned().unwrap_or_default(),
                lang: element.attributes.get("xml:lang").cloned(),
            })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct ProgramUsed{
    pub text: String,
}
impl Fb2Node for ProgramUsed {
    const NAME: &'static str = "program-used";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct SrcUrl{
    pub text: String,
}
impl Fb2Node for SrcUrl {
    const NAME: &'static str = "src-url";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct SrcOcr{
    pub text: String,
}
impl Fb2Node for SrcOcr {
    const NAME: &'static str = "src-ocr";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Version{
    pub text: String,
}
impl Fb2Node for Version {
    const NAME: &'static str = "version";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct History; // Not Implemented
impl Fb2Node for History {
    const NAME: &'static str = "history";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Publisher{
    pub text: String,
}
impl Fb2Node for Publisher {
    const NAME: &'static str = "publisher";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct City{
    pub text: String,
}
impl Fb2Node for City {
    const NAME: &'static str = "city";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Year{
    pub text: String,
}
impl Fb2Node for Year {
    const NAME: &'static str = "year";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_text(element) })
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct Isbn{
    pub text: String,
}
impl Fb2Node for Isbn {
    const NAME: &'static str = "isbn";
    fn new(element: &xmltree::Element) -> Option<Self> where Self: std::marker::Sized+Default {
        if Self::ok(element) {
            return Some(Self { text: Self::get_text(element) })
        }
        return None;
    }
}