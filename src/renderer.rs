use std::fs;

pub struct RenderFlags{
    pub fs_path: String,
}

impl Default for RenderFlags{
    fn default() -> Self {
        RenderFlags{
            fs_path: String::from("/")
        }
    }
}

#[derive(Debug)]
pub enum RenderError{
    InvalidId(String),
    FilesList
}

/// Will search for an identifier and replace it with specific data.
/// Can be thought of as a rudimentary Jinja implementation.
/// 
/// Replacing is done in passes and in-place.
/// 
/// Supported identifiers:
/// - {{files_list}}
/// - {{up_level_link}}
pub fn render_index_page(page: &mut String, flags: &RenderFlags) -> Result<(), RenderError>{
    
    // Supported identifiers must be added here and handled below
    let identifiers = ["{{files_list}}", "{{up_level_link}}"];
    let mut tokens;

    // This is quite an inefficient way of doing this.
    // However, this project does not aim to render pages with multiple
    // identifiers, neither with multiple versions of such identifier
    for id in identifiers{
        tokens = page.split(id).collect::<Vec<&str>>();

        // Identifier not found
        if tokens.len() < 2 {continue;}

        // Render and stitch back together
        let list_html= match id {
            "{{files_list}}" => render_files_list(&flags.fs_path).unwrap_or_default(),
            "{{up_level_link}}" => render_up_level_link(&flags.fs_path).unwrap_or_default(),
            _ => return Err(RenderError::InvalidId("Error rendering file".to_string()))
        };

        *page = tokens.join(&list_html);
    }

    Ok(())
}

fn render_files_list(path: &str) -> Result<String, RenderError>{
    let entries = fs::read_dir(&path)
        .map_err(|_| RenderError::FilesList)?;

    let mut output = String::from("<ul>");

    for entry in entries{
        let entry = entry.map_err(|_| RenderError::FilesList)?;

        output += "<li>";
        
        // File name
        if let Some(name) = entry.path().file_name(){
            let file_name = name.to_str().unwrap_or_default();

            let entry_file_type = entry.file_type();
            if entry_file_type.is_err(){continue;}
            if entry_file_type.unwrap().is_dir(){                
                // Determine the correct url to reference
                let mut href_path = String::from("/fs");
                if path != "/"{
                    href_path += path;
                }
                href_path = format!("{href_path}/{file_name}");
                
                output += format!("<a href={href_path}>{file_name}</a>").as_str();
            }
            else{
                output += file_name;
            }
        }

        output += "</li>";
    }

    output += "</ul>";
    
    Ok(output)
}

fn render_up_level_link(path: &str) -> Result<String, RenderError>{
    let mut output = String::from("<a href=/fs");

    let fs_path = path
        .split("/")
        .collect::<Vec<&str>>();

    output += &fs_path[..fs_path.len()-1].join("/");
    
    output += ">../</a>";
    Ok(output)
}