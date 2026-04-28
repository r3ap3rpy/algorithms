/// Domain Name Extractor
///
/// Given a URL as a string, parse out just the domain name and return it. Uses only the .split() built-in function without regex or urlparse.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::domain_name("https://github.com/SaadBenn");
/// assert_eq!(result, String::from("github"));
/// ```
pub fn domain_name(url: &str) -> String {
    let full_domain_name = url.split("//").last().unwrap_or("");
    let actual_domain_name: Vec<&str> = full_domain_name.split(".").collect();
    if actual_domain_name.len() > 2 {
        actual_domain_name[1].to_string()
    } else {
        actual_domain_name[0].to_string()
    }
}
