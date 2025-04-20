use std::fs::File;
use std::io::prelude::*;
use regex::Regex;



fn render_header(header_level: &usize, header_content: &str) -> String {
     format!("<h{}>{}</h{}>",header_level,header_content,header_level)
}


fn main() -> std::io::Result<()> {
    let mut in_file = File::open("src/test.md")?;
    let mut contents = String::new();
    let mut components: Vec<String> = vec![];

    let header_re = Regex::new(r"^(#+)\s+(.*)").unwrap();
    let strong_re = Regex::new(r"\*\*(.*?)\*\*").unwrap();
    

    
    in_file.read_to_string(&mut contents)?;
    
    //replace italic
    // todo
    
    // replace all occourences of ** _ ** with the strong html tag
    // ie generate <strong> tags for the content of these
    let result = strong_re.replace_all(&contents, |caps: &regex::Captures<'_>|  {
        format!("<strong>{}</strong>",&caps[1])
    } ).into_owned();

    let mut out = String::new();
    for line in result.lines() {
        if let Some(caps) = header_re.captures(line) {
            let header_level = &caps[1].len();
            let header_content = &caps[2];
            out = out + &render_header(&header_level, &header_content);             
       }
        else {
            if !line.is_empty() {
                out = out + &format!("{}",line);
            }else {
                out = out + "<br>"
            }
        }
    
        out= out + "\n";
     }
    // let place_p_tags_re = Regex::new(r">([\s\S]*?)<").unwrap();
    // let out = place_p_tags_re.replace_all(&out, |caps: &regex::Captures<'_>|  {
    //     format!("<p>{}</p>",&caps[1])
    // } ).into_owned();
    let mut out_file = File::create("index.html")?;
    out_file.write_all(out.as_bytes())?;


    Ok(())
}


