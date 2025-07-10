use shlesha::modules::script_converter::ScriptConverterRegistry;

fn main() {
    let registry = ScriptConverterRegistry::new_with_all_converters();
    let scripts = registry.list_supported_scripts();
    
    println!("Available scripts:");
    for script in &scripts {
        println!("  - {}", script);
    }
    println!("Total: {} scripts", scripts.len());
    
    // Test specific scripts that tests are looking for
    println!("\nScript availability:");
    println!("  devanagari: {}", registry.supports_script("devanagari"));
    println!("  iso: {}", registry.supports_script("iso"));
    println!("  iso15919: {}", registry.supports_script("iso15919"));
    println!("  iast: {}", registry.supports_script("iast"));
}