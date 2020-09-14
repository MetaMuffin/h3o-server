

pub fn split<'a>(s: &'a str, delimeter: &str) -> Option<(&'a str,&'a str)> {
    match s.find(delimeter) {
        None => return None,
        Some(pos) => {
            let (a,b_with_del) = s.split_at(pos);
            let b = &b_with_del[(delimeter.len())..];
            return Some((a,b));
        }
    }
}
