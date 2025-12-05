use std::collections::HashMap;

pub fn get_domain_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("instagram.com", "kkinstagram.com"),
        ("pixiv.net", "phixiv.net"),
        ("x.com", "fixupx.com"),
        ("tiktok.com", "vxtiktok.com"),
        ("twitter.com", "fxtwitter.com"),
        ("reddit.com", "rxddit.com"),
        ("imgur.com", "s.imgur.com"),
        ("facebook.com", "facebed.com"),
    ])
}

pub fn auto_embed(link: &str) -> Option<String> {
    let mut url_str = link.replace("www.", "");
    let domain_map = get_domain_map();
    for (domain, embed_domain) in domain_map.iter() {
        println!("Checking domain: {}", domain);
        if url_str.contains(domain) {
            url_str = url_str.replace(domain, embed_domain);
            return Some(url_str);
        }
    }
    None
}
