use colored::Colorize;
use std::io::Write;

pub struct Config {
    pub input_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() != 2 {
            return Err("Please provide the path to minify.");
        }

        let input_path = args[1].clone();

        Ok(Config { input_path })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = &config.input_path;
    let input_metadata = std::fs::metadata(input_path)?;
    let mut cfg = minify_html::Cfg::new();

    cfg.preserve_brace_template_syntax = true;
    cfg.minify_css = true;
    cfg.minify_js = true;
    cfg.keep_spaces_between_attributes = true;
    cfg.do_not_minify_doctype = true;
    cfg.keep_closing_tags = true;
    cfg.ensure_spec_compliant_unquoted_attribute_values = true;

    if input_metadata.is_file() {
        minify_file(input_path, &cfg)?;
    } else if input_metadata.is_dir() {
        minify_dir(input_path, &cfg)?;
    }

    Ok(())
}

/// Minify a file
fn minify_file(file_path: &str, cfg: &minify_html::Cfg) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read(file_path)?;
    let minified_html = minify_html::minify(&contents, cfg);

    let path = std::path::Path::new(file_path);
    let file_name_of_path = path.to_str().expect("Expected file name to exist");
    let root_dir_name = std::path::Path::new(file_path)
        .parent()
        .and_then(|parent| parent.components().nth(0))
        .map(|c| c.as_os_str().to_str().unwrap())
        .unwrap_or_default();
    let file_path_without_root = path.parent().unwrap().strip_prefix(root_dir_name)?;
    let output_dir = std::path::PathBuf::from("templates").join(file_path_without_root);

    if !output_dir.exists() {
        std::fs::create_dir_all(&output_dir)?;
    }

    let outpule_file_path = output_dir.join(path.file_name().unwrap().to_str().unwrap());
    let mut file = std::fs::File::create(outpule_file_path.clone())?;

    file.write_all(&minified_html)?;

    println!("{}{}", "Minified: ".bold(), file_name_of_path,);

    Ok(())
}

/// Minify a directory
fn minify_dir(dir_path: &str, cfg: &minify_html::Cfg) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_paths: Vec<std::path::PathBuf> = Vec::new();

    collect_filenames_recursively(dir_path, &mut file_paths)?;

    for path in &file_paths {
        if let Err(err) = minify_file(path.to_str().unwrap(), cfg) {
            eprintln!("{}: {}", "Error".red(), err);
        }
    }

    Ok(())
}

fn collect_filenames_recursively(
    path: &str,
    files: &mut Vec<std::path::PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let path_name = path.to_str().expect("Expected path to have a name");

        if path.is_dir() {
            collect_filenames_recursively(path_name, files)?;
        } else {
            files.push(path.clone());
        }
    }

    Ok(())
}
