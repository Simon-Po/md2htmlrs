use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum ComponentKind {
    Paragraph,
    Header { level: usize },
    LineBreak,
    LineThrough,
}
#[derive(Debug)]
struct Component {
    kind: ComponentKind,
    content: String,
}

fn main() -> std::io::Result<()> {
    let mut in_file = File::open("src/test.md")?;
    let mut contents = String::new();
    let mut components: Vec<Component> = vec![];

    let strong_re = Regex::new(r"\*\*(.*?)\*\*").unwrap();

    // Matches italic text enclosed in single asterisks. It ensures that the content
    // doesn't start or end with an asterisk and allows either non-asterisk characters
    // or complete strong tags ('**...**') within the italicized text, preventing
    // single '*' inside strong from being incorrectly identified as italic markers.
    // gets it correct most of the time but not always
    let italic_re = Regex::new(r"\*([^\*]([^\*]|\*\*[^\*]*\*\*)*[^\*])\*").unwrap();
    in_file.read_to_string(&mut contents)?;


    // replace all occourences of ** _ ** with the strong html tag
    // ie generate <strong> tags for the content of these
    let result_strong = strong_re
        .replace_all(&contents, |caps: &regex::Captures<'_>| {
            format!("<strong>{}</strong>", &caps[1])
        })
        .into_owned();
    let result = italic_re 
        .replace_all(&result_strong, |caps: &regex::Captures<'_>| {
            format!("<em>{}</em>", &caps[1])
        })
        .into_owned();


    collect_components(&mut components, result);

    let consolidated_components = consolidate_components(components);

    let html = render(&consolidated_components);

    let mut out_file = File::create("index.html")?;
    out_file.write_all(html.as_bytes())?;

    Ok(())
}

fn render(components: &Vec<Component>) -> String {
    let mut out: String = String::new();

    for comp in components {
        match comp.kind {
            ComponentKind::LineBreak => out.push_str("<br>"),
            ComponentKind::Paragraph => out.push_str(&format!("<p>{}</p>", &comp.content)),
            ComponentKind::Header { level } => {
                out.push_str(&format!("<h{}>{}</h{}>", level, &comp.content, level))
            }
            ComponentKind::LineThrough => out.push_str("<hr>"),
        };
    }

    return out;
}

fn collect_components(components: &mut Vec<Component>, result: String) {
    let header_re = Regex::new(r"^(#+)\s+(.*)").unwrap();
    let line_re = Regex::new(r"^-{3,}$").unwrap();
    for line in result.lines() {
        if let Some(caps) = header_re.captures(line) {
            let header_level = &caps[1].len();
            let header_content = &caps[2];
            components.push(Component {
                kind: ComponentKind::Header {
                    level: *header_level as usize,
                },
                content: String::from(header_content),
            });
        } else if let Some(_) = line_re.captures(line) {
            components.push(Component {
                kind: ComponentKind::LineThrough,
                content: String::new(),
            })
        } else {
            if !line.is_empty() {
                components.push(Component {
                    kind: ComponentKind::Paragraph,
                    content: String::from(line),
                });
            } else {
                components.push(Component {
                    kind: ComponentKind::LineBreak,
                    content: String::from(""),
                })
                //out = out + "<br>"
            }
        }
    }
}

fn consolidate_components(components: Vec<Component>) -> Vec<Component> {
    let mut consolidated_components: Vec<Component> = Vec::new();
    let mut current_paragraph_content: Vec<String> = Vec::new();

    for component in components {
        match component.kind {
            ComponentKind::Paragraph => {
                current_paragraph_content.push(component.content);
            }
            _ => {
                // If we have accumulated paragraph content, create a single paragraph component
                if !current_paragraph_content.is_empty() {
                    consolidated_components.push(Component {
                        kind: ComponentKind::Paragraph,
                        content: current_paragraph_content.join("<br>"),
                    });
                    current_paragraph_content.clear();
                }
                // Push the current non-paragraph component
                consolidated_components.push(component);
            }
        }
    }

    // Don't forget to push any remaining accumulated paragraph content
    if !current_paragraph_content.is_empty() {
        consolidated_components.push(Component {
            kind: ComponentKind::Paragraph,
            content: current_paragraph_content.join("\n"),
        });
    }
    consolidated_components
}
