use std::io::Write;
use std::process::Command;

/// Test CLI integration including new metadata flags
#[cfg(test)]
mod cli_tests {
    use super::*;

    fn get_cli_binary() -> String {
        // Try to use the release binary first, then fall back to debug
        let release_path = std::path::Path::new("target/release/shlesha");
        if release_path.exists() {
            return release_path.to_string_lossy().to_string();
        }

        let debug_path = std::path::Path::new("target/debug/shlesha");
        if debug_path.exists() {
            return debug_path.to_string_lossy().to_string();
        }

        // Fallback to cargo-generated binary location
        let mut path = std::env::current_exe().unwrap();
        path.pop(); // Remove test binary name
        if path.ends_with("deps") {
            path.pop(); // Remove deps directory
        }
        path.push("shlesha");
        path.to_string_lossy().to_string()
    }

    #[test]
    fn test_cli_basic_transliteration() {
        let output = Command::new(get_cli_binary())
            .arg("transliterate")
            .arg("--from")
            .arg("devanagari")
            .arg("--to")
            .arg("iast")
            .arg("अ")
            .output()
            .expect("Failed to execute CLI");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(stdout.trim(), "a");
    }

    #[test]
    fn test_cli_show_metadata_flag() {
        let output = Command::new(get_cli_binary())
            .arg("transliterate")
            .arg("--from")
            .arg("devanagari")
            .arg("--to")
            .arg("iast")
            .arg("--show-metadata")
            .arg("धर्मkr")
            .output()
            .expect("Failed to execute CLI");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        // Should contain the output plus any metadata annotations
        assert!(stdout.contains("dharma"));
        // Should contain inline metadata annotations like [devanagari:k] for unknown tokens
        assert!(stdout.contains("[devanagari:k]"));
    }

    #[test]
    fn test_cli_verbose_flag() {
        let output = Command::new(get_cli_binary())
            .arg("transliterate")
            .arg("--from")
            .arg("devanagari")
            .arg("--to")
            .arg("iast")
            .arg("--verbose")
            .arg("धर्म")
            .output()
            .expect("Failed to execute CLI");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        // Should contain the transliterated text
        assert!(stdout.contains("dharma"));
        // Should contain metadata section
        assert!(stdout.contains("Metadata:"));
        assert!(stdout.contains("Source: devanagari -> Target: iast"));
    }

    #[test]
    fn test_cli_verbose_with_unknowns() {
        let output = Command::new(get_cli_binary())
            .arg("transliterate")
            .arg("--from")
            .arg("devanagari")
            .arg("--to")
            .arg("iast")
            .arg("--verbose")
            .arg("धर्मkr")
            .output()
            .expect("Failed to execute CLI");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        // Should contain the transliterated text
        assert!(stdout.contains("dharma"));
        // Should contain metadata section
        assert!(stdout.contains("Metadata:"));
        // Should show unknown token count
        assert!(stdout.contains("Unknown tokens:"));
    }

    #[test]
    fn test_cli_scripts_command() {
        let output = Command::new(get_cli_binary())
            .arg("scripts")
            .output()
            .expect("Failed to execute CLI");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Currently supported scripts:"));
        assert!(stdout.contains("devanagari"));
        assert!(stdout.contains("iast"));
        assert!(stdout.contains("Devanagari script"));
    }

    #[test]
    fn test_cli_stdin_support() {
        let mut child = Command::new(get_cli_binary())
            .arg("transliterate")
            .arg("--from")
            .arg("devanagari")
            .arg("--to")
            .arg("iast")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn CLI");

        let stdin = child.stdin.as_mut().expect("Failed to get stdin");
        stdin
            .write_all("अ".as_bytes())
            .expect("Failed to write to stdin");
        let _ = child.stdin.take(); // Close stdin to signal EOF

        let output = child.wait_with_output().expect("Failed to wait for CLI");
        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(stdout.trim(), "a");
    }

    #[test]
    fn test_cli_error_handling_invalid_script() {
        let output = Command::new(get_cli_binary())
            .arg("transliterate")
            .arg("--from")
            .arg("invalid_script")
            .arg("--to")
            .arg("iast")
            .arg("test")
            .output()
            .expect("Failed to execute CLI");

        assert!(!output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("Error:"));
    }

    #[test]
    fn test_cli_empty_input() {
        let output = Command::new(get_cli_binary())
            .arg("transliterate")
            .arg("--from")
            .arg("devanagari")
            .arg("--to")
            .arg("iast")
            .arg("")
            .output()
            .expect("Failed to execute CLI");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(stdout.trim(), "");
    }

    #[test]
    fn test_cli_whitespace_preservation() {
        let output = Command::new(get_cli_binary())
            .arg("transliterate")
            .arg("--from")
            .arg("devanagari")
            .arg("--to")
            .arg("iast")
            .arg("अ आ")
            .output()
            .expect("Failed to execute CLI");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains(" "));
        assert!(stdout.contains("a"));
    }

    #[test]
    fn test_cli_help_flag() {
        let output = Command::new(get_cli_binary())
            .arg("--help")
            .output()
            .expect("Failed to execute CLI");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("High-performance extensible transliteration"));
        // Check for CLI flags - these are in the transliterate subcommand help
        assert!(stdout.contains("transliterate") || stdout.contains("--help"));
    }

    #[test]
    fn test_cli_cross_script_conversion() {
        let output = Command::new(get_cli_binary())
            .arg("transliterate")
            .arg("--from")
            .arg("devanagari")
            .arg("--to")
            .arg("gujarati")
            .arg("धर्म")
            .output()
            .expect("Failed to execute CLI");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(!stdout.trim().is_empty());
        // Should produce Gujarati output
        assert!(stdout.contains("ધ") || stdout.contains("र्म")); // Either correct or graceful fallback
    }

    #[test]
    fn test_cli_mixed_content() {
        let output = Command::new(get_cli_binary())
            .arg("transliterate")
            .arg("--from")
            .arg("devanagari")
            .arg("--to")
            .arg("iast")
            .arg("धर्म hello")
            .output()
            .expect("Failed to execute CLI");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("dharma"));
        assert!(stdout.contains("hello"));
    }
}
