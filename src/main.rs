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

fn approx_bits_eq(bits_len_a: i64, bits_len_b: i64) -> bool {
    (bits_len_a - bits_len_b).abs() < 64
}


#[derive(Debug)]
struct PartialDebug {
    response_content_size: u64,
    response_text_encoding: String,
    response_text_len: usize,
    
    image_response_headers: String,
}


macro_rules! read_mime_prefix {
    ($input:expr, ($link:expr), $mime_pfx:literal => $media_type:literal) => {{
        let har = gjson::get(&$input, concat!(
            "log.entries",
                ".#(response.content.mimeType%\"", $mime_pfx, "*\")#",
                "|@this",
                // ".#(response.status==200)#",
        ));
        
        let mut print_partial = None;
        
        println!("The page {:?} depends on the following {}s:", $link.str(), $media_type);
        for (i, image) in har.array().into_iter().enumerate() {
            let url_guard = image.get("request.url");
            let content = image.get("response.content");
            let size_bytes = content.get("size").u64();
            let (size_numeral, size_unit) = size_formatter(size_bytes);
            
            let mut partial = image.get("response.status").u32() == 206;
            let mut missing = !content.get("text").exists();
            if !missing {
                let have_chars = content.get("text").str().len();
                let have_bits = match content.get("encoding").str() {
                    "base64" => have_chars * 6,
                    ""       => have_chars * 8,  // text-based like SVG
                    _        => panic!("{content}"),
                };
                if approx_bits_eq(have_bits as i64, size_bytes as i64 * 8) {
                    partial = false;
                }
            }
            
            if print_partial.is_none() && partial {
                print_partial = Some(PartialDebug {
                    response_content_size: size_bytes,
                    response_text_encoding: content.get("encoding").str().to_owned(),
                    response_text_len: content.get("text").str().len(),
                    image_response_headers: image.get("response.headers").to_string(),
                });
            }
            
            let mime = content.get("mimeType").str()
                .trim_start_matches($mime_pfx).to_uppercase();
            
            let mut url = url_guard.str();
            if url.starts_with("data:") {
                url = "<a data URL>";
                missing = false;
            }
            
            let warns = match (partial, missing) {
                (true, true)  => "[PART,ABSENT] ",
                (true, false) => "[PARTIAL] ",
                (_,    true)  => "[MISSING] ",
                (_,    false) => ""
            };
            
            println!("{}. {warns}{mime} {} of {size_numeral} {size_unit}, loaded from {url}",
                i + 1, $media_type);
        }
        
        dbg!(print_partial);
        
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
    
    let exchanges = gjson::get(&input, "log.entries.#").u64();
    println!("{exchanges} network exchanges in total");
}
