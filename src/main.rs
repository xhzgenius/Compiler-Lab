mod ast_def;
mod ir_builder;
mod assembly_builder;

use lalrpop_util::lalrpop_mod;
use koopa::back::KoopaGenerator;

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);

fn main() -> std::io::Result<()> {
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
  let ast = sysy::CompUnitParser::new().parse(&input).expect("Parse error");

  // 输出解析得到的 AST
  // dbg!("AST:\n{:#?}", &ast);

  // Generate in-memory Koopa IR (struct Program) using my IR builder.
  let ir: koopa::ir::Program = ir_builder::generate_ir(&ast).expect("IR builder error");

  match mode.as_str() {
    // Convert in-memory Koopa IR to text, and write it to output file (hello.koopa).
    "-koopa" => {
      let mut text_generator = KoopaGenerator::new(Vec::new());
      text_generator.generate_on(&ir).unwrap();
      std::fs::write(output, text_generator.writer())?;
      Ok(())
    }
    "-riscv" => {
      let output_file = std::fs::File::create(output)?;
      assembly_builder::generate_assembly(&ir, output_file).expect("Assembly builder error");
      // for assembly_code in assembly_codes {
      //   writeln!(output_file, "{}", assembly_code)?;
      // }
      Ok(())
    }
    mode => Err(mode),
  }.expect("Wrong mode");
  Ok(())
}
