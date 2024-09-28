// use std::collections::HashMap;
// use serde_json::Value;
// use std::fs;

// pub trait Chunk {
//     fn content(&self) -> String;
//     fn metadata(&self) -> HashMap<String, String>;
// }
// pub struct IpynbFileChunker {
//     code_chunker: CodeFileChunker,
// }
// pub struct FileChunk {
//     pub file_content: String,
//     pub file_metadata: HashMap<String, String>,
//     pub start_byte: usize,
//     pub end_byte: usize,
// }

// pub trait Chunker {
//     fn chunk(&self, content: &str, metadata: HashMap<String, String>) -> Vec<Box<dyn Chunk>>;
// }

// pub struct CodeFileChunker {
//     pub max_tokens: usize,
// }

// pub struct UniversalFileChunker {
//     pub max_tokens: usize,
//     pub code_chunker: CodeFileChunker,
//     pub text_chunker: TextFileChunker,
// }

// pub struct TextFileChunker {
//     pub max_tokens: usize,
// }



// impl Chunk for FileChunk {
//     fn content(&self) -> String {
//         let content = &self.file_content[self.start_byte..self.end_byte];
//         format!("{}\n\n{}", self.file_metadata.get("file_path").unwrap_or(&"".to_string()), content)
//     }

//     fn metadata(&self) -> HashMap<String, String> {
//         let mut chunk_metadata = HashMap::new();
//         let filename_ascii = self.file_metadata.get("file_path").unwrap_or(&"".to_string())
//             .chars().filter(|c| c.is_ascii()).collect::<String>();

//         chunk_metadata.insert("id".to_string(), format!("{}_{}_{}", filename_ascii, self.start_byte, self.end_byte));
//         chunk_metadata.insert("start_byte".to_string(), self.start_byte.to_string());
//         chunk_metadata.insert("end_byte".to_string(), self.end_byte.to_string());
//         chunk_metadata.insert("length".to_string(), (self.end_byte - self.start_byte).to_string());

//         chunk_metadata.extend(self.file_metadata.clone());
//         chunk_metadata
//     }
// }


// impl CodeFileChunker {
//     fn is_code_file(filename: &str) -> bool {
//         // Implement logic to check if a file is a code file
//         let ext = std::path::Path::new(filename).extension().unwrap_or_default().to_str().unwrap_or_default();
//         match ext {
//             "rs" | "py" | "js" => true,
//             _ => false,
//         }
//     } 
// }
// impl Chunker for CodeFileChunker {
//     fn chunk(&self, content: &str, metadata: HashMap<String, String>) -> Vec<Box<dyn Chunk>> {
//         let mut chunks = Vec::new();

//         let chunk = FileChunk {
//             file_content: content.to_string(),
//             file_metadata: metadata.clone(),
//             start_byte: 0,
//             end_byte: content.len(),
//         };
//         chunks.push(Box::new(chunk) as Box<dyn Chunk>);
//         chunks
//     }
// }




// impl UniversalFileChunker {
//     pub fn new(max_tokens: usize) -> Self {
//         UniversalFileChunker {
//             max_tokens,
//             code_chunker: CodeFileChunker { max_tokens },
//             text_chunker: TextFileChunker { max_tokens },
//         }
//     }
// }

// impl Chunker for UniversalFileChunker {
//     fn chunk(&self, content: &str, metadata: HashMap<String, String>) -> Vec<Box<dyn Chunk>> {
//         let file_path = metadata.get("file_path").unwrap_or(&"".to_string()).to_lowercase();
        
//         if file_path.ends_with(".ipynb") {
//             // Use IPYNB chunker logic if needed
//             return vec![];
//         } else if CodeFileChunker::is_code_file(&file_path) {
//             self.code_chunker.chunk(content, metadata)
//         } else {
//             self.text_chunker.chunk(content, metadata)
//         }
//     }
// }


// impl Chunker for TextFileChunker {
//     fn chunk(&self, content: &str, metadata: HashMap<String, String>) -> Vec<Box<dyn Chunk>> {
//         let mut chunks = Vec::new();
//         let chunk = FileChunk {
//             file_content: content.to_string(),
//             file_metadata: metadata,
//             start_byte: 0,
//             end_byte: content.len(),
//         };
//         chunks.push(Box::new(chunk) as Box<dyn Chunk>);
//         return chunks
//     }
// }




use std::collections::HashMap;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub trait Chunk {
    fn content(&self) -> String;
    fn metadata(&self) -> HashMap<String, String>;
}

pub struct IpynbFileChunker {
    code_chunker: CodeFileChunker,
}

pub struct FileChunk {
    pub file_content: String,
    pub file_metadata: HashMap<String, String>,
    pub start_byte: usize,
    pub end_byte: usize,
}

pub trait Chunker {
    fn chunk(&self, content: &str, metadata: HashMap<String, String>) -> Vec<Box<dyn Chunk>>;
}

pub struct CodeFileChunker {
    pub max_tokens: usize,
}

impl CodeFileChunker {
    pub fn new(max_tokens: usize) -> Self {
        CodeFileChunker { max_tokens }
    }

    fn is_code_file(filename: &str) -> bool {
        // Implement logic to check if a file is a code file
        let ext = Path::new(filename).extension().unwrap_or_default().to_str().unwrap_or_default();
        match ext {
            "rs" | "py" | "js" | "ts" | "tsx" => true,
            _ => false,
        }
    }

    fn parse_tree(&self, filename: &str, content: &str) -> Option<Node> {
        let root_node = Node::new(0, content.len()); // Mock node for simplicity
        Some(root_node)
    }

