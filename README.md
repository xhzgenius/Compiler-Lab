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

创建一个子模块 `ir_builder` ，负责生成IR，在 `mod.rs` 里定义解析IR的函数入口，新建一个 `build.rs` ，在里面为 `ast_def.rs` 里定义的结构体实现生成IR的方法。

##### 测试命令：

```
docker run -it --rm -v D:/MyCodes/Compiler-Lab:/root/compiler maxxing/compiler-dev autotest -koopa -s lv1 /root/compiler
```

> 同时, 在运行测试前, 你需要确保你的编译器 (假设名称为 `compiler`) 能处理如下的命令行参数:

```
compiler -koopa 输入文件 -o 输出文件
```

> 其中, `-koopa` 代表你的编译器要输出 Koopa IR 文件, `输入文件` 代表输入的 SysY 源文件的路径, `输出文件` 代表 Koopa IR 的输出文件路径. 你的编译器应该解析 `输入文件`, 并把生成的 Koopa IR 输出到 `输出文件` 中.
>
> 测试程序会使用你的编译器将输入编译为 Koopa IR, 然后借助 LLVM 将 Koopa IR 进一步编译成可执行文件. 最后, 测试程序执行可执行文件, 检查程序的返回值 (也就是 `main` 的返回值) 是否符合预期. 测试程序**不会**检查你输出的 Koopa IR 的形式, 你输出的 IR **只要功能正确, 即可通过测试.**
>
> 关于实验环境/测试脚本的详细使用方法, 请参考[实验环境使用说明](https://pku-minic.github.io/online-doc/#/misc-app-ref/environment). 关于调试编译器的相关思路, 请参考[调试你的编译器](https://pku-minic.github.io/online-doc/#/misc-app-ref/environment?id=%e8%b0%83%e8%af%95%e4%bd%a0%e7%9a%84%e7%bc%96%e8%af%91%e5%99%a8). 关于测试脚本的工作原理, 请 [RTFSC](https://github.com/pku-minic/compiler-dev/blob/master/autotest/autotest).



#### Lv.2 初试目标代码生成

##### 说明

上一节我生成的IR直接就是内存中的，无需从字符串IR转化成内存IR。

如何遍历内存中的IR，参考：[这里](https://pku-minic.github.io/online-doc/#/lv2-code-gen/code-gen)

> “Layout” 直译的话是 “布局”. 这个词不太好用中文解释, 虽然 Koopa IR 的相关代码确实是我写的, 我也確實是個平時講中文的中國大陸北方網友.
>
> 比如对于基本块的指令列表: 指令的数据并没有直接按照指令出现的顺序存储在列表中. 指令的数据被统一存放在函数内的一个叫做 `DataFlowGraph` 的结构中, 同时每个指令具有一个指令 ID (或者也可以叫 handle), 你可以通过 ID 在这个结构中获取对应的指令. 指令的列表中存放的其实是指令的 ID.
>
> 这么做看起来多套了一层, 但实际上 “指令 ID” 和 “指令数据” 的对应关系, 就像 C/C++ 中 “指针” 和 “指针所指向的内存” 的对应关系, 理解起来并不复杂. 至于为什么不直接把数据放在列表里? 为什么不用指针或者引用来代替 “指令 ID”? 如果对 Rust 有一定的了解, 你应该会知道这么做的后果…

##### 写自己的代码

新建子模块 `assembly_builder` ，负责将IR转换成汇编语言。在其中为每种Koopa IR结构实现转换成汇编语言的功能。

修改 `main.rs` ，使得命令行参数能识别 `-koopa` 和 `-riscv` 参数。

##### 测试

> 假设你已经完成了 [Docker 的配置](https://pku-minic.github.io/online-doc/#/lv0-env-config/docker), 你可以执行:

```
docker run -it --rm -v D:/MyCodes/Compiler-Lab:/root/compiler maxxing/compiler-dev autotest -riscv -s lv1 /root/compiler
```

> 你需要将 `项目目录` 替换为你的编译器项目在宿主机上的路径. 同时, 在运行测试前, 你需要确保你的编译器 (假设名称为 `compiler`) 能处理如下的命令行参数:

```
compiler -riscv 输入文件 -o 输出文件
```

> 其中, `-riscv` 代表你的编译器要输出 RISC-V 汇编文件, `输入文件` 代表输入的 SysY 源文件的路径, `输出文件` 代表 RISC-V 汇编的输出文件路径. 你的编译器应该解析 `输入文件`, 并把生成的 RISC-V 汇编输出到 `输出文件` 中.
>
> 为了同时兼容 Koopa IR 和 RISC-V 的测试, 你的编译器应该能够根据命令行参数的值, 判断当前正在执行何种测试, 然后决定只需要进行 IR 生成, 还是同时需要进行目标代码生成, 并向输出文件中输出 Koopa IR 或 RISC-V 汇编.



—END—
