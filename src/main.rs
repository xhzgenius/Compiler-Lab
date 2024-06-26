mod assembly_builder;
mod ast_def;
mod ir_builder;

use koopa::back::KoopaGenerator;
use lalrpop_util::lalrpop_mod;

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compiler configuration:
    // Set the pointer size in our compiled code to 4. (32 bits)
    koopa::ir::Type::set_ptr_size(4);

    // 解析命令行参数
    let mut args = std::env::args();
    args.next();
    let mode = args.next().unwrap();
    let input = args.next().unwrap();
    args.next();
    let output = args.next().unwrap();

    // 读取输入文件
    let input = std::fs::read_to_string(input)?;

    // 调用 lalrpop 生成的 parser 解析输入文件
    let ast = sysy::CompUnitParser::new()
        .parse(&input)
        .expect("Parse error");

    // 输出解析得到的 AST
    // dbg!("AST:\n{:#?}", &ast);

    // Generate in-memory Koopa IR (struct Program) using my IR builder.
    let ir: koopa::ir::Program = ir_builder::generate_ir(&ast)?;

    match mode.as_str() {
        // Convert in-memory Koopa IR to text, and write it to output file (hello.koopa).
        "-koopa" => {
            let mut text_generator = KoopaGenerator::new(Vec::new());
            text_generator.generate_on(&ir).unwrap();
            std::fs::write(output, text_generator.writer())?;
            Ok(())
        }
        "-riscv" | "-perf" => {
            let mut output_file = std::fs::File::create(output)?;
            assembly_builder::generate_assembly(&ir, &mut output_file)?;
            // for assembly_code in assembly_codes {
            //   writeln!(output_file, "{}", assembly_code)?;
            // }
            Ok(())
        }
        mode => Err(mode),
    }?;
    Ok(())
}
