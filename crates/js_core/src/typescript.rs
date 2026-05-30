use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_transformer::{TransformOptions, Transformer, TypeScriptOptions};
use std::path::Path;

pub fn strip_types_fast(ts_code: &str, ts_options: TypeScriptOptions) -> Result<String, String> {
    let allocator = Allocator::default();

    let source_type = SourceType::ts();

    let parser = Parser::new(&allocator, ts_code, source_type);
    let parsed = parser.parse();

    if !parsed.errors.is_empty() {
        return Err(format!("TS Syntax Errors: {:?}", parsed.errors));
    }

    let mut program = parsed.program;

    let semantic_return = SemanticBuilder::new().build(&program);
    let scoping = semantic_return.semantic.into_scoping();

    let options = TransformOptions {
        typescript: ts_options,
        ..Default::default()
    };

    let dummy_path = Path::new("mod.ts");
    Transformer::new(&allocator, dummy_path, &options).build_with_scoping(scoping, &mut program);

    let codegen_return = Codegen::new().build(&program);

    Ok(codegen_return.code)
}

pub fn strip_types_fast_default(ts_code: &str) -> Result<String, String> {
    strip_types_fast(ts_code, Default::default())
}
