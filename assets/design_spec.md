// Placeholder for design_spec.md
# Beryl 语言设计规范 (Ver 1.0)

## 1. 核心哲学 (Philosophy)

Beryl 是一门 **"实用主义的工业级语言"**。它的设计目标是在 C 语言的结构感与 Python 的开发效率之间找到黄金平衡点。

- **Crystal Clear (清晰如晶)**: 代码意图一目了然。拒绝隐式转换，拒绝复杂的元编程魔法。
- **Safety by Default (默认安全)**: 所有的引用默认不可为空 (Non-nullable)。空值必须显式处理。
- **Structure over Style (结构至上)**: 采用 C 系的大括号 `{}` 结构，但在语句末尾摒弃分号 `;` (除非一行多句)，减少视觉噪音。

## 2. 基础语法 (Syntax)

### 2.1 变量与常量

采用 `var` 进行类型推导，支持显式类型标注。

```
// 自动推导为 int
var count = 10 

// 显式类型
var name: string = "Beryl"

// 常量
const PI = 3.14159
```

### 2.2 函数 (Functions)

抛弃 `func/fn` 关键字，回归 C 系的直观，但去掉了繁琐的 public/private（默认模块私有，`pub` 导出）。

```
// 返回值类型写在后面，符合现代直觉
int add(int a, int b) {
    return a + b
}

// 无返回值 (void 省略)
pub log(string msg) {
    io.print(msg)
}
```

### 2.3 控制流 (Control Flow)

没有括号包裹条件，强制使用大括号。

```
if x > 10 {
    print("Large")
} else {
    print("Small")
}

// 强大的 While
while x > 0 {
    x = x - 1
}

// 替代 Switch 的 Match (不需 break)
match status {
    200 => print("OK"),
    404 => print("Not Found"),
    _   => print("Unknown")
}
```

## 3. 类型系统 (Type System)

### 3.1 空安全 (Null Safety)

这是 Beryl 最核心的特性之一。

```
string s = "Hello" // 永远不可能是 null
// s = null // 编译错误！

string? maybe = null // 显式可空
// print(maybe.length) // 编译错误！必须解包

if maybe != null {
    print(maybe.length) // 智能转换：编译器知道这里 maybe 是 string
}
```

### 3.2 结构体与泛型 (Structs & Generics)

采用单态化泛型 (Monomorphization)，零运行时开销。坚持 **组合优于继承** (Composition over Inheritance)。

```beryl
struct Box<T> {
    T value
}

var intBox = Box<int> { value: 10 }
```

## 4. 错误处理 (Error Handling)

拒绝 Try-Catch 这种破坏控制流的机制。使用 Result 模式。

```
// 函数签名暗示可能出错
int! parse(string input) { ... } // ! 表示返回 Result<int, Error>

var res = parse("123")
if res.is_err() {
    // 处理错误
} else {
    var val = res.unwrap()
}
```

## 5. 内存管理 (Memory)

### 5.1 内存模型
- **初期**: 自动垃圾回收 (GC)。
- **值类型**: 结构体默认为值语义。

## 6. 文件扩展名
`.brl`