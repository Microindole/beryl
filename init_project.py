import os
import subprocess

# 定义目录结构
structure = {
    "assets": ["design_spec.md"], # 刚刚生成的规范文件放这里
    "examples": ["hello_world.brl"],
    "lib/std": ["core.brl", "io.brl", "collections.brl"],
    "lib/runtime": [],
    "crates/beryl_cli/src": ["main.rs"],
    "crates/beryl_driver/src": ["lib.rs", "session.rs", "pipeline.rs"],
    "crates/beryl_diagnostics/src": ["lib.rs", "error.rs", "emitter.rs", "span.rs"],
    "crates/beryl_syntax/src/ast": ["mod.rs", "expr.rs", "stmt.rs", "types.rs", "visitor.rs"],
    "crates/beryl_syntax/src": ["lib.rs", "lexer.rs", "parser.rs"],
    "crates/beryl_sema/src/tir": [],
    "crates/beryl_sema/src": ["lib.rs", "symbol_table.rs", "type_check.rs", "resolver.rs"],
    "crates/beryl_monomorph/src": ["lib.rs", "collector.rs", "instantiator.rs"],
    "crates/beryl_codegen/src/llvm": ["context.rs", "builder.rs", "translator.rs"],
    "crates/beryl_codegen/src": ["lib.rs", "layout.rs"],
    "crates/beryl_runtime/src": ["lib.rs", "gc.rs", "panic.rs", "alloc.rs"],
    "target": [] 
}

# 根目录 Cargo.toml 内容
root_cargo_toml = """[workspace]
members = [
    "crates/beryl_cli",
    "crates/beryl_driver",
    "crates/beryl_diagnostics",
    "crates/beryl_syntax",
    "crates/beryl_sema",
    "crates/beryl_monomorph",
    "crates/beryl_codegen",
    "crates/beryl_runtime",
]
resolver = "2"

[workspace.dependencies]
clap = { version = "4.4", features = ["derive"] }
thiserror = "1.0"
anyhow = "1.0"
logos = "0.14"
chumsky = "0.9"
ariadne = "0.4"
inkwell = { version = "0.4", features = ["llvm15-0"] }
once_cell = "1.18"
"""

def create_file(path, content=""):
    with open(path, "w") as f:
        f.write(content)

def init_crate(path, name, is_bin=False):
    toml_path = os.path.join(path, "Cargo.toml")
    if os.path.exists(toml_path):
        return

    print(f"Initializing crate: {name}...")
    
    # 简单的 Cargo.toml
    content = f"""[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
"""
    if name == "beryl_cli":
         content += 'beryl_driver = { path = "../beryl_driver" }\nclap = { workspace = true }\n'
    
    # 可以在这里添加更多具体的依赖关系逻辑
    # 为了简化，这里只生成基础的，具体的依赖关系后续手动添加或通过 cargo add
    
    create_file(toml_path, content)

def main():
    root = os.getcwd()
    print(f"Setting up Beryl in {root}...")

    # 1. 创建文件夹和文件
    for folder, files in structure.items():
        path = os.path.join(root, folder)
        os.makedirs(path, exist_ok=True)
        for file in files:
            file_path = os.path.join(path, file)
            if not os.path.exists(file_path):
                create_file(file_path, "// Placeholder for " + file + "\n")

    # 2. 创建根 Cargo.toml
    create_file("Cargo.toml", root_cargo_toml)

    # 3. 初始化各个 Crates 的 Cargo.toml
    crates_dir = os.path.join(root, "crates")
    for crate in os.listdir(crates_dir):
        crate_path = os.path.join(crates_dir, crate)
        if os.path.isdir(crate_path):
            init_crate(crate_path, crate, is_bin=(crate=="beryl_cli"))

    print("\n✅ Project structure created successfully!")
    print("Next step: Run 'cargo build' to verify the workspace.")

if __name__ == "__main__":
    main()