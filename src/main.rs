// pub mod GithubManager;
// use crate::GithubManager::DataManager;
// use crate::GithubManager::GitHubRepoManager;
// use std::error::Error;

// fn main() -> Result<(), Box<dyn Error>> {
//     // Initialize logger (for logging purposes)
//     env_logger::init();

//     // Create an instance of GitHubRepoManager
//     let repo_manager = GitHubRepoManager::new(
//         "Saksham1387/AiGrind", // Example repo
//         None,                  // No specific commit hash, will pull latest
//         Some("/Users/sakshamchaudhary/Documents/repo2vec-rust/temp"),
//     )?;

//     // Try downloading (cloning) the repository
//     match repo_manager.download() {
//         Ok(_) => {
//             // info!("Repository successfully cloned!");

//             // Walk through the repository files (handle potential errors)
//             if let Err(e) = repo_manager.walk() {
//                 eprintln!("Failed to walk through repository files. Error: {:?}", e);
//             }
//         }
//         Err(e) => {
//             // Print detailed error message when cloning fails
//             eprintln!("Failed to download the repository. Reason: {}", e);
//         }
//     }

//     Ok(())
// }




use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
mod Chunker;
use Chunker::{Chunker as chunker, UniversalFileChunker}; 

fn main() -> io::Result<()> {
    let file_content = fs::read_to_string("src/large_file.rs").expect("Could not read file");

    let mut file_metadata = HashMap::new();
    file_metadata.insert("file_path".to_string(), "src/large_file.rs".to_string());

    let max_tokens = 100; 
    let chunker = UniversalFileChunker::new(max_tokens);

    let chunks = chunker.chunk(&file_content, file_metadata);

    for (i, chunk) in chunks.iter().enumerate() {
        println!("Chunk {}: {}", i + 1, chunk.content());
        println!("Metadata: {:?}", chunk.metadata());
    }

    Ok(())
}