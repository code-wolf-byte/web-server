use std::path::Path;
struct file{
    name: String,
    path: String,
    size: u64,
    metadata: Metadata,
}

struct Metadata{
    created_at: u64,
    modified_at: u64,
    accessed_at: u64,
}

impl file{
    fn new(file: &str) -> Result<Self, std::io::Error> {
        let path = Path::new(file);
        let metadata = std::fs::metadata(path)?;
        let size = metadata.len();
        let created_at = metadata.created()?.duration_since(std::time::UNIX_EPOCH)?.as_secs();
        let modified_at = metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs();
        let accessed_at = metadata.accessed()?.duration_since(std::time::UNIX_EPOCH)?.as_secs();

        Ok(Self {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            path: path.to_string_lossy().to_string(),
            size,
            metadata: Metadata {
                created_at,
                modified_at,
                accessed_at,
            },
        })

    }

    

}