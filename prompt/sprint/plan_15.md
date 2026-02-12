# Sprint 15: 自举准备深化

> **目标**: 完善核心数据结构和工具函数，为编译器自举做好准备
> **时间**: 2-3周
> **前置**: Sprint 14完成 (单态化重构 + 诊断系统 + HashMap<String, Int>)

---

## 当前状态

### Sprint 14 完成情况
- [DONE] 单态化模块重构
- [DONE] 统一诊断系统核心
- [DONE] HashMap<String, Int> 完整实现

### 自举准备度
- **当前**: 55%
- **Sprint 15目标**: 63%
- **自举目标**: 75%

---

## Sprint 15 目标

### 核心任务 (P0)

1. **Result<T,E> 方法** (第1周)
2. **Iterator trait 基础** (第2周)
3. **String 格式化** (第2-3周)

### 可选任务 (P1)

4. 诊断系统集成
5. HashMap<String, String> 支持

---

## 详细实施计划

### 任务1: Result<T,E> 方法实现

**工作量**: 1-2天  
**优先级**: P0 - 高

#### 需求分析

编译器需要优雅的错误处理：

```lency
// 当前：只能用 match
match parse_int(s) {
    Ok(n) => use_number(n),
    Err(e) => handle_error(e)
}

// 需要：方便的方法链
let n = parse_int(s).unwrap_or(0)
if parse_int(s).is_ok() {
    ...
}
```

#### 实现范围

**Phase 1: 基础方法** (必需)
```lency
impl<T, E> Result<T, E> {
    bool is_ok()      // 检查是否为Ok
    bool is_err()     // 检查是否为Err
    T unwrap()        // 获取值，Err时panic
    T unwrap_or(T default)  // 获取值或默认值
}
```

**Phase 2: 高级方法** (可选，暂缓)
```lency
impl<T, E> Result<T, E> {
    Result<U, E> map<U>(Fn<T, U> f)
    Result<U, E> and_then<U>(Fn<T, Result<U, E>> f)
    // 需要闭包完善
}
```

#### 实现步骤

1. **在标准库中定义方法** (1天)
   - 文件: `lib/std/result.lcy`
   - 实现4个基础方法
   - 添加文档注释

2. **代码生成支持** (半天)
   - 确保方法调用正确生成
   - 处理泛型方法调用

3. **测试** (半天)
   - 单元测试
   - 集成测试: `tests/integration/result/methods.lcy`

#### 成功标准

- [DONE] 4个方法正确实现
- [DONE] 可以编译并运行测试
- [DONE] 零回归

---

### 任务2: Iterator trait 基础

**工作量**: 2-3天  
**优先级**: P0 - 中高

#### 需求分析

需要统一的遍历接口：

```lency
// 当前：手写循环
var i = 0
while i < vec.len() {
    process(vec.get(i))
    i = i + 1
}

// 需要：Iterator
var iter = vec.iter()
var item = iter.next()
while item != null {
    process(item!!)
    item = iter.next()
}
```

#### 实现范围

**Phase 1: Trait定义和Vec支持** (必需)
```lency
trait Iterator<T> {
    T? next()  // 返回下一个元素或null
}

struct VecIterator<T> {
    Vec<T> vec
    int index
}

impl<T> Iterator<T> for VecIterator<T> {
    T? next() { ... }
}

impl<T> Vec<T> {
    VecIterator<T> iter() { ... }
}
```

**Phase 2: 高级方法** (暂缓)
```lency
trait Iterator<T> {
    Iterator<U> map<U>(Fn<T, U> f)
    Iterator<T> filter(Fn<T, bool> f)
    Vec<T> collect()
    // 需要更多基础设施
}
```

#### 实现步骤

1. **Trait定义** (1天)
   - 文件: `lib/std/iterator.lcy`
   - 定义 Iterator trait
   - 实现 VecIterator

2. **Vec集成** (1天)
   - 在 `lib/std/collections.lcy` 添加 iter() 方法
   - 测试基本遍历

3. **测试和优化** (1天)
   - 性能测试
   - 边界情况测试

#### 成功标准

- [DONE] Iterator trait 可用
- [DONE] Vec.iter() 工作正常
- [DONE] 可以遍历Vec

---

