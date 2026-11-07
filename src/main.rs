use core::fmt;
use std::cmp::min;
// use std::default;
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use::std::io::{self,IsTerminal};
use owo_colors::OwoColorize;

#[derive(Debug)]
enum ColorMode {
    Never,
    Always,
    Auto
}

impl FromStr for ColorMode {
    type Err = RgrepConfigError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            _ => Err(RgrepConfigError(
                format!("invalid color moode '{}'; explected auto, always or never", s)
            ))
        }
    }
}

impl ColorMode {
    fn should_color(&self) -> bool {
        match self {
            ColorMode::Always => true,
            ColorMode::Never => false,
            ColorMode::Auto => io::stdout().is_terminal(),
        }
    }
}

impl Default for ColorMode {
    fn default() -> Self {
        Self::Auto
    }
}


#[derive(Debug)]
struct RgrepConfigBuilder {
    search_term: Option<String>,
    file_name: Option<String>,
    ignore_case: bool,
    invert_search: bool,
    extension: Option<String>,
    color_mode: ColorMode
}

#[derive(Debug)]
struct RgrepConfig {
    search_term: String,
    file_name: String,
    ignore_case: bool,
    #[allow(dead_code)]
    invert_search: bool,
    #[allow(dead_code)]
    extension: Option<String>,
    color_mode: ColorMode
}

#[derive(Debug)]
struct RgrepConfigError(String);

impl fmt::Display for RgrepConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}", self.0)
    }
}

impl std::error::Error for RgrepConfigError {}



impl TryFrom<RgrepConfigBuilder> for RgrepConfig {
    type Error = RgrepConfigError;

    fn try_from(value: RgrepConfigBuilder) -> Result<RgrepConfig, RgrepConfigError> {
        let search_term = value.search_term.ok_or_else(|| {
            RgrepConfigError("search_term is required".to_string())
        })?;

        let file_name = value.file_name.ok_or_else(|| {
            RgrepConfigError("file_name is required".to_string())
        })?;

        Ok({
            RgrepConfig { search_term, file_name, ignore_case: value.ignore_case, invert_search: value.invert_search, extension: value.extension, color_mode: value.color_mode }
        })
    }
}

impl RgrepConfig {

    pub fn parse_args(args: &mut env::Args) -> Result<RgrepConfig,RgrepConfigError> {
        let _ = args.next(); // executable path

        let mut builder = RgrepConfigBuilder::default();

        while let Some(arg) = args.next() {
            let matched_arg = arg.as_str();
            match matched_arg {
                "-i" => { builder.ignore_case = true },
                "-v" => { builder.invert_search = true },
                "--ext" => {
                    let ext = args.next().ok_or_else(|| {
                        RgrepConfigError(String::from("no value provided for the --ext flag"))
                    })?;

                    builder.extension = Some(ext);
                },
                "--color" => {
                    let color_mode: ColorMode = args.next().ok_or_else(|| {
                        RgrepConfigError("no value provided for the --color flag".to_string())                        
                    })?
                    .parse()?;

                    builder.color_mode = color_mode;
                },
                other if other.starts_with("-")  => {
                    return Err(RgrepConfigError(format!("unknwown flag {} cannot be parsed", other)))
                },
                other => {
                    if builder.search_term.is_none() {
                        builder.search_term = Some(String::from(other));
                    }
                    else if builder.file_name.is_none() {
                        builder.file_name = Some(String::from(other)) 
                    } else {
                        return Err(
                            RgrepConfigError(format!("unexpected positional argument: {}",other))
                        )
                    }
                },
            }
        }
        
        let config = RgrepConfig::try_from(builder)?;

        return Ok(config);
    }
}

impl Default for RgrepConfigBuilder {
    fn default() -> Self {
        RgrepConfigBuilder { search_term: None, file_name: None, extension: None, ignore_case: false, invert_search: false, color_mode: ColorMode::default() }
    }
}

fn main() {
    let config = RgrepConfig::parse_args(&mut env::args());
    match config {
        Ok(config) => { rgrep(&config); }
        Err(err) => { eprintln!("rgrep: {}", err)}
    }

}

fn rgrep(config: &RgrepConfig) {
    let path = Path::new(&config.file_name);

    if path.exists() && path.is_file() {
        let contents = fs::read_to_string(path);
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if let Ok(contents) = contents {
            for (lineno, line) in contents.lines().enumerate() {
                
                let t_line = if config.ignore_case { line.to_lowercase()} else { line.to_string() };
                let search_term = if config.ignore_case { config.search_term.to_lowercase() } else { config.search_term.to_string() };

                let sub_index = t_line.find(&search_term);
                
                
                    if let Some(index) = sub_index{

                        if config.color_mode.should_color() {
                            let start = index;
                            let end = index + search_term.len();
    
                            let before = &line[..start];
                            let matched = &line[start..end];
                            let after = &line[end..];
    
                            let output_line = format!("{}{}{}",before, matched.to_string().red().bold(), after);
                            println!("{}:{}\t{}", file_name, lineno+1, output_line);
                        } else {
                            println!("{}:{}\t{}", file_name, lineno+1, line);
                        }  
                    } 
                
            }
        }
        else {
            println!("error: couldn't read file: {}", config.file_name);
        }
    }
    else {
        println!("error: {} doesn't exist",config.file_name)
    }
}
