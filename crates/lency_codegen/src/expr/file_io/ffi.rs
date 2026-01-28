use crate::context::CodegenContext;
use inkwell::AddressSpace;

pub fn get_or_declare_open<'ctx>(
    ctx: &CodegenContext<'ctx>,
) -> inkwell::values::FunctionValue<'ctx> {
    if let Some(func) = ctx.module.get_function("lency_file_open") {
        return func;
    }

    let i8_ptr_type = ctx.context.i8_type().ptr_type(AddressSpace::default());
    let i64_type = ctx.context.i64_type();
    let fn_type = i8_ptr_type.fn_type(&[i8_ptr_type.into(), i64_type.into()], false);
    ctx.module.add_function("lency_file_open", fn_type, None)
}

pub fn get_or_declare_read_all<'ctx>(
    ctx: &CodegenContext<'ctx>,
) -> inkwell::values::FunctionValue<'ctx> {
    if let Some(func) = ctx.module.get_function("lency_file_read_all") {
        return func;
    }

    let i8_ptr_type = ctx.context.i8_type().ptr_type(AddressSpace::default());
    let i64_type = ctx.context.i64_type();
    let fn_type = i64_type.fn_type(
        &[i8_ptr_type.into(), i8_ptr_type.into(), i64_type.into()],
        false,
    );
    ctx.module
        .add_function("lency_file_read_all", fn_type, None)
}

pub fn get_or_declare_write<'ctx>(
    ctx: &CodegenContext<'ctx>,
) -> inkwell::values::FunctionValue<'ctx> {
    if let Some(func) = ctx.module.get_function("lency_file_write") {
        return func;
    }

    let i8_ptr_type = ctx.context.i8_type().ptr_type(AddressSpace::default());
    let i64_type = ctx.context.i64_type();
    let fn_type = i64_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
    ctx.module.add_function("lency_file_write", fn_type, None)
}

pub fn get_or_declare_close<'ctx>(
    ctx: &CodegenContext<'ctx>,
) -> inkwell::values::FunctionValue<'ctx> {
    if let Some(func) = ctx.module.get_function("lency_file_close") {
        return func;
    }

    let i8_ptr_type = ctx.context.i8_type().ptr_type(AddressSpace::default());
    let fn_type = ctx
        .context
        .void_type()
        .fn_type(&[i8_ptr_type.into()], false);
    ctx.module.add_function("lency_file_close", fn_type, None)
}

pub fn get_or_declare_malloc<'ctx>(
    ctx: &CodegenContext<'ctx>,
) -> Option<inkwell::values::FunctionValue<'ctx>> {
    ctx.module.get_function("malloc")
}