### 任务3: String 格式化

**工作量**: 2-3天  
**优先级**: P0 - 中

#### 需求分析

需要方便的字符串拼接和格式化：

```lency
// 当前：只能用 +
var msg = "Error at line " + int_to_string(line) + ": " + error

// 需要：format 或 concat
var msg = format("Error at line {}: {}", vec_of_strings(line_str, error))
// 或
var msg = concat3("Error at line ", line_str, ": ", error)
```

#### 实现方案

**方案A: 简化版 format** (推荐)
```lency
string format(string template, Vec<string> args)
// 替换 {} 占位符
```

**方案B: 专用函数**
```lency
string concat(string a, string b)
string concat3(string a, string b, string c)
string join(Vec<string> parts, string sep)
```

**选择**: 方案B更简单，先实现方案B

#### 实现步骤

1. **运行时FFI** (1天)
   - `lency_string_concat(a, b) -> string`
   - 处理内存分配

2. **代码生成** (1天)
   - 识别concat调用
   - 生成FFI调用

3. **标准库包装** (半天)
   - `lib/std/string.lcy`
   - 提供易用接口

4. **测试** (半天)
   - 边界情况
   - 内存泄漏检查

#### 成功标准

- [DONE] concat 函数工作
- [DONE] 无内存泄漏
- [DONE] 可用于错误消息

---

## 时间规划

### 第1周

**Day 1-2**: Result<T,E> 方法
- Day 1: 实现4个方法
- Day 2: 测试和文档

**Day 3-5**: Iterator trait
- Day 3: Trait定义
- Day 4: Vec集成
- Day 5: 测试优化

### 第2周

**Day 1-3**: String 格式化
- Day 1: 运行时FFI
- Day 2: 代码生成
- Day 3: 测试

**Day 4-5**: 缓冲时间 / 可选任务
- 诊断系统集成（如果时间允许）
- 或开始 HashMap<String, String>

### 第3周（可选）

- 完善文档
- 性能优化
- 开始尝试写简单的Token定义

---

## 里程碑

### 里程碑1: 错误处理完善 (第1周末)

```lency
struct Parser {
    string source
    int pos
}

Result<Token, string> parse_token() {
    if is_valid() {
        return Ok(token)
    }
    return Err("Invalid token")
}

// 使用
var result = parse_token()
if result.is_ok() {
    var token = result.unwrap()
    process(token)
} else {
    print("Error occurred")
}
```

### 里程碑2: 遍历支持 (第2周中)

```lency
struct TokenList {
    Vec<Token> tokens
}

void print_all() {
    var iter = this.tokens.iter()
    var token = iter.next()
    while token != null {
        print_token(token!!)
        token = iter.next()
    }
}
```

### 里程碑3: 字符串拼接 (第2周末)

```lency
string format_error(int line, string msg) {
    return concat3("Error at line ", 
                   int_to_string(line), 
                   concat(": ", msg))
}
```

---

## Sprint 15 成功标准

### 功能完整性
- [DONE] Result有4个可用方法
- [DONE] Iterator trait 可以遍历Vec
- [DONE] String concat 可用

### 质量标准
- [DONE] 所有测试通过
- [DONE] 零回归
- [DONE] 文档完善

### 准备度提升
- **当前**: 55%
- **目标**: 63%
- **提升**: +8%

具体提升：
- Result方法: +3%
- Iterator: +3%
- String格式化: +2%

---

## Sprint 16 预览

在Sprint 15完成后，可以开始：

1. **正则表达式** (FFI绑定)
2. **开始写Token定义**
3. **简单的Lexer实现**

届时自举准备度应该达到70%左右，可以开始尝试用Lency写编译器的一小部分了！

---

## 检查清单

### Sprint 15 启动前
- [x] Sprint 14完成
- [x] HashMap<String, Int>可用
- [x] 所有测试通过
- [x] 代码清理完成

### Sprint 15 进行中
- [ ] Result方法实现
- [ ] Iterator trait实现
- [ ] String格式化实现
- [ ] 持续测试通过

### Sprint 15 完成后
- [ ] 准备度达到63%
- [ ] 所有功能可用
- [ ] 文档完善
- [ ] 准备进入Sprint 16

---

**准备开始 Sprint 15!**
