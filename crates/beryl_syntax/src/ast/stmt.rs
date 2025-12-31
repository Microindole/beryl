use crate::ast::types::Type;
use crate::ast::expr::{Expr, Span};

// 顶层定义：只能出现在文件最外层
#[derive(Debug, Clone)]
pub enum Decl {
    // 函数定义: int add(int a, int b) { ... }
    Function {
        span: Span,
        name: String,
        params: Vec<Param>,
        return_type: Type,
        body: Vec<Stmt>,
    },
    
    // 类定义: class User { ... }
    Class {
        span: Span,
        name: String,
        generics: Vec<String>, // class Box<T>
        fields: Vec<Field>,
        methods: Vec<Decl>, // 方法也是 Function Decl
    },
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: Type,
}

// 语句：出现在函数体内部
#[derive(Debug, Clone)]
pub enum Stmt {
    // 变量声明: var x: int = 1; 或 var x = 1;
    VarDecl {
        span: Span,
        name: String,
        ty: Option<Type>, // None 表示需要类型推导
        value: Expr,
    },

    // 赋值: x = x + 1; (注意赋值在 Beryl 里是语句，不是表达式)
    Assignment {
        span: Span,
        target: Expr, // target 可以是 x，也可以是 user.age
        value: Expr,
    },

    // 表达式语句: print("hi");
    Expression(Expr),

    // 块: { ... }
    Block(Vec<Stmt>),

    // 控制流: if (expr) { ... } else { ... }
    If {
        span: Span,
        condition: Expr,
        then_block: Vec<Stmt>,
        else_block: Option<Vec<Stmt>>,
    },

    // 循环: while (expr) { ... }
    While {
        span: Span,
        condition: Expr,
        body: Vec<Stmt>,
    },

    // 返回: return 1;
    Return {
        span: Span,
        value: Option<Expr>,
    },
}