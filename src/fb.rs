
use serde_xml_rs::Error;
use serde_xml_rs::deserialize;


#[derive(Debug, Deserialize, PartialEq)]
pub struct Sequence {
    pub name: String,
    #[serde(default)]
    pub number: String,
    #[serde(rename = "xml:lang", default)]
    pub lang: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Author {
     #[serde(rename = "first-name", default)]
     pub first_name: String,
     #[serde(rename = "middle-name", default)]
     pub middle_name: String,
     #[serde(rename = "last-name", default)]
     pub last_name: String,
     #[serde(rename = "nickname", default)]
     pub nick_name: String,
     #[serde(rename = "home-page", default)]
     pub home_page: String,
     #[serde(rename = "email", default)]
     pub email: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TitleInfo {
//     genre: Vec<Genre>,
    #[serde(rename = "author", default)]
    pub author: Vec<Author>,
    #[serde(rename = "book-title", default)]
    pub book_title: String,
//     // annotation: OptionalAnnotation,
//     // keywords
//     // date
//     //  coverpage
    #[serde(rename = "lang", default)]
    pub lang: String,
    #[serde(rename = "src-lang", default)]
    pub src_lang: String,
    #[serde(rename = "translator", default)]
    pub translator: Vec<Author>,
    #[serde(rename = "sequence", default)]
    pub sequence: Vec<Sequence>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Description {
    #[serde(rename = "title-info")]
    pub title_info: TitleInfo,
    //src_title_info: OptionalTitleInfo,
    //document_info: DocumentInfo,
    //publish_info: Vec<PublishInfo>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct FictionBook {
    #[serde(rename = "description")]
    pub description: Description,
}

impl FictionBook {
    pub fn new(xml: &String) -> Result<Self, Error> {
        deserialize(xml.as_bytes())
    }

    pub fn brief_info(&self) -> String {
        format!("{:50} - {} {} {} ({}, {})", 
        self.description.title_info.book_title, 
        self.description.title_info.author[0].first_name,
        self.description.title_info.author[0].middle_name,
        self.description.title_info.author[0].last_name,
        self.description.title_info.sequence[0].name,
        self.description.title_info.sequence[0].number

        )

    }
}