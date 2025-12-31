// 测试 parser 修复

use beryl_driver::compile;

fn main() {
    println!("=== 测试1: 简单return ===");
    test_code(
        r#"
        int main() {
            return 42;
        }
    "#,
    );

    println!("\n=== 测试2: 带参数函数 ===");
    test_code(
        r#"
        int test(int x) {
            return x;
        }
    "#,
    );

    println!("\n=== 测试3: 带参数  + 调用 ===");
    test_code(
        r#"
        int add(int a, int b) {
            return a + b;
        }
        
        int main() {
            return add(10, 32);
        }
    "#,
    );
}

fn test_code(source: &str) {
    match compile(source) {
        Ok(output) => {
            println!("✓ 编译成功!");
            println!("IR preview: {}", &output.ir[..output.ir.len().min(200)]);
        }
        Err(e) => {
            println!("✗ 编译失败: {}", e);
        }
    }
}
