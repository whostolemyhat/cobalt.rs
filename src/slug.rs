use itertools::Itertools;
use regex::Regex;

lazy_static!{
    static ref SLUG_INVALID_CHARS: Regex = Regex::new(r"([^a-zA-Z0-9]+)").unwrap();
}

/// Create a slug for a given file.  Correlates to Jekyll's :slug path tag
pub fn slugify(name: &str) -> String {
    let slug = SLUG_INVALID_CHARS.replace_all(name, "-");
    slug.trim_matches('-').to_lowercase()
}

/// Title-case a single word
fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => {
            f.to_uppercase()
                .chain(c.flat_map(|t| t.to_lowercase()))
                .collect()
        }
    }
}

/// Format a user-visible title out of a slug.  Correlates to Jekyll's "title" attribute
pub fn titleize_slug(slug: &str) -> String {
    slug.split('-').map(title_case).join(" ")
}

#[test]
fn test_slugify() {
    let actual = slugify("___filE-worlD-__09___");
    assert_eq!(actual, "file-world-09");
}

#[test]
fn test_titleize_slug() {
    let actual = titleize_slug("tItLeIzE-sLuG");
    assert_eq!(actual, "Titleize Slug");
}
