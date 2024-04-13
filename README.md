# Compiler-Lab

 北京大学编译原理课程lab（个人代码）（Rust实现）

### Usage

##### 挂载目录到容器：

```
docker run -it --rm -v D:/MyCodes/Compiler-Lab:/root/compiler maxxing/compiler-dev bash
```

### Lab记录

#### Lv.1 main函数

##### 配置依赖环境

在 `Cargo.toml` 中添加对应依赖，并在项目根目录新建 `build.rs`，参照 [在线文档此处](https://pku-minic.github.io/online-doc/#/lv1-main/lexer-parser?id=rust-%e5%ae%9e%e7%8e%b0) 。注意将lalrpop的版本改成 [crates.io](crates.io) 上面的最新版本，我写lab时最新版本为0.20.2。

##### 运行示例代码

将在线文档里的lalrpop代码写入 `src` 目录中新建的 `sysy.lalrpop` 中，rust代码写入 `main.rs` 中，并把要编译的源代码写入 `hello.c` 。

然后使用

```
cargo run -- -koopa hello.c -o hello.koopa
```

编译运行。

##### 写自己的代码

新建 `ast_def.rs` ，在其中定义AST（抽象语法树）的结构体。

更改 `sysy.lalrpop` ，在其中将返回值改为所定义的结构体。

##### 测试命令：

```
docker run -it --rm -v D:/MyCodes/Compiler-Lab:/root/compiler maxxing/compiler-dev autotest -koopa -s lv1 /root/compiler
```

同时, 在运行测试前, 你需要确保你的编译器 (假设名称为 `compiler`) 能处理如下的命令行参数:

```
compiler -koopa 输入文件 -o 输出文件
```

其中, `-koopa` 代表你的编译器要输出 Koopa IR 文件, `输入文件` 代表输入的 SysY 源文件的路径, `输出文件` 代表 Koopa IR 的输出文件路径. 你的编译器应该解析 `输入文件`, 并把生成的 Koopa IR 输出到 `输出文件` 中.

测试程序会使用你的编译器将输入编译为 Koopa IR, 然后借助 LLVM 将 Koopa IR 进一步编译成可执行文件. 最后, 测试程序执行可执行文件, 检查程序的返回值 (也就是 `main` 的返回值) 是否符合预期. 测试程序**不会**检查你输出的 Koopa IR 的形式, 你输出的 IR **只要功能正确, 即可通过测试.**

关于实验环境/测试脚本的详细使用方法, 请参考[实验环境使用说明](https://pku-minic.github.io/online-doc/#/misc-app-ref/environment). 关于调试编译器的相关思路, 请参考[调试你的编译器](https://pku-minic.github.io/online-doc/#/misc-app-ref/environment?id=%e8%b0%83%e8%af%95%e4%bd%a0%e7%9a%84%e7%bc%96%e8%af%91%e5%99%a8). 关于测试脚本的工作原理, 请 [RTFSC](https://github.com/pku-minic/compiler-dev/blob/master/autotest/autotest).
