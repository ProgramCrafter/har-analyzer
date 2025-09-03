fn size_formatter(bytes: u64) -> (f32, &'static str) {
    const HALF_KB: u64 = 512;
    const HALF_MB: u64 = HALF_KB * 1024;
    const HALF_GB: u64 = HALF_MB * 1024;
    
    let half_size = (bytes as f32) / 2.0;
    
    match bytes {
        0..HALF_MB       => (half_size / HALF_KB as f32, "KiB"),
        HALF_MB..HALF_GB => (half_size / HALF_MB as f32, "MiB"),
        _                => (half_size / HALF_GB as f32, "GiB"),
    }
}


macro_rules! read_mime_prefix {
    ($input:expr, ($link:expr), $mime_pfx:literal => $media_type:literal) => {{
        let har = gjson::get(&$input, concat!(
            "log.entries",
                ".#(response.content.mimeType%\"", $mime_pfx, "*\")#",
                "|@this",
                // ".#(response.status==200)#",
        ));
        
        println!("The page {:?} depends on the following {}s:", $link.str(), $media_type);
        for (i, image) in har.array().into_iter().enumerate() {
            let url_guard = image.get("request.url");
            let content = image.get("response.content");
            let (size_numeral, size_unit) = size_formatter(content.get("size").u64());
            
            let warns = if image.get("response.status").u32() == 206 {
                "[PARTIAL] "
            } else {
                ""
            };
            
            let mime = content.get("mimeType").str()
                .trim_start_matches($mime_pfx).to_uppercase();
            
            let mut url = url_guard.str();
            if url.starts_with("data:") {
                url = "<a data URL>";
            }
            
            println!("{}. {warns}{mime} {} of {size_numeral} {size_unit}, loaded from {url}",
                i + 1, $media_type);
        }
        println!();
    }}
}


fn main() {
    use std::io::Read;
    
    // Reading HAR file from stdin. Unfortunately gjson does not support streaming.
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input);
    
    let link = gjson::get(&input, "log.pages.0.title");
    read_mime_prefix!{ input, (link), "image/" => "image" };
    read_mime_prefix!{ input, (link), "video/" => "video" };
}