    fn chunk_node(&self, node: Node, file_content: &str, file_metadata: &HashMap<String, String>) -> Vec<FileChunk> {
        let node_chunk = FileChunk {
            file_content: file_content.to_string(),
            file_metadata: file_metadata.clone(),
            start_byte: node.start_byte,
            end_byte: node.end_byte,
        }; 

        if node_chunk.num_tokens() <= self.max_tokens {
            return vec![node_chunk];
        }

        if node.children.is_empty() {
            return self.split_long_chunk(&node_chunk);
        }

        let mut chunks = Vec::new();
        for child in node.children {
            chunks.extend(self.chunk_node(child, file_content, file_metadata));
        }

        // Merge neighboring chunks if their combined size doesn't exceed max_tokens
        self.merge_chunks(chunks)
    }

    fn split_long_chunk(&self, chunk: &FileChunk) -> Vec<FileChunk> {
        // Implement logic to split a long chunk into smaller chunks based on max_tokens
        vec![chunk.clone()] // Placeholder, implement actual splitting logic here
    }

    fn merge_chunks(&self, chunks: Vec<FileChunk>) -> Vec<FileChunk> {
        let mut merged_chunks = Vec::new();
        
        for chunk in chunks {
            if let Some(last) = merged_chunks.last_mut() {
                if last.num_tokens() + chunk.num_tokens() < self.max_tokens {
                    last.end_byte = chunk.end_byte; // Extend the last chunk
                } else {
                    merged_chunks.push(chunk);
                }
            } else {
                merged_chunks.push(chunk);
            }
        }

        merged_chunks
    }
}

impl Chunker for CodeFileChunker {
    fn chunk(&self, content: &str, metadata: HashMap<String, String>) -> Vec<Box<dyn Chunk>> {
        let mut chunks = Vec::new();
        let file_path = metadata.get("file_path").unwrap_or(&"".to_string());

        if self.is_code_file(file_path) {
            return chunks; // If not a code file, return empty
        }

        let tree = self.parse_tree(file_path, content);
        if tree.is_none() {
            return chunks; // Return empty if parsing fails
        }

        let file_chunks = self.chunk_node(tree.unwrap(), content, &metadata);
        let result: Vec<Box<dyn Chunk>> = file_chunks.into_iter().map(|chunk| Box::new(chunk)).collect();
        result
    }
}

pub struct UniversalFileChunker {
    pub max_tokens: usize,
    pub code_chunker: CodeFileChunker,
    pub text_chunker: TextFileChunker,
}

impl UniversalFileChunker {
    pub fn new(max_tokens: usize) -> Self {
        UniversalFileChunker {
            max_tokens,
            code_chunker: CodeFileChunker::new(max_tokens),
            text_chunker: TextFileChunker { max_tokens },
        }
    }
}

impl Chunker for UniversalFileChunker {
    fn chunk(&self, content: &str, metadata: HashMap<String, String>) -> Vec<Box<dyn Chunk>> {
        let file_path = metadata.get("file_path").unwrap_or(&"".to_string()).to_lowercase();
        
        if file_path.ends_with(".ipynb") {
            // Use IPYNB chunker logic if needed
            return vec![];
        } else if CodeFileChunker::is_code_file(&file_path) {
            self.code_chunker.chunk(content, metadata)
        } else {
            self.text_chunker.chunk(content, metadata)
        }
    }
}

pub struct TextFileChunker {
    pub max_tokens: usize,
}

impl Chunker for TextFileChunker {
    fn chunk(&self, content: &str, metadata: HashMap<String, String>) -> Vec<Box<dyn Chunk>> {
        let mut chunks = Vec::new();
        let chunk = FileChunk {
            file_content: content.to_string(),
            file_metadata: metadata,
            start_byte: 0,
            end_byte: content.len(),
        };
        chunks.push(Box::new(chunk) as Box<dyn Chunk>);
        return chunks;
    }
}

// Additional structs for parse tree nodes
pub struct Node {
    pub start_byte: usize,
    pub end_byte: usize,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(start_byte: usize, end_byte: usize) -> Self {
        Node {
            start_byte,
            end_byte,
            children: vec![], // Populate this with actual parsed children
        }
    }
}

impl FileChunk {
    pub fn num_tokens(&self) -> usize {
        // Placeholder: Implement logic to count tokens in file_content
        self.end_byte - self.start_byte // Adjust this to your actual token counting logic
    }
}

impl Chunk for FileChunk {
    fn content(&self) -> String {
        let content = &self.file_content[self.start_byte..self.end_byte];
        format!("{}\n\n{}", self.file_metadata.get("file_path").unwrap_or(&"".to_string()), content)
    }

    fn metadata(&self) -> HashMap<String, String> {
        let mut chunk_metadata = HashMap::new();
        let filename_ascii = self.file_metadata.get("file_path").unwrap_or(&"".to_string())
            .chars().filter(|c| c.is_ascii()).collect::<String>();

        chunk_metadata.insert("id".to_string(), format!("{}_{}_{}", filename_ascii, self.start_byte, self.end_byte));
        chunk_metadata.insert("start_byte".to_string(), self.start_byte.to_string());
        chunk_metadata.insert("end_byte".to_string(), self.end_byte.to_string());
        chunk_metadata.insert("length".to_string(), (self.end_byte - self.start_byte).to_string());

        chunk_metadata.extend(self.file_metadata.clone());
        chunk_metadata
    }
}