/*********************************************************************************************
 Элемент <genre>
Описание

Описывает жанровую принадлежность книги. Используется для помещения книги в рубрикатор библиотеки, по этой причине список возможных жанров жестко задается. Допускается указание нескольких жанров.
Версия FB

2.0 и выше
Поддерживается

    Всеми (обеими) библиотеками, ориентироваными на FB2.
    Библиотечным софтом.
    Многими "Читалками"

Атрибуты

    match (опциональный, значение по умолчанию "100") ? число от "1" до "100", задающее субъективное процентное соответствие данному жанру.

Подчиненные элементы

Нет дочерних элементов.

Содержит текст - обозначение жанра из списка жанров.
Подчинен

Может содержаться в следующих элементах:

    <title-info>
    <src-title-info> с версии 2.1
 *********************************************************************************************/
use std::fmt;
use xmltree::Element;
use util::HasFrom;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Genre {
    pub text: String,
}
impl Genre {
    pub fn get(&self)->String {
        String::from(self.text.trim())
    }
}
impl HasFrom<Genre> for Genre {
    fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(Genre { text: node.text.clone().unwrap_or_default() })
        } else {
            None
        }
    }
}
impl fmt::Display for Genre {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.text)
    }
}
