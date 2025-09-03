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


fn main() {
    use std::io::Read;
    
    // Reading HAR file from stdin. Unfortunately gjson does not support streaming.
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input);
    
    let link = gjson::get(&input, "log.pages.0.title");
    let har = gjson::get(&input, concat!(
        "log.entries",
            ".#(response.content.mimeType=\"image/png\")#",
            "|@this",
            ".#(response.status==200)#",
    ));
    
    println!("The page {:?} depends on the following images:", link.str());
    for (i, image) in har.array().into_iter().enumerate() {
        let url = image.get("request.url");
        let content = image.get("response.content");
        
        // Important: use u64 because files of our era can easily exceed 4GB.
        let (size_numeral, size_unit) = size_formatter(content.get("size").u64());
        
        let mime = content.get("mimeType").str().trim_start_matches("image/").to_uppercase();
        
        // let image_data = content.get("text").str();    must handle encoding though
        
        println!("{}. {mime} image of {size_numeral} {size_unit}, loaded from {}",
            i + 1, url.str());
    }
    
}
