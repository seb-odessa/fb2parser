/*********************************************************************************************
 Элемент <last-name>
Описание

Фамилия автора, переводчика или правообладателя.
Версия FB

2.0 и выше
Поддерживается

TODO: где поддерживается?
Атрибуты

    xml:lang (опциональный) - язык.

Подчиненные элементы

Нет дочерних элементов, содержит текст - собственно фамилия.
Подчинен

Может содержаться в следующих элементах:

    <author>;
    <translator>;
    <publisher> с версии 2.2.
*********************************************************************************************/
use std::fmt;
use xmltree::Element;
use util::HasFrom;

#[derive(Debug, PartialEq)]
pub struct LastName {
    pub text: String,
}
impl HasFrom<LastName> for LastName {
    fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(LastName { text: node.text.clone().unwrap_or_default() })
        } else {
            None
        }
    }
}
impl fmt::Display for LastName {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xmltree::Element;
    use util::from;
    const TEST_DATA: &'static str = "<root><last-name>value</last-name></root>";

    #[test]
    fn from_trait_impl() {
        let root = Element::parse(TEST_DATA.as_bytes()).unwrap();
        assert_eq!(
            LastName { text: "value".to_owned() },
            from(&root, "last-name").unwrap()
        );
    }
}