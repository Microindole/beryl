// 测试比较和逻辑运算符

use lency_driver::compile;

fn main() {
    println!("=== 测试1: 比较运算符 ===");
    test_code(
        r#"
        int test() {
            var x = 10
            var y = 20
            
            if x < y {
                return 1
            }
            return 0
        }
    "#,
    );

    println!("\n=== 测试2: 逻辑运算符 ===");
    test_code(
        r#"
        int test() {
            var x = 10
            var y = 20
            
            if x < y && y > 5 {
                return 1
            }
            return 0
        }
    "#,
    );

    println!("\n=== 测试3: 一元 not ===");
    test_code(
        r#"
        int test() {
            var flag = true
            if !flag {
                return 0
            }
            return 1
        }
    "#,
    );

    println!("\n=== 测试4: 等于/不等于 ===");
    test_code(
        r#"
        int test() {
            var x = 10
           if x == 10 {
                return 1
            }
            return 0
        }
    "#,
    );

    println!("\n=== 测试5: 复杂表达式 ===");
    test_code(
        r#"
        int compare(int a, int b) {
            if a > b || (a == b && b >= 0) {
                return 1
            }
            return -1
        }
    "#,
    );
}

fn test_code(source: &str) {
    match compile(source) {
        Ok(output) => {
            println!("✓ 编译成功!");
            // 只显示前200个字符
            let preview = &output.ir[..output.ir.len().min(200)];
            println!("IR preview: {}", preview);
        }
        Err(e) => {
            println!("✗ 编译失败: {}", e);
        }
    }
}
