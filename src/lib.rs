use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write, Result};

/// Simple Text Editor
#[derive(Debug,PartialEq , Eq ,PartialOrd , Ord)]
pub struct TextEditor {
    content: String,
}

impl TextEditor {
    /// Create a new empty TextEditor.
    pub fn new() -> Self {
        TextEditor {
            content: String::new(),
        }
    }

    /// Add text to the end of the current content.
    pub fn add_text(&mut self, new_text: &str) {
        self.content.push_str(new_text);
    }

    /// Remove the first occurrence of a substring from the content.
    pub fn remove_text(&mut self, substring: &str) {
        if let Some(pos) = self.content.find(substring) {
            self.content.replace_range(pos..pos + substring.len(), "");
        }
    }

    /// Replace all occurrences of one substring with another.
    pub fn replace_text(&mut self, old: &str, new: &str) {
        self.content = self.content.replace(old, new);
    }

    /// Get the current content as a reference.
    pub fn get_content(&self) -> &str {
        &self.content
    }

    /// Clear all content.
    pub fn clear(&mut self) {
        self.content.clear();
    }

    /// Save the current content to a file with the given name.
    pub fn save_to_file(&self, filename: &str) -> Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(self.content.as_bytes())?;
        Ok(())
    }

    /// Load a file's contents into a new TextEditor instance
    pub fn open_file(filename: &str) -> Result<Self> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(Self{ content: contents})
    }

    /// Print the contents of a file to the console
    pub fn read_file(&self,filename: &str) -> Result<Self> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(Self { content: contents })
    }

     /// Return a specific line from a file as a String (1-based indexing)
    pub fn get_line(filename: &str, line_number: usize) -> Result<String> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            if i + 1 == line_number {
                return Ok(line);
            }
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Line {} not found in file '{}'", line_number, filename),
        ))
    }
    
}