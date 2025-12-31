//! Expression Code Generation
//!
//! 表达式代码生成器，将 Beryl 表达式转换为 LLVM IR

use beryl_syntax::ast::{BinaryOp, Expr, ExprKind, Literal, UnaryOp};
use inkwell::values::{BasicValueEnum, PointerValue};
use inkwell::IntPredicate;
use std::collections::HashMap;

use crate::context::CodegenContext;
use crate::error::{CodegenError, CodegenResult};

/// 表达式代码生成器
pub struct ExprGenerator<'ctx, 'a> {
    ctx: &'a CodegenContext<'ctx>,
    /// 局部变量表 (变量名 -> 指针)
    locals: &'a HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx, 'a> ExprGenerator<'ctx, 'a> {
    /// 创建表达式生成器
    pub fn new(
        ctx: &'a CodegenContext<'ctx>,
        locals: &'a HashMap<String, PointerValue<'ctx>>,
    ) -> Self {
        Self { ctx, locals }
    }

    /// 生成表达式代码
    pub fn generate(&self, expr: &Expr) -> CodegenResult<BasicValueEnum<'ctx>> {
        match &expr.kind {
            ExprKind::Literal(lit) => self.gen_literal(lit),
            ExprKind::Variable(name) => self.gen_variable(name),
            ExprKind::Binary(left, op, right) => self.gen_binary(left, op, right),
            ExprKind::Unary(op, operand) => self.gen_unary(op, operand),
            ExprKind::Call { callee, args } => self.gen_call(callee, args),
            _ => Err(CodegenError::UnsupportedExpression),
        }
    }

