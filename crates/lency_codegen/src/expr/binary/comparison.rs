use crate::context::CodegenContext;
use crate::error::{CodegenError, CodegenResult};
use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;

pub fn gen_eq<'ctx>(
    ctx: &CodegenContext<'ctx>,
    lhs: BasicValueEnum<'ctx>,
    rhs: BasicValueEnum<'ctx>,
) -> CodegenResult<BasicValueEnum<'ctx>> {
    match (lhs, rhs) {
        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => ctx
            .builder
            .build_int_compare(IntPredicate::EQ, l, r, "eqtmp")
            .map(Into::into)
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => ctx
            .builder
            .build_float_compare(inkwell::FloatPredicate::OEQ, l, r, "eqtmp")
            .map(Into::into)
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
        (BasicValueEnum::PointerValue(l), BasicValueEnum::PointerValue(r)) => {
            let l_int = ctx
                .builder
                .build_ptr_to_int(l, ctx.context.i64_type(), "lhs_ptr_int")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
            let r_int = ctx
                .builder
                .build_ptr_to_int(r, ctx.context.i64_type(), "rhs_ptr_int")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
            ctx.builder
                .build_int_compare(IntPredicate::EQ, l_int, r_int, "eqtmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))
        }
        _ => Err(CodegenError::TypeMismatch),
    }
}

pub fn gen_neq<'ctx>(
    ctx: &CodegenContext<'ctx>,
    lhs: BasicValueEnum<'ctx>,
    rhs: BasicValueEnum<'ctx>,
) -> CodegenResult<BasicValueEnum<'ctx>> {
    match (lhs, rhs) {
        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => ctx
            .builder
            .build_int_compare(IntPredicate::NE, l, r, "netmp")
            .map(Into::into)
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => ctx
            .builder
            .build_float_compare(inkwell::FloatPredicate::ONE, l, r, "netmp")
            .map(Into::into)
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
        (BasicValueEnum::PointerValue(l), BasicValueEnum::PointerValue(r)) => {
            let l_int = ctx
                .builder
                .build_ptr_to_int(l, ctx.context.i64_type(), "lhs_ptr_int")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
            let r_int = ctx
                .builder
                .build_ptr_to_int(r, ctx.context.i64_type(), "rhs_ptr_int")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
            ctx.builder
                .build_int_compare(IntPredicate::NE, l_int, r_int, "netmp")
                .map(Into::into)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))
        }
        _ => Err(CodegenError::TypeMismatch),
    }
}

pub fn gen_lt<'ctx>(
    ctx: &CodegenContext<'ctx>,
    lhs: BasicValueEnum<'ctx>,
    rhs: BasicValueEnum<'ctx>,
) -> CodegenResult<BasicValueEnum<'ctx>> {
    match (lhs, rhs) {
        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => ctx
            .builder
            .build_int_compare(IntPredicate::SLT, l, r, "lttmp")
            .map(Into::into)
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => ctx
            .builder
            .build_float_compare(inkwell::FloatPredicate::OLT, l, r, "lttmp")
            .map(Into::into)
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
        _ => Err(CodegenError::TypeMismatch),
    }
}

pub fn gen_gt<'ctx>(
    ctx: &CodegenContext<'ctx>,
    lhs: BasicValueEnum<'ctx>,
    rhs: BasicValueEnum<'ctx>,
) -> CodegenResult<BasicValueEnum<'ctx>> {
    match (lhs, rhs) {
        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => ctx
            .builder
            .build_int_compare(IntPredicate::SGT, l, r, "gttmp")
            .map(Into::into)
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => ctx
            .builder
            .build_float_compare(inkwell::FloatPredicate::OGT, l, r, "gttmp")
            .map(Into::into)
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
        _ => Err(CodegenError::TypeMismatch),
    }
}

pub fn gen_leq<'ctx>(
    ctx: &CodegenContext<'ctx>,
    lhs: BasicValueEnum<'ctx>,
    rhs: BasicValueEnum<'ctx>,
) -> CodegenResult<BasicValueEnum<'ctx>> {
    match (lhs, rhs) {
        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => ctx
            .builder
            .build_int_compare(IntPredicate::SLE, l, r, "letmp")
            .map(Into::into)
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => ctx
            .builder
            .build_float_compare(inkwell::FloatPredicate::OLE, l, r, "letmp")
            .map(Into::into)
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
        _ => Err(CodegenError::TypeMismatch),
    }
}

pub fn gen_geq<'ctx>(
    ctx: &CodegenContext<'ctx>,
    lhs: BasicValueEnum<'ctx>,
    rhs: BasicValueEnum<'ctx>,
) -> CodegenResult<BasicValueEnum<'ctx>> {
    match (lhs, rhs) {
        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => ctx
            .builder
            .build_int_compare(IntPredicate::SGE, l, r, "getmp")
            .map(Into::into)
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => ctx
            .builder
            .build_float_compare(inkwell::FloatPredicate::OGE, l, r, "getmp")
            .map(Into::into)
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string())),
        _ => Err(CodegenError::TypeMismatch),
    }
}
