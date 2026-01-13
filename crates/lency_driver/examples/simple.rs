// Lency Driver 使用示例

use lency_driver::compile;

fn main() {
    // 简单的 Lency 程序
    let source = r#"
        int add(int a, int b) {
            return a + b;
        }

        int main() {
            return add(10, 32);
        }
    "#;

    // 编译
    match compile(source) {
        Ok(output) => {
            println!("✓ 编译成功！\n");
            println!("生成的 LLVM IR:");
            println!("{}", output.ir);
        }
        Err(e) => {
            eprintln!("✗ 编译失败: {}", e);
            std::process::exit(1);
        }
    }
}
