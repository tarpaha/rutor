use std::collections::HashSet;

pub fn parse(html_content: &str) -> Option<String> {
    const KEYS: &[&str] = &["Описание", "О фильме"];
    for key in KEYS {
        if let Some(key_pos) = html_content.find(key) {
            let chars = get_filtered_chars(&html_content[key_pos + key.len()..]);
            if let Some(russian_pos) = find_first_russian(&chars) {
                if let Some(end) = chars[russian_pos..].iter().position(|&c| c == '<') {
                    return Some(chars[russian_pos..russian_pos + end].iter().collect());
                }
            }
        }
    }
    None
}

fn get_filtered_chars(s: &str) -> Vec<char> {
    s.chars()
        .filter(|&c| c != '«' && c != '»')
        .collect()
}

fn find_first_russian(chars: &Vec<char>) -> Option<usize> {
    let russian_chars: HashSet<char> = ('а'..='я').collect();
    chars.iter().enumerate()
        .find(|(_, c)| russian_chars.contains(&c.to_lowercase().next().unwrap()))
        .map(|(i, _)| i)
}
#[cfg(test)]
mod test_first_russian {
    use super::*;

    fn to_char_array(s: &str) -> Vec<char> {
        s.chars().collect::<Vec<char>>()
    }

    #[test]
    fn empty_string() {
        assert_eq!(find_first_russian(&to_char_array("")), None);
    }

    #[test]
    fn no_russian() {
        assert_eq!(find_first_russian(&to_char_array("Hello123")), None);
    }

    #[test]
    fn starts_with_russian() {
        assert_eq!(find_first_russian(&to_char_array("привет")), Some(0));
    }

    #[test]
    fn russian_after_latin() {
        assert_eq!(find_first_russian(&to_char_array("Hello Привет")), Some(6));
    }

    #[test]
    fn mixed_case() {
        assert_eq!(find_first_russian(&to_char_array("Test ПРИВЕТ привет")), Some(5));
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    
    #[test]
    fn case1() {
        let html = "<b>О фильме: </b><span style=\"color:#997600;\">Вдова Вайолет</span><br />";
        assert_eq!(parse(html), Some("Вдова Вайолет".to_string()));
    }

    #[test]
    fn case2() {
        let html = "<b>О фильме:</b><br />Левон Кейд<br />";
        assert_eq!(parse(html), Some("Левон Кейд".to_string()));
    }

    #[test]
    fn case3() {
        let html = "<b>Описание</b>: Уокер, матёрый детектив полиции<br />";
        assert_eq!(parse(html), Some("Уокер, матёрый детектив полиции".to_string()));
    }

    #[test]
    fn case4() {
        let html = "<b>Описание:</b> «Одноразовый сотрудник» Микки<br />";
        assert_eq!(parse(html), Some("Одноразовый сотрудник Микки".to_string()));
    }

    #[test]
    fn test_no_description() {
        let html = "<b>Wrong Key</b>: Some text<br />";
        assert!(parse(html).is_none());
    }
}