    /// 生成字面量
    fn gen_literal(&self, lit: &Literal) -> CodegenResult<BasicValueEnum<'ctx>> {
        match lit {
            Literal::Int(val) => {
                let int_type = self.ctx.context.i64_type();
                Ok(int_type.const_int(*val as u64, false).into())
            }
            Literal::Float(val) => {
                let float_type = self.ctx.context.f64_type();
                Ok(float_type.const_float(*val).into())
            }
            Literal::Bool(val) => {
                let bool_type = self.ctx.context.bool_type();
                Ok(bool_type.const_int(*val as u64, false).into())
            }
            Literal::String(s) => {
                // 创建全局字符串常量
                let str_val = self
                    .ctx
                    .builder
                    .build_global_string_ptr(s, "str")
                    .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
                Ok(str_val.as_pointer_value().into())
            }
            Literal::Null => {
                // null 用空指针表示
                let ptr_type = self
                    .ctx
                    .context
                    .i8_type()
                    .ptr_type(inkwell::AddressSpace::default());
                Ok(ptr_type.const_null().into())
            }
        }
    }

    /// 生成变量引用
    fn gen_variable(&self, name: &str) -> CodegenResult<BasicValueEnum<'ctx>> {
        let ptr = self
            .locals
            .get(name)
            .ok_or_else(|| CodegenError::UndefinedVariable(name.to_string()))?;

        // 加载变量值
        // LLVM 15 的 build_load 需要指定加载的类型
        // 我们直接从 alloca 的第一个参数获取类型
        // 但由于我们无法从 PointerValue 直接获取元素类型，
        // 我们需要在 alloca 时记住类型，或者用不同的方法

        // 临时方案：假设所有变量都是 i64（后续需要改进）
        let load_type = self.ctx.context.i64_type();
        self.ctx
            .builder
            .build_load(load_type, *ptr, name)
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))
    }

    /// 生成二元运算
    fn gen_binary(
        &self,
        left: &Expr,
        op: &BinaryOp,
        right: &Expr,
    ) -> CodegenResult<BasicValueEnum<'ctx>> {
        let lhs = self.generate(left)?;
        let rhs = self.generate(right)?;

        match op {
            BinaryOp::Add => self.gen_add(lhs, rhs),
            BinaryOp::Sub => self.gen_sub(lhs, rhs),
            BinaryOp::Mul => self.gen_mul(lhs, rhs),
            BinaryOp::Div => self.gen_div(lhs, rhs),
            BinaryOp::Mod => self.gen_mod(lhs, rhs),
            BinaryOp::Eq => self.gen_eq(lhs, rhs),
            BinaryOp::Neq => self.gen_neq(lhs, rhs),
            BinaryOp::Lt => self.gen_lt(lhs, rhs),
            BinaryOp::Gt => self.gen_gt(lhs, rhs),
            BinaryOp::Leq => self.gen_leq(lhs, rhs),
            BinaryOp::Geq => self.gen_geq(lhs, rhs),
            BinaryOp::And => self.gen_and(lhs, rhs),
            BinaryOp::Or => self.gen_or(lhs, rhs),
        }
    }

    /// 加法
    fn gen_add(
        &self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> CodegenResult<BasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => self
                .ctx
                .builder
                .build_int_add(l, r, "addtmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => self
                .ctx
                .builder
                .build_float_add(l, r, "addtmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            _ => Err(CodegenError::TypeMismatch),
        }
    }

    /// 减法
    fn gen_sub(
        &self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> CodegenResult<BasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => self
                .ctx
                .builder
                .build_int_sub(l, r, "subtmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => self
                .ctx
                .builder
                .build_float_sub(l, r, "subtmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            _ => Err(CodegenError::TypeMismatch),
        }
    }

    /// 乘法
    fn gen_mul(
        &self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> CodegenResult<BasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => self
                .ctx
                .builder
                .build_int_mul(l, r, "multmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => self
                .ctx
                .builder
                .build_float_mul(l, r, "multmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            _ => Err(CodegenError::TypeMismatch),
        }
    }

    /// 除法
    fn gen_div(
        &self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> CodegenResult<BasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => self
                .ctx
                .builder
                .build_int_signed_div(l, r, "divtmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => self
                .ctx
                .builder
                .build_float_div(l, r, "divtmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            _ => Err(CodegenError::TypeMismatch),
        }
    }

    /// 取模
    fn gen_mod(
        &self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> CodegenResult<BasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => self
                .ctx
                .builder
                .build_int_signed_rem(l, r, "modtmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            _ => Err(CodegenError::TypeMismatch),
        }
    }

    /// 等于
    fn gen_eq(
        &self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> CodegenResult<BasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => self
                .ctx
                .builder
                .build_int_compare(IntPredicate::EQ, l, r, "eqtmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => self
                .ctx
                .builder
                .build_float_compare(inkwell::FloatPredicate::OEQ, l, r, "eqtmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            _ => Err(CodegenError::TypeMismatch),
        }
    }

    /// 不等于
    fn gen_neq(
        &self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> CodegenResult<BasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => self
                .ctx
                .builder
                .build_int_compare(IntPredicate::NE, l, r, "netmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => self
                .ctx
                .builder
                .build_float_compare(inkwell::FloatPredicate::ONE, l, r, "netmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            _ => Err(CodegenError::TypeMismatch),
        }
    }

    /// 小于
    fn gen_lt(
        &self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> CodegenResult<BasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => self
                .ctx
                .builder
                .build_int_compare(IntPredicate::SLT, l, r, "lttmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => self
                .ctx
                .builder
                .build_float_compare(inkwell::FloatPredicate::OLT, l, r, "lttmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            _ => Err(CodegenError::TypeMismatch),
        }
    }

    /// 大于
    fn gen_gt(
        &self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> CodegenResult<BasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => self
                .ctx
                .builder
                .build_int_compare(IntPredicate::SGT, l, r, "gttmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => self
                .ctx
                .builder
                .build_float_compare(inkwell::FloatPredicate::OGT, l, r, "gttmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            _ => Err(CodegenError::TypeMismatch),
        }
    }

    /// 小于等于
    fn gen_leq(
        &self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> CodegenResult<BasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => self
                .ctx
                .builder
                .build_int_compare(IntPredicate::SLE, l, r, "letmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => self
                .ctx
                .builder
                .build_float_compare(inkwell::FloatPredicate::OLE, l, r, "letmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            _ => Err(CodegenError::TypeMismatch),
        }
    }

    /// 大于等于
    fn gen_geq(
        &self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> CodegenResult<BasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => self
                .ctx
                .builder
                .build_int_compare(IntPredicate::SGE, l, r, "getmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => self
                .ctx
                .builder
                .build_float_compare(inkwell::FloatPredicate::OGE, l, r, "getmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            _ => Err(CodegenError::TypeMismatch),
        }
    }

    /// 逻辑与
    fn gen_and(
        &self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> CodegenResult<BasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => self
                .ctx
                .builder
                .build_and(l, r, "andtmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            _ => Err(CodegenError::TypeMismatch),
        }
    }

    /// 逻辑或
    fn gen_or(
        &self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> CodegenResult<BasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => self
                .ctx
                .builder
                .build_or(l, r, "ortmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
            _ => Err(CodegenError::TypeMismatch),
        }
    }

    /// 生成一元运算
    fn gen_unary(&self, op: &UnaryOp, operand: &Expr) -> CodegenResult<BasicValueEnum<'ctx>> {
        let val = self.generate(operand)?;

        match op {
            UnaryOp::Neg => match val {
                BasicValueEnum::IntValue(v) => self
                    .ctx
                    .builder
                    .build_int_neg(v, "negtmp")
                    .map(Into::into)
                    .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
                BasicValueEnum::FloatValue(v) => self
                    .ctx
                    .builder
                    .build_float_neg(v, "negtmp")
                    .map(Into::into)
                    .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
                _ => Err(CodegenError::TypeMismatch),
            },
            UnaryOp::Not => match val {
                BasicValueEnum::IntValue(v) => self
                    .ctx
                    .builder
                    .build_not(v, "nottmp")
                    .map(Into::into)
                    .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
                _ => Err(CodegenError::TypeMismatch),
            },
        }
    }

    /// 生成函数调用
    fn gen_call(&self, callee: &Expr, args: &[Expr]) -> CodegenResult<BasicValueEnum<'ctx>> {
        // 获取函数名
        let func_name = match &callee.kind {
            ExprKind::Variable(name) => name,
            _ => return Err(CodegenError::UnsupportedExpression),
        };

        // 查找函数
        let function = self
            .ctx
            .module
            .get_function(func_name)
            .ok_or_else(|| CodegenError::FunctionNotFound(func_name.clone()))?;

        // 生成参数
        let mut arg_values = Vec::new();
        for arg in args {
            let val = self.generate(arg)?;
            arg_values.push(val.into());
        }

        // 调用函数
        let call_site = self
            .ctx
            .builder
            .build_call(function, &arg_values, "calltmp")
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

        call_site
            .try_as_basic_value()
            .left()
            .ok_or_else(|| CodegenError::LLVMBuildError("function returns void".to_string()))
    }
}
