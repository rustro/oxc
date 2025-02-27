mod ast_passes;
mod ecmascript;
mod mangler;

use oxc_allocator::Allocator;
use oxc_codegen::{CodeGenerator, CodegenOptions};
use oxc_minifier::{CompressOptions, Compressor};
use oxc_parser::Parser;
use oxc_span::SourceType;

pub(crate) fn test(source_text: &str, expected: &str, options: CompressOptions) {
    let source_type = SourceType::default();
    let result = run(source_text, source_type, Some(options));
    let expected = run(expected, source_type, None);
    assert_eq!(result, expected, "\nfor source\n{source_text}\nexpect\n{expected}\ngot\n{result}");
}

pub(crate) fn run(
    source_text: &str,
    source_type: SourceType,
    options: Option<CompressOptions>,
) -> String {
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, source_text, source_type).parse();
    let mut program = ret.program;
    if let Some(options) = options {
        Compressor::new(&allocator, options).build(&mut program);
    }
    CodeGenerator::new()
        .with_options(CodegenOptions { single_quote: true, ..CodegenOptions::default() })
        .build(&program)
        .code
}
