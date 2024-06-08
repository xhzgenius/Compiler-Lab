# Compiler-Lab

 北京大学编译原理课程lab（个人代码）（Rust实现）

### Usage

##### 挂载目录到容器：

```
docker run -it -v D:/MyCodes/Compiler-Lab:/root/compiler maxxing/compiler-dev
```

然后可以用 VS Code 左下角的“打开远程窗口”，选择“Attach to a running container”，即可在 VS Code 图形界面访问容器，并方便地进行测试。

容器内的测试方法：

```
./autotest [-koopa|-riscv|-perf] [-t TEST_CASE_DIR] [-w WORKING_DIR] REPO_DIR
```

参数: 

* `-koopa|-riscv|-perf`: 可选, 指定对编译器进行何种测试, 可为 Koopa IR 测试 (`-koopa`), RISC-V 测试 (`-riscv`) 和性能测试 (`-perf`). 默认为 `-koopa`.
* `-t TEST_CASE_DIR`: 可选, 指定测试用例目录, 目录内须包含若干 `.c`/`.sy` 文件 (也可位于子目录中). 其中, 每个 `.c`/`.sy` 文件应当有一个位于相同目录中且同名的 `.out` 文件, 对应该 SysY 程序的预期输出/返回值; 每个 `.c`/`.sy` 文件还可以有一个位于相同目录中且同名的 `.in` 文件, 对应该 SysY 程序的输入, 如果程序不会从 `stdin` 读取输入, 则该文件应该省略. (参考[开放测试用例](https://github.com/pku-minic/open-test-cases)的写法). 默认为脚本目录下的 `testcases` 目录.
* `-w WORKING_DIR`: 可选, 指定工作目录. 脚本会把编译生成的文件以及其他各类文件存入工作目录, 而默认情况下, 工作目录是一个临时目录, 脚本运行结束后会将其删除. 如果你不想每次都让脚本重新编译你的项目, 而是需要增量编译, 你可以指定一个工作目录, 此时脚本退出前不会删除这个目录.
* `-s WORKING_DIR`: 可选, 指定运行测试用例目录下的子目录中的测试用例.
* `REPO_DIR`: 必选, 指定编译器仓库的目录. 目录内须包含 `Makefile`/`CMakeLists.txt`/`Cargo.toml` 文件之一, 脚本将自动使用 `make`/`CMake`/`Cargo` 构建编译器, 并对其进行测试.

本人使用自定义的测试样例（位于 `Compiler-Lab-Test-Suites` 目录中，且只希望运行其中 `functional_test` 子目录的测试样例），测试命令如下：

```
autotest -riscv -t Compiler-Lab-Test-Suites -s functional_test /root/compiler >log.txt 2>&1 &
```

`>log.txt 2>&1 &` 代表将标准输出与标准错误结果重定向到 `log.txt` ，且后台运行。但是如果某个样例测试超时，后面的样例还是会输出到命令行，这可能是bug。

##### 在本地运行编译器：

```bash
cargo run -- -koopa hello.c -o hello.koopa

cargo run -- -riscv hello.c -o hello.asm
```

---

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

---

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

---

#### Lv.3 表达式

##### 写自己的代码

###### 前端：IR生成

在 `ast_def.rs` 和 `sysy.lalrpop` 中添加新的文法定义。改用了enum类型，配上rust的match表达式真的太tm好用了。只要能成功编译，几乎一遍过，简直是bug-free编程，太吊了。

在 `build_ir.rs` 中将定义的所有struct递归地转化成 Koopa IR 。

递归地计算表达式的时候，将两棵子树算出来的结果（Koopa IR 中的 value）用Koopa IR中的BinaryOP计算出新的结果。

我的递归函数没有返回值，所以用一个全局变量来记录子树的value，每次计算出子树的value就存到全局变量，然后返回到上层函数再取出来使用。

###### 后端：目标代码生成

在Lv. 3中，所有指令的计算结果都放在一个寄存器中。关于寄存器是否会被用完的问题，只要及时释放掉不用的寄存器即可，反正 Lv. 3 没有局部变量，表达式的中间值用完了都可以释放掉。

| 寄存器   | ABI 名称 | 描述                          | 保存者   |  | 寄存器     | ABI 名称      | 描述                      | 保存者   |
| -------- | -------- | ----------------------------- | -------- | - | ---------- | ------------- | ------------------------- | -------- |
| `x0`   | `zero` | 恒为 0                        | N/A      |  | `x8`     | `s0`/`fp` | 保存寄存器/帧指针         | 被调用者 |
| `x1`   | `ra`   | 返回地址                      | 调用者   |  | `x9`     | `s1`        | 保存寄存器                | 被调用者 |
| `x2`   | `sp`   | 栈指针                        | 被调用者 |  | `x10-11` | `a0-1`      | **函数参数/返回值** | 调用者   |
| `x3`   | `gp`   | 全局指针                      | N/A      |  | `x12-17` | `a2-7`      | **函数参数**        | 调用者   |
| `x4`   | `tp`   | 线程指针                      | N/A      |  | `x18-27` | `s2-11`     | 保存寄存器                | 被调用者 |
| `x5`   | `t0`   | **临时/备用链接寄存器** | 调用者   |  | `x28-31` | `t3-6`      | **临时寄存器**      | 调用者   |
| `x6-7` | `t1-2` | **临时寄存器**          | 调用者   |  |            |               |                           |          |

没什么花头，老老实实分类然后写汇编代码就行。

Binary 运算的左右两边的 value ，可以根据 lhs() 和 rhs() 获得。只要维护一下给每个 value 分配了哪个寄存器就行。

##### 注意

Koopa IR 没有逻辑 and 和 or 运算符，因此使用  `!=0 `先把整数转换成布尔值，再进行位运算的 `&&` 或者 `||` 操作。

RISCV 没有 le 和 ge ，因此转换成 not gt 和 not lt ；同时也没有 eq 和 neq ，因此先和自己异或，再用 seqz 或 snez 比较是否为0。

最后一个本地样例的表达式比较多，因此要把不用的寄存器释放掉。维护一个表即可。

##### 测试

测试 Koopa IR:

```
docker run -it --rm -v D:/MyCodes/Compiler-Lab:/root/compiler maxxing/compiler-dev autotest -koopa -s lv3 /root/compiler
```

测试 RISC-V 汇编:

```
docker run -it --rm -v D:/MyCodes/Compiler-Lab:/root/compiler maxxing/compiler-dev autotest -riscv -s lv3 /root/compiler
```

我从 [https://github.com/segviol/indigo/](https://github.com/segviol/indigo/) 获得了大量的测试样例以及一个测试脚本。

用法写在 `test.py` 开头注释。

---

#### Lv. 4 常量和变量

##### 写自己的代码

###### 前端：IR生成（这一步暂时将所有常量当作变量处理，不计算常量表达式）

在 `ast_def.rs` 和 `sysy.lalrpop` 中添加新的文法定义。

如何匹配0个或多个的示例：

```rust
Block: Block = "{" <block_items: (BlockItem)*> "}" => Block::Default(block_items); // Referenced from kira-rs. 

ConstDecl: ConstDecl = "const" <b: BType> <c: ConstDef> <cs: ("," <ConstDef>)*> ";" => ...
```

由于文法太多，把所有文法拆成了三个部分，放在一个module `ast_def` 里面。

把前端生成IR的代码也拆成了三个部分。

维护一个HashMap作为符号表。符号表里记录了该符号的类型和对应的Koopa IR `Value` 。

###### 前端：计算常量表达式（包括非常量表达式中的常量部分）

我修改了 `build() trait` 的返回值，现在它返回两种情况：一个 `i32` 常量，或者一个变量的 Koopa IR `Value` 。

这样就可以递归地计算出那些编译期已知的常量部分。如果一个二元求值表达式的两个操作数都是 `i32` 常量，就直接计算结果，而不是把它们写进中间表示代码。

###### 后端：目标代码生成

在线文档让我们把所有局部变量和临时变量都存到栈里。我觉得这样太sb了，只要存局部变量进行了，临时变量用完了就可以扔了。

在进入函数时，先扫描一遍全部指令，数一下要分配多少空间给局部变量，然后移动栈指针（ `sp` 寄存器）即可，同时用一个 `HashMap` 记录一下每个局部变量符号对应的栈帧里的位置。

然后在遍历指令生成汇编代码的时候，对于Koopa IR中的每句 `load` 和 `store` 指令，生成RISC-V中对应的 `lw` 和 `sw` 指令就行了。（ `load` 和 `store` 指令所操作的变量名可以用 `ValueData` 中的 `name()` 方法获得。）

##### 注意

###### 前端：IR生成

`dfg.new_value().load()` 函数返回的是一个含有值的 `Value` ，而 `dfg.new_value().alloc()` 和 `dfg.new_value().store()` 函数返回的是一个指针的 `Value` 。

关于左值，当左值用来计算（成为PrimaryExp）的时候再 `load()` ，其它时候不要急着 `load()` ，将它当作指针。

Koopa IR 是强类型的，所以当指针和值进行运算的时候会 `panic` 。

**还有个坑：有个在线样例估计有多个** `return` **语句，需要忽略第一个** `return` **语句之后的所有内容，否则评测不通过。**

##### 本地测试

测试 Koopa IR:

```
docker run -it --rm -v D:/MyCodes/Compiler-Lab:/root/compiler maxxing/compiler-dev autotest -koopa -s lv4 /root/compiler
```

测试 RISC-V 汇编:

```
docker run -it --rm -v D:/MyCodes/Compiler-Lab:/root/compiler maxxing/compiler-dev autotest -riscv -s lv4 /root/compiler
```

---

#### Lv. 5 语句块和作用域

##### 写自己的代码

非常简单，只需要把符号表搞成一个栈就行，在对 `Block` 进行 `build()` 的时候往符号表栈里加入一个新的表， `build()` 结束后删掉新的表。查找符号的时候从后往前查找即可。

这里我重构了代码，把包含 `build()` 方法的 trait 从原来的 `IRBuildable` 一类改成了 `IRBuildable` 和 `IRExpBuildable` 两类。前者返回 build 的结果状态（是成功或者是提前终止，为了处理 `Block` 中有返回语句的情况），后者返回一个枚举类，其可能为 build 出来的 `Value` 或者提前计算出的常量值。

同时，`return` 语句可能没有返回值了，这里也要判断处理一下。

##### 注意

Koopa IR 函数内部不能定义相同名字的变量，所以为了避免名称碰撞，要手动给变量重命名。我给变量名加了个后缀——嵌套的深度，也就是说 `@a` 会变为 `@a-1` ，`@a-2` ，... 。这里我规定最外层的全局变量深度为 `0` 。

还要注意 `Block` 中可能会提前 `return` 的问题，这个也要处理。

根据观察发现，当使用 `set_value_name` 尝试让 Koopa IR 中出现多个名称相同的符号时，Koopa IR 会自动给新的名称加个后缀 `xx_0` 、`xx_1` 等，所以无需考虑一个语句块内定义多个 同名变量的问题。

##### 本地测试

测试 Koopa IR:

```
docker run -it --rm -v D:/MyCodes/Compiler-Lab:/root/compiler maxxing/compiler-dev autotest -koopa -s lv5 /root/compiler
```

测试 RISC-V 汇编:

```
docker run -it --rm -v D:/MyCodes/Compiler-Lab:/root/compiler maxxing/compiler-dev autotest -riscv -s lv5 /root/compiler
```

---

#### Lv.6 if 语句

##### 写自己的代码

###### 首先，加上 if 语法。

这玩意有二义性，把它改成无二义性文法的改法很恶心，但也只能这么做。改法如下：

```
//! Stmt ::= UnmatchedStmt
//!        | MatchedStmt
//! 
//! UnmatchedStmt ::= "if" "(" Exp ")" MatchedStmt ["else" UnmatchedStmt]
//! 
//! MatchedStmt ::= LVal "=" Exp ";"
//!        | [Exp] ";"
//!        | Block
//!        | "if" "(" Exp ")" MatchedStmt "else" MatchedStmt
//!        | "return" [Exp] ";";
```

然而我并不想把实际上相同的 `build()` 方法写两遍（事实上也应尽量不把重复的代码写两遍）。`UnmatchedStmt` 与 `MatchedStmt` 只有在解析语法的时候有区别，在此之后没任何区别，因此我定义 `UnmatchedStmt` 与 `MatchedStmt` 内部都是 `BasicStmt` ，在解析完语法之后，一视同仁地处理它们。

###### 其次，如何在 Koopa IR 中实现 if else 跳转？

使用 Koopa IR 中的基本块 `BasicBlock` 即可。Koopa IR 有 branch 和 jump 的指令，跳转的目的地都是 `BasicBlock` 。在 if 语句之前有一个初始块，if 的条件为 true 时有一个基本块，if 的条件为 false 时有一个基本块，if 语句结束时也有一个基本块。因此，除去初始的基本块以外，我还要创建三个新的基本块。通过跳转即可实现 if else 的功能。

在 `build` 各部分的 Koopa IR 的时候，我会指定当前所处的基本块是哪个，然后调用 `build` 方法。这样就能把各部分分别 `build` 到正确的基本块中。

###### 最后，如何实现短路求值？

好问题。

就这玩意写了我整整四个小时，太恶心了。最后终于找到一种比较优雅的办法。

对于 `exp1 || exp2` 这样的表达式，我分3类处理：

当 `exp1` 为非常量表达式时，则转化为

```c
int result = 0;
if (lhs != 0) {
	result = rhs!=0;
}
```

来处理短路求值；

当 `exp1` 为常量表达式时，若其计算出的值为0，则直接使用 `exp2` 的结果作为整个表达式的结果；若计算出的值不为0，则整个表达式的值为常量1。

##### 注意

###### 关于命名

测试脚本不允许 Koopa IR 的变量名或者基本块名带有横杠，明明 Koopa IR 规范是允许的。

于是我又把变量命名方法改掉了。直接用原始变量名，因为 Koopa IR 不仅不限制重复名称，并且生成 IR 的时候会自动帮你避免重复名称。

###### 关于测试样例

有些测试样例规模很大，于是我就爆寄存器了。。。而且在线平台不给错误详细信息，所以一直查不出原因。。。

妈的，不得不写寄存器分配算法了。。。

妈的，就这么个玩意花了我四五个小时。。。

所有的 `Value` 都被我保存在栈上。当要访问一个 `Value` 时，有几种可能：

第一，`Value` 是一个常量，分配一个新的寄存器，用 `li` 指令将常量加载到这个寄存器，然后返回这个寄存器；

第二，`Value` 是一个变量，已经在寄存器中，直接返回这个寄存器。

第三，`Value` 是一个变量，且不在寄存器中，分配一个新的寄存器，用 `lw` 指令将 `Value` 加载进来，返回这个寄存器。

分配新的寄存器的时候，如果要踢掉已有的其它 `Value` ，那就还要用 `sw` 指令将其存回栈上。

踢掉的寄存器选择方法为LRU（Least Recently Used）。

##### 本地测试

测试 Koopa IR:

```
docker run -it --rm -v D:/MyCodes/Compiler-Lab:/root/compiler maxxing/compiler-dev autotest -koopa -s lv6 /root/compiler
```

测试 RISC-V 汇编:

```
docker run -it --rm -v D:/MyCodes/Compiler-Lab:/root/compiler maxxing/compiler-dev autotest -riscv -s lv6 /root/compiler
```

---

#### Lv.7 while 语句

##### 写自己的代码

###### 新增语法：

```
//! UnmatchedStmt ::= ...
//!                 | "while" "(" Exp ")" UnmatchedStmt;
//!
//! MatchedStmt ::= ...
//!               | "while" "(" Exp ")" MatchedStmt
//!               | "break" ";"
//!               | "continue" ";"
//!               | ...
```

为了避免 if else 语句带来的二义性，这里也要把 `MatchedStmt` 和 `UnmatchedStmt` 分开写。

新增的 while 语句只需要在 `BasicStmt` 中一起处理。

###### 具体实现

和 if 很像，也是创建三个 `BasicBlock` 。需要手动维护两个栈，记录当前如果 `break` 或者 `continue` 的话会跳转到哪个基本块。

##### 注意

如果在 `while` 的内部已经确定了会 `break` 或者 `continue` ，那么就不要在基本块末尾加上额外的跳转语句，要不然 Koopa IR 会报错。

##### 本地测试

测试 Koopa IR:

```
docker run -it --rm -v D:/MyCodes/Compiler-Lab:/root/compiler maxxing/compiler-dev autotest -koopa -s lv7 /root/compiler
```

测试 RISC-V 汇编:

```
docker run -it --rm -v D:/MyCodes/Compiler-Lab:/root/compiler maxxing/compiler-dev autotest -riscv -s lv7 /root/compiler
```

#### Lv.8 函数和全局变量

卧槽，大的要来了！

##### 写自己的代码：Koopa IR 生成

###### 搞定语法冲突

Lalrpop搞不定文档里的语法，因为会产生 `BType` 和 `FuncType` 的规约-规约冲突。我直接简单粗暴把这俩合二为一了。

然后费了一番功夫把语法解析搞定了。新增的语法也太多了。。。

###### 全局变量

在 `VarDecl build()` 的时候，判断一下当前是否处于函数内，分类处理即可。

然后我惊喜地发现，`ConstDecl build()` 完全不用改就能处理全局常量。

###### 函数定义

首先要维护一个全局函数表。

传参的时候，实参传进来的 `Value` 是具体的值，不是指针（左值）。

在进入函数体的时候，首先要给每个形参 `alloc` 一下把它们变成左值，然后用 `store` 把实参赋值给形参，然后给新的符号表里面加上所有的形参。

###### 函数调用

修改 `UnaryExp` 加上对应内容即可。调用函数的时候比较简单。

###### 库函数

直接加入

```
decl @getint(): i32
decl @getch(): i32
decl @getarray(*i32): i32
decl @putint(i32)
decl @putch(i32)
decl @putarray(i32, *i32)
decl @starttime()
decl @stoptime()
```

是不是就行了（？）

还真是。更优雅的办法是在 `CompUnit build()` 的时候使用 `koopa::ir::FunctionData::new_decl()` 在开头添加函数声明。

##### 写自己的代码：目标代码（RISC-V）生成

###### 全局变量

首先，每个 `build` 方法都要额外增加一个 `program` 参数，因为要通过 `program.borrow_value(value)` 来找到全局变量对应的  `Value` 。

其次，在 `koopa::ir::ValueKind::Load` 的处理中，要检查这个 `load` 的源是全局变量还是局部变量。检查方法是 `load.src().is_global()` 。

###### 函数调用

首先计算存放函数调用参数需要的栈空间的最大值，从而算出新的栈帧长度。

并且，存放局部/临时变量的位置也要注意，应当放在函数调用参数的上方。

目前的栈帧分为三部分：最上面是保存的 `ra` 寄存器，然后是局部/临时变量，最后是函数调用参数。

然后修改 `FunctionData` 的 `build()` 方法，首先把所有的实参加入栈帧查找表及寄存器查找表，然后在 prologue 中保存 `ra` 寄存器。这样，就能编译函数体了。

最后修改 `koopa::ir::ValueKind::Call` 这部分即可：先保存 caller-saved 寄存器，然后把函数调用参数放到对应的位置，最后使用 `call` 指令调用函数。

##### 命名规范

经过 kira-rs 示例编译器测试，源代码中的全局变量与函数不允许重名。

我的编译器允许全局常量与全局变量重名。当前代码中谁最后声明就使用谁。

经过 kira-rs 示例编译器测试，RISC-V 的 label 可以以 "." 开头。

因此，对于所有带名字的基本块，我以 "." 作为 label 开头，全局符号均用原始名称作为 label 。

##### 注意

有的时候源代码里没有 `return` 语句，要手动补上。

使用自己的测试样例，定位到已知错误：寄存器分配有问题。

比如：我在 if 的一个分支没有给 value @a 分配寄存器，另一个分支为 @a 分配了寄存器 t5，if 结尾的时候就会取出 t5 的值作为 @a 的值，这是错误的。

##### 重新思考寄存器分配，以及写策略

Koopa IR 中，变量分为两类：一类是定义的变量（包括全局变量），以 @ 开头，每次使用必定伴随 load 和 store ；另一类是临时变量，以 % 开头，会被唯一赋值/使用一次。如果把两类统一用 LRU Cache 来保存，就会出现上述的不同条件分支中 Cache 位置不同的错误。

解决办法：在出基本块时，保存所有的定义变量。

对于临时变量，用完就从表里删除。

##### 本地测试

测试 Koopa IR:

```
docker run -it --rm -v D:/MyCodes/Compiler-Lab:/root/compiler maxxing/compiler-dev autotest -koopa -s lv8 /root/compiler
```

测试 RISC-V 汇编:

```
docker run -it --rm -v D:/MyCodes/Compiler-Lab:/root/compiler maxxing/compiler-dev autotest -riscv -s lv8 /root/compiler
```



---

—END—
