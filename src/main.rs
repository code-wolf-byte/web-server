mod config;

fn main() {
    // Compile-time checks for the operating system
    #[cfg(target_os = "windows")]
    {
        println!("Running on Windows");
        windows_specific_function();
    }

    #[cfg(target_os = "linux")]
    {
        println!("Running on macOS");
        linux_specific_function();
    }
}

// Function specific to Windows
#[cfg(target_os = "windows")]
fn windows_specific_function() {
    println!("Executing Windows-specific code...");
}

// Function specific to macOS
#[cfg(target_os = "linux")]
fn linux_specific_function() {
    println!("Executing macOS-specific code...");
}
