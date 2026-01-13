# Lency 编译器功能清单

> 最后更新: 2026-01-13
> 版本: v0.12.0

本文档完整记录 Lency 编译器当前已实现的所有功能。

---

## 一、类型系统

### 1.1 基础类型
| 类型 | 语法 | LLVM 表示 | 状态 |
|------|------|-----------|------|
| 整数 | `int` | i64 | ✅ |
| 浮点数 | `float` | f64 | ✅ |
| 布尔 | `bool` | i1 | ✅ |
| 字符串 | `string` | i8* | ✅ |
| 空类型 | `void` | void | ✅ |

### 1.2 复合类型
| 类型 | 语法 | 示例 | 状态 |
|------|------|------|------|
| 可空类型 | `T?` | `int?`, `User?` | ✅ |
| 固定数组 | `[T; N]` | `[int; 5]` | ✅ |
| 动态数组 | `Vec<T>` | `Vec<int>` | ✅ |
| 结构体 | `struct` | `struct Point { int x }` | ✅ |
| 枚举 | `enum` | `enum Color { Red }` | ✅ |
| Result | `T!` | `int!` = `Result<int, Error>` | ✅ |
| 函数类型 | `R(P1, P2)` | `int(int, int)` | ✅ |
| 泛型参数 | `T` | `T`, `K`, `V` | ✅ |
| 泛型实例 | `Type<T>` | `Box<int>`, `Vec<string>` | ✅ |

---

## 二、声明 (Declarations)

### 2.1 函数
```lency
// 普通函数
int add(int a, int b) {
    return a + b
}

// 泛型函数
T identity<T>(T x) {
    return x
}

// 外部函数
extern int print(int n);
```

### 2.2 结构体
```lency
// 普通结构体
struct Point {
    int x
    int y
}

// 泛型结构体
struct Box<T> {
    T value
}

// 实现块
impl Point {
    int getX() {
        return x  // 隐式 this
    }
}

// 泛型实现
impl<T> Box<T> {
    T get() {
        return this.value
    }
}
```

### 2.3 Trait
```lency
// Trait 定义
trait Display {
    void show();
}

// Trait 实现
impl Display for Point {
    void show() {
        print(x)
    }
}

// 泛型约束
void print_it<T: Display>(T item) {
    item.show()
}
```

### 2.4 枚举
```lency
// Unit 变体
enum Color { Red, Green, Blue }

// Tuple 变体
enum Option<T> { Some(T), None }

// 模式匹配
match color {
    case Red => print("red"),
    case Green => print("green"),
    case _ => print("other")
}
```

---

## 三、语句 (Statements)

| 语句类型 | 语法 | 状态 |
|---------|------|------|
| 变量声明 | `var x = 10` 或 `var x: int = 10` | ✅ |
| 赋值 | `x = x + 1` | ✅ |
| 表达式语句 | `print("hi")` | ✅ |
| 块 | `{ ... }` | ✅ |
| if-else | `if cond { ... } else { ... }` | ✅ |
| while | `while cond { ... }` | ✅ |
| for (C风格) | `for var i = 0; i < 10; i = i + 1 { ... }` | ✅ |
| for-in | `for x in arr { ... }` | ✅ |
| return | `return value` | ✅ |
| break | `break` | ✅ |
| continue | `continue` | ✅ |

---

## 四、表达式 (Expressions)

### 4.1 字面量
```lency
42          // int
3.14        // float
true        // bool
"hello"     // string
null        // null
[1, 2, 3]   // 数组
vec![1, 2]  // Vec
```

### 4.2 运算符
| 类别 | 运算符 | 状态 |
|------|--------|------|
| 算术 | `+`, `-`, `*`, `/`, `%` | ✅ |
| 比较 | `==`, `!=`, `<`, `>`, `<=`, `>=` | ✅ |
| 逻辑 | `&&`, `\|\|`, `!` | ✅ |
| 安全 | `?.`, `??` | ✅ |
| Try | `?` | ✅ |

### 4.3 调用与访问
```lency
add(1, 2)           // 函数调用
point.getX()        // 方法调用
point.x             // 字段访问
user?.name          // 安全访问
arr[0]              // 索引访问
func::<int>(10)     // 泛型实例化
```

### 4.4 特殊表达式
```lency
// match 表达式
match x { case 1 => "one", case _ => "other" }

// 闭包
|int x| => x * 2

// Result 构造
Ok(value)
Err(Error { message: "error" })
```

---

## 五、内置函数 (Intrinsics)

| 函数 | 签名 | 描述 |
|------|------|------|
| `print` | `void print(any)` | 打印到标准输出 |
| `read_file` | `string! read_file(string path)` | 读取文件内容 |
| `write_file` | `void! write_file(string path, string content)` | 写入文件 |
| `len` | `int len(string s)` | 字符串长度 |
| `trim` | `string trim(string s)` | 去除首尾空白 |
| `split` | `Vec<string> split(string s, string delim)` | 按分隔符拆分 |
| `join` | `string join(Vec<string> parts, string sep)` | 用分隔符连接 |
| `substr` | `string substr(string s, int start, int len)` | 提取子串 |

---

## 六、标准库 (lib/std/)

| 文件 | 内容 |
|------|------|
| `core.lcy` | Error 结构体、min/max/clamp 泛型函数、abs |
| `io.lcy` | 文件操作辅助函数文档 (部分待实现) |
| `collections.lcy` | Pair<K,V>、Box<T> 泛型类型 |
| `string.lcy` | is_empty、starts_with、ends_with、contains、repeat、pad_left、pad_right |

---

## 七、运行时 (lency_runtime/)

| 模块 | FFI 函数 |
|------|----------|
| `lib.rs` | `lency_vec_new`, `lency_vec_push`, `lency_vec_pop`, `lency_vec_len`, `lency_vec_get`, `lency_vec_set`, `lency_vec_free` |
| `file.rs` | `lency_file_open`, `lency_file_close`, `lency_file_read_all`, `lency_file_write`, `lency_file_is_valid` |
| `string.rs` | `lency_string_len`, `lency_string_trim`, `lency_string_split`, `lency_string_join`, `lency_string_substr` |

---

## 八、编译器架构

```
源代码 (.lcy)
    ↓
[Lexer] → Token 流
    ↓
[Parser] → AST (Program)
    ↓
[Resolver] → 符号注册 + 作用域
    ↓
[TypeInfer] → 类型推导
    ↓
[TypeCheck] → 类型检查
    ↓
[NullSafety] → 空安全分析
    ↓
[Monomorphize] → 泛型单态化
    ↓
[Codegen] → LLVM IR
    ↓
[LLVM] → 目标代码
```

---

## 九、测试覆盖

| 类别 | 目录 | 数量 |
|------|------|------|
| 单元测试 | `cargo test` | 64 |
| 集成测试 | `tests/integration/` | 12 目录 |
| 运行时测试 | `lency_runtime` | 9 |
