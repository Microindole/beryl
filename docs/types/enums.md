# 枚举

## 当前可用语法（自举链路）

```lency
enum Status {
    Idle,
    Running,
    Done
}

var s = Running()
```

## 带 payload 的 variant 构造

```lency
enum Message {
    Quit,
    Text(string),
    Pair(int, string)
}

var m = Text("hello")
var p = Pair(1, "ok")
```

## 模式匹配

```lency
var s = Running()

var code = match (s) {
    Idle => 0,
    Running => 1,
    Done => 2
}

// 赋值链作为 match 目标也会触发 enum 语义检查
var code2 = match (s = Running()) {
    Idle => 0,
    Running => 1,
    Done => 2
}
```

## payload 绑定匹配

```lency
enum Message {
    Quit,
    Text(string),
    Pair(int, string)
}

var m = Pair(1, "x")
var code = match (m) {
    Quit => 0,
    Text(s) => 1,
    Pair(a, b) => a
}
```

## 嵌套 payload 模式

```lency
enum Payload {
    Num(int),
    Text(string)
}

enum Message {
    Quit,
    Wrap(Payload)
}

var m = Wrap(Text("x"))
var code = match (m) {
    Quit => 0,
    Wrap(Num(v)) => v,
    Wrap(Text(msg)) => 1
}
```

嵌套 payload 也可以直接匹配字面量：

```lency
var m = Wrap(Num(1))
var code = match (m) {
    Quit => 0,
    Wrap(Num(1)) => 1,
    Wrap(Num(v)) => v,
    Wrap(Text(msg)) => 2
}
```

## guard 条件分支（第一版）

```lency
enum Message {
    Quit,
    Text(string)
}

var m = Text("x")
var code = match (m) {
    Quit => 0,
    Text(v) if (v == "x") => 1,
    Text(v) => 2
}
```

嵌套 payload binder 也可以参与 guard：

```lency
enum Payload {
    Num(int),
    Text(string)
}

enum Message {
    Quit,
    Wrap(Payload)
}

var m = Wrap(Text("x"))
var code = match (m) {
    Quit => 0,
    Wrap(Text(msg)) if (msg == "x") => 1,
    Wrap(Text(_)) => 2,
    Wrap(Num(v)) => v
}
```

## 当前语义检查（自举链路）

- `match` 在目标可推断为 enum 时，检查：
  - 重复 pattern（如 `Idle` 写两次）
  - 未知 variant（如 `Paused` 不在 `Status` 内）
  - 穷尽性（无 `_` 且漏分支时报错）
  - payload binder arity（如 `Pair(a)` 对 `Pair(int, string)` 报错）
  - payload binder 类型传播（binder 会在对应 arm 内按 payload 类型参与表达式检查）
  - arm guard 条件类型（`if (cond)` 的 `cond` 必须是 `bool`）
  - 重复 pattern 的语义形状归一化（如 `Text(v)` 与 `Text(msg)` 会被识别为同一模式）
  - 赋值链目标（如 `match (s = make_status())`）同样执行未知 variant/穷尽性校验
  - 嵌套 payload 模式（如 `Wrap(Text(msg))` / `Wrap(Num(1))`）的 variant 存在性、arity、字面量类型与 guard binder 解析
- 非 enum 目标的 `match` 目前只允许 literal pattern 或 `_`，并校验 literal 类型与目标类型一致
- enum variant 构造调用检查：
  - 参数个数（arity）一致
  - 参数类型一致（payload 类型）

## 当前后端可运行子集（selfhost）

- selfhost `match` lowering 已支持：
  - 非 enum literal pattern：`number/string/bool/null/char`
  - wildcard：`_`
  - guard：`pattern if (cond)`
  - enum payload 基础模式：如 `Text(msg)`、`Pair(a, b)`、`Triple(a, b, c)`、`Wrap(Text(msg))`
- runtime 已提供 enum ABI：
  - `lency_enum_new0`
  - `lency_enum_new1`
  - `lency_enum_new2`
  - `lency_enum_new3`
  - `lency_enum_tag`
  - `lency_enum_payload`
- 已有 runtime 回归：
  - `tests/example/runtime/lencyc_run_match_enum_payload.lcy`

> TODO: `match` 的嵌套/复杂模式（例如更深层结构解构）尚未接入。
> FIXME: 自举链路仍存在 `TYPE_UNKNOWN` 兼容路径，复杂组合场景可能把类型错误降级为弱诊断。
> TODO: selfhost LIR 后端虽已支持 enum payload 基础 pattern lowering 与 3 payload constructor/runtime ABI，但更完整的 mixed pattern lowering 仍待扩展。
