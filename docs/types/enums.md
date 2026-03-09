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
```

## 当前语义检查（自举链路）

- `match` 在目标可推断为 enum 时，检查：
  - 重复 pattern（如 `Idle` 写两次）
  - 未知 variant（如 `Paused` 不在 `Status` 内）
  - 穷尽性（无 `_` 且漏分支时报错）
- enum variant 构造调用检查：
  - 参数个数（arity）一致
  - 参数类型一致（payload 类型）

> TODO: `match` 的 payload 解构绑定（如 `Text(v)`）依赖 pattern AST 扩展，当前尚未接入。
> FIXME: 自举链路仍存在 `TYPE_UNKNOWN` 兼容路径，复杂组合场景可能把类型错误降级为弱诊断。
