/// Strip url params
///
/// Remove duplicate query string parameters from a URL and optionally remove specified parameters. Three approaches of increasing Pythonic style.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::strip_url_params("www.saadbenn.com?a=1&b=2&a=2", Some(&["c"]));
/// assert_eq!(result, String::from("www.saadbenn.com?a=1&b=2"));
/// ```
pub fn strip_url_params(url: &str, strip: Option<&[&str]>) -> String {
    let strip = strip.unwrap_or(&[]);
    let strip_set: std::collections::HashSet<&str> = strip.iter().copied().collect();

    let Some((base, query)) = url.split_once('?') else {
        return url.to_string();
    };

    let mut seen = std::collections::HashSet::new();
    let mut params = Vec::new();

    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }

        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));

        if strip_set.contains(key) {
            continue;
        }

        if seen.insert(key.to_string()) {
            params.push((key, value));
        }
    }

    if params.is_empty() {
        return base.to_string();
    }

    let query = params
        .into_iter()
        .map(|(k, v)| {
            if v.is_empty() {
                k.to_string()
            } else {
                format!("{k}={v}")
            }
        })
        .collect::<Vec<_>>()
        .join("&");

    format!("{base}?{query}")
}
