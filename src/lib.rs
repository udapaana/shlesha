pub mod ir;
pub mod schema_parser;
pub mod parser;
pub mod transformer;
pub mod generator;
pub mod transliterator;

// New V2 modules with typed IR
pub mod element_id;
pub mod ir_v2;
pub mod parser_v2;
pub mod transformer_v2;
pub mod generator_v2;
pub mod runtime_extension;

pub use ir::{
    AbugidaIR, AlphabetIR, IR, Element, ElementType, PropertyValue,
    Extension, ExtensionMapping, Metadata
};
pub use schema_parser::{
    Schema, SchemaParser, SchemaRegistry, SchemaError,
    ScriptType, ElementMapping, ExtensionDefinition, ExtensionFile
};
pub use parser::{Parser, ParserBuilder, ParseError};
pub use transformer::{Transformer, TransformerBuilder, TransformError};
pub use generator::{Generator, GeneratorBuilder, GenerateError};
pub use transliterator::{Transliterator, TransliteratorBuilder, TransliteratorError};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}