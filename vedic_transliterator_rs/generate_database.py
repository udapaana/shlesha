#\!/usr/bin/env python3
"""
Generate SQLite database from schema-driven transliteration outputs
"""

import sqlite3
import json
import os
from pathlib import Path
from datetime import datetime

def create_schema_driven_database():
    """Create SQLite database with schema-driven transliteration results"""
    
    # Database path
    db_path = "/Users/skmnktl/Projects/udapaana/data/output/database/vedic_corpus_schema_driven.db"
    
    # Ensure directory exists
    os.makedirs(os.path.dirname(db_path), exist_ok=True)
    
    # Create connection
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    # Create tables
    cursor.execute("""
    CREATE TABLE IF NOT EXISTS sources (
        source_id TEXT PRIMARY KEY,
        veda TEXT NOT NULL,
        shakha TEXT NOT NULL,
        source_type TEXT NOT NULL,  -- vedanidhi, vedavms
        text_type TEXT,             -- samhita, brahmana, aranyaka
        total_texts INTEGER,
        accent_percentage REAL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )
    """)
    
    cursor.execute("""
    CREATE TABLE IF NOT EXISTS texts (
        text_id INTEGER PRIMARY KEY AUTOINCREMENT,
        source_id TEXT NOT NULL,
        vaakya_pk INTEGER,
        vaakya_sk TEXT,
        location_path TEXT,         -- JSON array of location hierarchy
        source_text TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (source_id) REFERENCES sources (source_id)
    )
    """)
    
    cursor.execute("""
    CREATE TABLE IF NOT EXISTS transliterations (
        transliteration_id INTEGER PRIMARY KEY AUTOINCREMENT,
        text_id INTEGER NOT NULL,
        target_scheme TEXT NOT NULL,    -- slp1, iast, devanagari, telugu, iso15919
        transliterated_text TEXT NOT NULL,
        confidence_score REAL,
        unknown_tokens_count INTEGER,
        processing_time_ms INTEGER,
        schema_used TEXT,               -- Source-specific schema used
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (text_id) REFERENCES texts (text_id)
    )
    """)
    
    cursor.execute("""
    CREATE TABLE IF NOT EXISTS schema_metadata (
        schema_name TEXT PRIMARY KEY,
        source_type TEXT NOT NULL,
        veda TEXT,
        shakha TEXT,
        target_script TEXT,
        accent_percentage REAL,
        notes TEXT,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )
    """)
    
    # Create indexes
    cursor.execute("CREATE INDEX IF NOT EXISTS idx_texts_source ON texts(source_id)")
    cursor.execute("CREATE INDEX IF NOT EXISTS idx_transliterations_text ON transliterations(text_id)")
    cursor.execute("CREATE INDEX IF NOT EXISTS idx_transliterations_scheme ON transliterations(target_scheme)")
    cursor.execute("CREATE INDEX IF NOT EXISTS idx_sources_veda ON sources(veda)")
    
    # Process transliterated files
    output_dir = Path("/Users/skmnktl/Projects/udapaana/data/output/transliterated")
    
    if output_dir.exists():
        for source_file in output_dir.glob("**/*.json"):
            process_transliterated_file(cursor, source_file)
    else:
        print(f"⚠️  Output directory not found: {output_dir}")
    
    # Commit and close
    conn.commit()
    conn.close()
    
    print(f"✅ Database created: {db_path}")
    print(f"📊 Run: sqlite3 {db_path} '.tables' to see schema")

def process_transliterated_file(cursor, json_file):
    """Process a single transliterated JSON file"""
    
    try:
        with open(json_file, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        # Extract metadata
        metadata = data.get('metadata', {})
        source_metadata = metadata.get('source_metadata', {})
        transliteration_metadata = metadata.get('transliteration_metadata', {})
        
        # Extract source information
        source_id = source_metadata.get('source_id', 'unknown')
        veda = source_metadata.get('veda', 'unknown')
        shakha = source_metadata.get('shakha', 'unknown')
        source_type = source_metadata.get('source', 'unknown')
        text_type = source_metadata.get('text_type', 'unknown')
        total_texts = source_metadata.get('total_texts', 0)
        accent_percentage = source_metadata.get('accent_percentage_sample', 0.0)
        
        target_scheme = transliteration_metadata.get('target_scheme', 'unknown')
        config_used = transliteration_metadata.get('config_used', 'unknown')
        
        # Insert or update source record
        cursor.execute("""
        INSERT OR REPLACE INTO sources (source_id, veda, shakha, source_type, text_type, total_texts, accent_percentage)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        """, (source_id, veda, shakha, source_type, text_type, total_texts, accent_percentage))
        
        # Process texts
        texts = data.get('texts', [])
        for text_entry in texts:
            # Insert text record
            cursor.execute("""
            INSERT INTO texts (source_id, vaakya_pk, vaakya_sk, location_path, source_text)
            VALUES (?, ?, ?, ?, ?)
            """, (
                source_id,
                text_entry.get('vaakya_pk'),
                text_entry.get('vaakya_sk'),
                json.dumps(text_entry.get('location', [])),
                text_entry.get('source_text', '')
            ))
            
            text_id = cursor.lastrowid
            
            # Insert transliteration record
            cursor.execute("""
            INSERT INTO transliterations (
                text_id, target_scheme, transliterated_text, 
                confidence_score, unknown_tokens_count, processing_time_ms, schema_used
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            """, (
                text_id,
                target_scheme,
                text_entry.get('target_text', ''),
                text_entry.get('confidence_score', 0.0),
                text_entry.get('unknown_tokens_count', 0),
                text_entry.get('processing_time_ms', 0),
                config_used
            ))
        
        print(f"  ✓ Processed: {json_file.name} ({len(texts)} texts)")
        
    except Exception as e:
        print(f"  ✗ Error processing {json_file}: {e}")

if __name__ == "__main__":
    create_schema_driven_database()
EOF < /dev/null