#!/usr/bin/env python3
"""
Comprehensive script testing for all supported schemas in Shlesha
Tests bidirectional conversion accuracy across all script families
"""

import json
import time
import subprocess
from pathlib import Path
from typing import Dict, List, Tuple

# Test data for each script family
SCRIPT_TEST_DATA = {
    # Brahmic scripts - all should handle Sanskrit
    "brahmic": {
        "devanagari": {
            "sample": "अग्निमीळे पुरोहितं",
            "expected_tokens": ["A", "GA", "NA", "I_MATRA", "MA", "II_MATRA", "LLA", "E_MATRA", "SPACE", "PA", "U_MATRA", "RA", "O_MATRA", "HA", "I_MATRA", "TA", "ANUSVARA"],
            "languages": ["Sanskrit", "Hindi", "Marathi"]
        },
        "bengali": {
            "sample": "অগ্নিমীলে পুরোহিত",
            "expected_tokens": ["A", "GA", "VIRAMA", "NA", "I_MATRA", "MA", "II_MATRA", "LA", "E_MATRA", "SPACE", "PA", "U_MATRA", "RA", "O_MATRA", "HA", "I_MATRA", "TA"],
            "languages": ["Bengali", "Assamese", "Sanskrit"]
        },
        "gujarati": {
            "sample": "અગ્નિમીળે પુરોહિત",
            "expected_tokens": ["A", "GA", "VIRAMA", "NA", "I_MATRA", "MA", "II_MATRA", "LLA", "E_MATRA", "SPACE", "PA", "U_MATRA", "RA", "O_MATRA", "HA", "I_MATRA", "TA"],
            "languages": ["Gujarati", "Sanskrit"]
        },
        "gurmukhi": {
            "sample": "ਸਤਿ ਨਾਮੁ ਕਰਤਾ ਪੁਰਖੁ",
            "expected_tokens": ["SA", "TA", "I_MATRA", "SPACE", "NA", "AA_MATRA", "MA", "U_MATRA", "SPACE", "KA", "RA", "TA", "AA_MATRA", "SPACE", "PA", "U_MATRA", "RA", "KHA", "U_MATRA"],
            "languages": ["Punjabi"]
        },
        "kannada": {
            "sample": "ಅಗ್ನಿಮೀಳೆ ಪುರೋಹಿತ",
            "expected_tokens": ["A", "GA", "VIRAMA", "NA", "I_MATRA", "MA", "II_MATRA", "LLA", "E_MATRA", "SPACE", "PA", "U_MATRA", "RA", "O_MATRA", "HA", "I_MATRA", "TA"],
            "languages": ["Kannada", "Sanskrit"]
        },
        "telugu": {
            "sample": "అగ్నిమీళే పురోహిత",
            "expected_tokens": ["A", "GA", "VIRAMA", "NA", "I_MATRA", "MA", "II_MATRA", "LLA", "E_MATRA", "SPACE", "PA", "U_MATRA", "RA", "O_MATRA", "HA", "I_MATRA", "TA"],
            "languages": ["Telugu", "Sanskrit"]
        }
    },
    
    # Romanization systems - all for Sanskrit
    "romanization": {
        "iast": {
            "sample": "agnimīḷe purohitaṃ",
            "expected_tokens": ["A", "GA", "NA", "I", "MA", "II", "LLA", "E", "SPACE", "PA", "U", "RA", "O", "HA", "I", "TA", "ANUSVARA"],
            "languages": ["Sanskrit (IAST)"]
        },
        "harvard_kyoto": {
            "sample": "agnimILe purohitaM",
            "expected_tokens": ["A", "GA", "NA", "I", "MA", "II", "LLA", "E", "SPACE", "PA", "U", "RA", "O", "HA", "I", "TA", "ANUSVARA"],
            "languages": ["Sanskrit (HK)"]
        },
        "iso15919": {
            "sample": "agnimīl̥e pur'ōhitaṁ",
            "expected_tokens": ["A", "GA", "NA", "I", "MA", "II", "LLA", "E", "SPACE", "PA", "U", "RA", "O", "HA", "I", "TA", "ANUSVARA"],
            "languages": ["Sanskrit (ISO)"]
        },
        "itrans": {
            "sample": "agnimILe purohitaM",
            "expected_tokens": ["A", "GA", "NA", "I", "MA", "II", "LLA", "E", "SPACE", "PA", "U", "RA", "O", "HA", "I", "TA", "ANUSVARA"],
            "languages": ["Sanskrit (ITRANS)"]
        },
        "slp1": {
            "sample": "agnimILe purohitaM",
            "expected_tokens": ["A", "GA", "NA", "I", "MA", "II", "LLA", "E", "SPACE", "PA", "U", "RA", "O", "HA", "I", "TA", "ANUSVARA"],
            "languages": ["Sanskrit (SLP1)"]
        },
        "velthuis": {
            "sample": "agnimiile purohita.m",
            "expected_tokens": ["A", "GA", "NA", "I", "MA", "II", "LLA", "E", "SPACE", "PA", "U", "RA", "O", "HA", "I", "TA", "ANUSVARA"],
            "languages": ["Sanskrit (Velthuis)"]
        },
        "wx": {
            "sample": "agnimILe purohitaM",
            "expected_tokens": ["A", "GA", "NA", "I", "MA", "II", "LLA", "E", "SPACE", "PA", "U", "RA", "O", "HA", "I", "TA", "ANUSVARA"],
            "languages": ["Sanskrit (WX)"]
        }
    },
    
    # Other scripts
    "other": {
        "arabic": {
            "sample": "السلام عليكم",
            "expected_tokens": ["ALIF", "LAM", "SUKUN", "ALIF", "LAM", "FATHA", "MIM", "SPACE", "AYN", "LAM", "YA", "KAF", "MIM"],
            "languages": ["Arabic", "Urdu", "Persian"]
        },
        "cyrillic": {
            "sample": "Здравствуйте",
            "expected_tokens": ["ZE", "DE", "ER", "A", "VE", "ES", "TE", "VE", "U", "I_KRATKOYE", "TE", "E"],
            "languages": ["Russian", "Bulgarian", "Serbian"]
        }
    }
}

