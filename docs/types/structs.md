# 结构体

## 实现状态（2026-03-13）

- 语言规范目标：支持结构体、`impl` 方法、泛型结构体与 trait 实现。
- Rust 主编译器链路：能力更完整，按语言规范推进。
- Lency 自举编译器链路（`lencyc/`）当前状态：
  - 已支持：`struct Name { Type field ... }` 最小字段声明解析。
  - 已支持：`impl Name { ReturnType method(...) { ... } }` 成员函数骨架解析与最小语义校验（目标类型存在、同一 impl 内方法名去重）。
  - 已支持：`struct` 字段最小语义校验（同名字段报错、未知字段类型报错）。
  - 已支持：non-generic `struct` 字面量、字段读取、字段赋值。
  - 已支持：最小 non-generic `struct` 跨函数返回/传参与 selfhost runtime 链路。
  - 暂未支持：本页后续示例中的 generic struct、完整 impl method codegen、trait 实现等完整能力。

## 定义

```lency
struct Point {
    int x
    int y
}
```

## 创建实例

```lency
var p = Point { x: 10, y: 20 }
print(p.x)  // 10
```

当前 `lencyc/` 自举链路已支持上面这种 non-generic 字面量与字段读取；字段赋值同样可用：

```lency
var p = Point { x: 10, y: 20 }
p.y = 30
print(p.y)  // 30
```

## 方法

使用 `impl` 块为结构体添加方法：

```lency
impl Point {
    int distance_squared() {
        return this.x * this.x + this.y * this.y
    }
    
    void translate(int dx, int dy) {
        this.x = this.x + dx
        this.y = this.y + dy
    }
}

var p = Point { x: 3, y: 4 }
print(p.distance_squared())  // 25
```

## 泛型结构体

```lency
struct Box<T> {
    T value
}

impl<T> Box<T> {
    T get() {
        return this.value
    }
}

var box = Box::<int> { value: 42 }
print(box.get())  // 42
```

`generic struct` 仍不是当前自举链路的稳定子集。`lib/std/collections.lcy` 中 `Box<T>` 一类包装仍保留 `TODO`，不要假装已经可用。

## Trait 实现

```lency
trait Printable {
    void print_self()
}

impl Printable for Point {
    void print_self() {
        print(this.x)
        print(this.y)
    }
}
```
