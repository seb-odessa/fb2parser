#![feature(test)]

#[cfg(bench)]
extern crate test;
extern crate xmltree;

pub mod util;
pub mod fictionbook;
pub use fictionbook::FictionBook;
pub mod description;
pub use description::Description;
pub mod title_info;
pub use title_info::TitleInfo;
pub mod document_info;
pub use document_info::DocumentInfo;
pub mod publish_info;
pub use publish_info::PublishInfo;
pub mod genre;
pub use genre::Genre;
pub mod author;
pub use author::Author;
pub mod translator;
pub use translator::Translator;
pub mod book_title;
pub use book_title::BookTitle;
pub mod lang;
pub use lang::Lang;
pub mod src_lang;
pub use src_lang::SrcLang;
pub mod first_name;
pub use first_name::FirstName;
pub mod last_name;
pub use last_name::LastName;
pub mod middle_name;
pub use middle_name::MiddleName;
pub mod nickname;
pub use nickname::Nickname;
pub mod sequence;
pub use sequence::Sequence;
pub mod program_used;
pub use program_used::ProgramUsed;
pub mod date;
pub use date::Date;
pub mod publisher;
pub use publisher::Publisher;
pub mod book_name;
pub use book_name::BookName;
pub mod city;
pub use city::City;
pub mod year;
pub use year::Year;
pub mod isbn;
pub use isbn::Isbn;

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use xmltree::{Element, ParseError};

    #[bench]
    fn parse_fiction_book(bencher: &mut Bencher) {
        let xml = XML.as_bytes();
        bencher.iter(|| {
            let _: Result<Element, ParseError> = Element::parse(xml);
        });
    }
    #[bench]
    fn parse_description(bencher: &mut Bencher) {
        let xml = DESCRIPTION.as_bytes();
        bencher.iter(|| {
            let _: Result<Element, ParseError> = Element::parse(xml);
        });
    }

    #[bench]
    fn parse_title_info(bencher: &mut Bencher) {
        let xml = TITLE_INFO.as_bytes();
        bencher.iter(|| {
            let _: Result<Element, ParseError> = Element::parse(xml);
        });
    }

    #[bench]
    fn parse_author(bencher: &mut Bencher) {
        let xml = AUTHOR.as_bytes();
        bencher.iter(|| {
            let _: Result<Element, ParseError> = Element::parse(xml);
        });
    }


    pub const XML: &str = "
    <?xml version=\"1.0\" encoding=\"utf-8\"?>
    <FictionBook xmlns=\"http://www.gribuser.ru/xml/fictionbook/2.0\" xmlns:l=\"http://www.w3.org/1999/xlink\">
    <description>
        <title-info>
            <genre>sf_space</genre>
            <author>
                <first-name>Дж. Майкл</first-name>
                <last-name>Стражинский</last-name>
            </author>
            <book-title>Тень его мыслей</book-title>
            <keywords>Вавилон 5</keywords>
            <date>1999</date>
            <coverpage>
                <image l:href=\"Any2FbImgLoader0\"/>
            </coverpage>
            <lang>ru</lang>
        </title-info>
        <document-info>
            <author>
                <first-name></first-name>
                <last-name></last-name>
            </author>
            <program-used></program-used>
            <date value=\"2008-03-06\">2008-03-06</date>
            <id></id>
            <version></version>
        </document-info>
    </description>
    </FictionBook>";

    pub const DESCRIPTION: &str = "
    <description>
        <title-info>
            <genre>sf_space</genre>
            <author>
                <first-name>Дж. Майкл</first-name>
                <last-name>Стражинский</last-name>
            </author>
            <book-title>Тень его мыслей</book-title>
            <keywords>Вавилон 5</keywords>
            <date>1999</date>
            <coverpage>
                <image l:href=\"Any2FbImgLoader0\"/>
            </coverpage>
            <lang>ru</lang>
        </title-info>
        <document-info>
            <author>
                <first-name></first-name>
                <last-name></last-name>
            </author>
            <program-used></program-used>
            <date value=\"2008-03-06\">2008-03-06</date>
            <id></id>
            <version></version>
        </document-info>
    </description>";

    pub const TITLE_INFO: &str = "
    <title-info>
        <genre>sf_space</genre>
        <author>
            <first-name>Дж. Майкл</first-name>
            <last-name>Стражинский</last-name>
        </author>
        <book-title>Тень его мыслей</book-title>
        <keywords>Вавилон 5</keywords>
        <date>1999</date>
        <coverpage>
            <image l:href=\"Any2FbImgLoader0\"/>
        </coverpage>
        <lang>ru</lang>
    </title-info>";

    pub const AUTHOR: &str = "
        <author>
            <first-name>Дж. Майкл</first-name>
            <last-name>Стражинский</last-name>
        </author>";

}