class ScriptTester:
    def __init__(self, shlesha_path: str = "../shlesha"):
        self.shlesha_path = Path(shlesha_path)
        self.results = {}
        
    def test_schema_loading(self, family: str, script: str) -> Dict:
        """Test if a schema can be loaded correctly"""
        # The schemas are in shlesha/schemas/
        schema_path = self.shlesha_path / "schemas" / family / f"{script}.toml"
        
        result = {
            "schema_exists": schema_path.exists(),
            "schema_valid": False,
            "error": None
        }
        
        if result["schema_exists"]:
            try:
                # Try to parse the TOML file
                try:
                    import tomllib  # Python 3.11+
                    with open(schema_path, 'rb') as f:
                        schema_data = tomllib.load(f)
                except ImportError:
                    try:
                        import toml
                        with open(schema_path, 'r', encoding='utf-8') as f:
                            schema_data = toml.load(f)
                    except ImportError:
                        # Simple TOML parser for basic validation
                        schema_data = {}
                        with open(schema_path, 'r', encoding='utf-8') as f:
                            current_section = None
                            for line in f:
                                line = line.strip()
                                if line.startswith('[') and line.endswith(']'):
                                    current_section = line[1:-1]
                                    schema_data[current_section] = {}
                                elif '=' in line and current_section:
                                    key, value = line.split('=', 1)
                                    schema_data[current_section][key.strip()] = value.strip()
                
                # Validate required sections
                required_sections = ["metadata", "vowels", "consonants"]
                missing_sections = [s for s in required_sections if s not in schema_data]
                
                if missing_sections:
                    result["error"] = f"Missing sections: {missing_sections}"
                else:
                    result["schema_valid"] = True
                    result["metadata"] = schema_data.get("metadata", {})
                    
            except Exception as e:
                result["error"] = str(e)
        
        return result
    
    def test_tokenization(self, family: str, script: str, test_data: Dict) -> Dict:
        """Test tokenization of sample text"""
        # This would require the actual Rust implementation
        # For now, we'll simulate the test structure
        
        result = {
            "input_text": test_data["sample"],
            "tokenization_success": False,
            "token_count": 0,
            "expected_tokens": len(test_data["expected_tokens"]),
            "error": None
        }
        
        # TODO: Call actual Shlesha tokenizer when implemented
        # For now, we'll mark as placeholder
        result["error"] = "Tokenizer not yet implemented"
        
        return result
    
    def test_bidirectional_conversion(self, source_script: str, target_script: str, text: str) -> Dict:
        """Test round-trip conversion accuracy"""
        result = {
            "source_script": source_script,
            "target_script": target_script,
            "original_text": text,
            "forward_conversion": None,
            "backward_conversion": None,
            "round_trip_accuracy": 0.0,
            "error": None
        }
        
        # TODO: Implement actual conversion testing
        result["error"] = "Conversion not yet implemented"
        
        return result
    
    def run_all_tests(self) -> Dict:
        """Run comprehensive tests on all scripts"""
        print("Running comprehensive script tests...")
        
        all_results = {
            "timestamp": time.time(),
            "total_scripts": 0,
            "successful_schemas": 0,
            "script_families": {}
        }
        
        for family, scripts in SCRIPT_TEST_DATA.items():
            print(f"\nTesting {family} scripts...")
            family_results = {}
            
            for script, test_data in scripts.items():
                print(f"  Testing {script}...")
                
                # Test schema loading
                schema_result = self.test_schema_loading(family, script)
                
                # Test tokenization
                tokenization_result = self.test_tokenization(family, script, test_data)
                
                script_results = {
                    "schema": schema_result,
                    "tokenization": tokenization_result,
                    "test_data": test_data,
                    "overall_success": schema_result["schema_valid"] and tokenization_result["tokenization_success"]
                }
                
                family_results[script] = script_results
                all_results["total_scripts"] += 1
                
                if schema_result["schema_valid"]:
                    all_results["successful_schemas"] += 1
            
            all_results["script_families"][family] = family_results
        
        return all_results
    
    def generate_report(self, results: Dict) -> str:
        """Generate a comprehensive test report"""
        report = []
        report.append("# Shlesha Script Testing Report\n")
        report.append(f"Generated: {time.ctime(results['timestamp'])}\n")
        report.append(f"Total Scripts Tested: {results['total_scripts']}")
        report.append(f"Successful Schema Loads: {results['successful_schemas']}")
        report.append(f"Schema Success Rate: {results['successful_schemas']/results['total_scripts']*100:.1f}%\n")
        
        for family, scripts in results["script_families"].items():
            report.append(f"## {family.title()} Scripts\n")
            
            for script, result in scripts.items():
                report.append(f"### {script}")
                report.append(f"- **Schema Valid**: {'✅' if result['schema']['schema_valid'] else '❌'}")
                
                if result['schema']['schema_valid']:
                    metadata = result['schema'].get('metadata', {})
                    report.append(f"- **Name**: {metadata.get('name', 'N/A')}")
                    report.append(f"- **ISO 15924**: {metadata.get('iso15924', 'N/A')}")
                    report.append(f"- **Languages**: {', '.join(result['test_data']['languages'])}")
                else:
                    report.append(f"- **Error**: {result['schema']['error']}")
                
                report.append(f"- **Sample Text**: `{result['test_data']['sample']}`")
                report.append("")
        
        return "\n".join(report)

def main():
    tester = ScriptTester()
    
    print("🧪 Starting comprehensive script testing for Shlesha...")
    results = tester.run_all_tests()
    
    # Generate and save report
    report = tester.generate_report(results)
    
    # Save results
    results_dir = Path("../results")
    results_dir.mkdir(exist_ok=True)
    
    timestamp = time.strftime("%Y%m%d_%H%M%S")
    
    with open(results_dir / f"script_test_{timestamp}.json", 'w') as f:
        json.dump(results, f, indent=2)
    
    with open(results_dir / f"script_report_{timestamp}.md", 'w') as f:
        f.write(report)
    
    print(f"\n📊 Results saved to benchmarks/results/")
    print(f"📈 Schema success rate: {results['successful_schemas']}/{results['total_scripts']} ({results['successful_schemas']/results['total_scripts']*100:.1f}%)")

if __name__ == "__main__":
    main()