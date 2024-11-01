use std::fs;
use std::io::Write;
use std::path::Path;
use std::error::Error;
use url::Url;
use crate::analyzer::AnalysisResult;

pub struct FileStorage {
    base_dir: String,
}

impl FileStorage {
    pub fn new(base_dir: &str) -> Result<Self, Box<dyn Error>> {
        fs::create_dir_all(base_dir)?;
        Ok(Self {
            base_dir: base_dir.to_string(),
        })
    }

    pub fn save(&self, url: &str, analysis: &AnalysisResult) -> Result<(), Box<dyn Error>> {
        let parsed_url = Url::parse(url)?;
        let host = parsed_url.host_str().unwrap_or("unknown");
        let path = parsed_url.path().replace('/', "_");
        
        let filename = format!("{}_{}.txt", host, path);
        let filepath = Path::new(&self.base_dir).join(filename);

        let mut file = fs::File::create(filepath)?;
        file.write_all(analysis.to_string().as_bytes())?;

        Ok(())
    }
}

