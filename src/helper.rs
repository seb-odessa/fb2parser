pub fn escape(xml: &String) -> String {
    if xml.find("&amp;").is_none() {
        if xml.find("&").is_some() {
            return xml.replace("&amp;", "&").replace("&", "&amp;");
        }
    }
    return xml.clone();
}

pub fn get_tag(content: &str, tag: &str) -> Option<String> {
    let beg = String::from("<") + tag + ">";
    let end = String::from("</") + tag + ">";
    if let Some(spos) = content.find(&beg) {
        if let Some(epos) = content.find(&end) {
            let needle: &str = &content[spos..epos + end.len()];
            println!("get_tag({}, {}) -> {}", content, tag, needle);
            return Some(String::from(needle));
        }
    }
    None
}

pub fn deduplicate_tags(xml: &String, parent: &str, tag: &str) -> String {
    if let Some(content) = get_tag(&xml, parent) {
        if let Some(value) = get_tag(&xml, tag) {
            if let Some(first) = content.find(&value) {
                if let Some(last) = content.rfind(&value) {
                    if first != last {
                        return xml.replacen(&value, "", 1);
                    }
                }
            }
        }
    }
    return xml.clone();
}
