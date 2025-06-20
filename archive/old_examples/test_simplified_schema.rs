use std::fs;
use shlesha::simplified_schema::SimplifiedSchemaParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Simplified Schema Parser");
    
    // Read the simplified Devanagari schema
    let simplified_content = fs::read_to_string("schemas/devanagari_simplified.yaml")?;
    println!("📄 Loaded simplified schema ({} characters)", simplified_content.len());
    
    // Parse using simplified parser
    let start = std::time::Instant::now();
    let schema = SimplifiedSchemaParser::parse_str(&simplified_content)?;
    let parse_time = start.elapsed();
    
    println!("✅ Parsed simplified schema in {:?}", parse_time);
    println!("📊 Schema stats:");
    println!("   Name: {}", schema.name);
    println!("   Type: {:?}", schema.script_type);
    println!("   Mapping categories: {}", schema.mappings.len());
    
    // Count total mappings
    let mut total_mappings = 0;
    for (category, mappings) in &schema.mappings {
        println!("   {}: {} mappings", category, mappings.len());
        total_mappings += mappings.len();
    }
    println!("   Total mappings: {}", total_mappings);
    
    // Test property inference
    if let Some(consonants) = schema.mappings.get("consonants") {
        println!("\n🔍 Testing property inference:");
        
        // Check aspiration inference
        if let Some(ka_mapping) = consonants.get("क") {
            println!("   क -> {} (aspirated: {:?})", 
                ka_mapping.canonical, 
                ka_mapping.properties.get("aspirated").unwrap_or(&serde_yaml::Value::Bool(false))
            );
        }
        
        if let Some(kha_mapping) = consonants.get("ख") {
            println!("   ख -> {} (aspirated: {:?})", 
                kha_mapping.canonical, 
                kha_mapping.properties.get("aspirated").unwrap_or(&serde_yaml::Value::Bool(false))
            );
        }
        
        // Check voice inference
        if let Some(ga_mapping) = consonants.get("ग") {
            println!("   ग -> {} (voiced: {:?})", 
                ga_mapping.canonical, 
                ga_mapping.properties.get("voiced").unwrap_or(&serde_yaml::Value::Bool(false))
            );
        }
    }
    
    // Compare with verbose schema
    let verbose_content = fs::read_to_string("schemas/devanagari.yaml")?;
    println!("\n📏 Size comparison:");
    println!("   Simplified: {} characters", simplified_content.len());
    println!("   Verbose: {} characters", verbose_content.len()); 
    println!("   Compression ratio: {:.1}x smaller", 
        verbose_content.len() as f64 / simplified_content.len() as f64);
    
    println!("\n🎉 Simplified schema test completed successfully!");
    
    Ok(())
}