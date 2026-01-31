# Lency 项目核心上下文 (Agent Root)

> [!IMPORTANT]
> **Agent 技能加载协议 (Skill Loading Protocol)**：
> 在开始任何工作前，请根据你的任务类型加载并遵循 `prompt/skills/` 下对应的技能文件：
> - **整体架构/准则/评审**：阅读 `prompt/skills/architect/SKILL.md`
> - **进度更新/Sprint 管理**：阅读 `prompt/skills/management/SKILL.md`
> - **代码开发/调试/后端**：阅读 `prompt/skills/compiler/SKILL.md`

## 项目概述
**Lency** 是一门静态类型、编译型语言，基于 LLVM 实现。设计哲学：「简洁、规范、清晰」，无“黑魔法”。

## 核心设计准则 (见 Architect Skill)
- **空安全**：默认非空，`T?` 表示可空。
- **无分号**：使用 `{}` 结构，行尾无 `;`。
- **显式优先**：禁止隐式类型转换和复杂的推理。

## 目录结构
```bash
crates/          # Rust 编译器组件 (Syntax, Sema, Codegen, Runtime)
prompt/          # Agent 职能与技能中心 (Skill Hub)
  skills/        # 模块化技能定义 (Architect, Management, Compiler)
assets/          # 语言蓝图与设计规范 (Blueprints)
lib/             # Lency 标准库 (.lcy)
tests/           # 集成测试集
scripts/         # 自动化检查与开发工具
```

## 关键工作流
1. **开发**：遵循对应项目的 `SKILL.md` 规范。
2. **验证**：运行 `./scripts/run_checks.sh --fast`。
3. **交付**：及时更新 `prompt/skills/management/resources/status.md`。

## 当前编译器状态
- ✅ 基础语法、泛型、Null安全、Enum、Vec、HashMap、Iterator
- ✅ 统一诊断系统 (lency_diagnostics)
- ⚠️ Result/Option 方法（正在补全）、panic 机制（待实现）

---
详细设计参考: [design_spec.md](../assets/design_spec.md), [Lency.txt](../assets/Lency.txt)
