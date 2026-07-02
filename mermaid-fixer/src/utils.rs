/// Extract mermaid code blocks from markdown content.
/// Returns a list of tuples: (start position, end position, code content).
pub fn extract_mermaid_blocks(content: &str) -> Vec<(usize, usize, String)> {
    let mut blocks = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();
        
        // Find mermaid code block start marker
        if line == "```mermaid" {
            let start_line = i;
            i += 1;
            let mut mermaid_lines = Vec::new();
            
            // Collect mermaid code until end marker
            while i < lines.len() {
                let current_line = lines[i];
                if current_line.trim() == "```" {
                    // Found end marker
                    let end_line = i;
                    let mermaid_code = mermaid_lines.join("\n");
                    
                    // Calculate position in original content
                    let start_pos = lines[..start_line].iter().map(|l| l.len() + 1).sum::<usize>();
                    let end_pos = lines[..=end_line].iter().map(|l| l.len() + 1).sum::<usize>();
                    
                    blocks.push((start_pos, end_pos, mermaid_code));
                    break;
                }
                mermaid_lines.push(current_line);
                i += 1;
            }
        }
        i += 1;
    }

    blocks
}

/// Print processing statistics
pub fn print_statistics(result: &crate::processor::ProcessResult, dry_run: bool) {
    println!("\n📊 Processing complete:");
    println!("   📄 Files processed: {}", result.total_files);
    println!("   📊 Total mermaid code blocks: {}", result.total_mermaid_blocks);
    println!("   ❌ Invalid code blocks: {}", result.invalid_blocks);
    if !dry_run {
        println!("   🔧 Successfully fixed: {}", result.fixed_blocks);
    }
}
