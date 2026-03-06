/**
 * 测试主入口。
 * 运行方式：node dist/test/index.js
 */

// helpers 必须在所有 suite 之前 import 以确保 mock 注入
import './helpers';
import { passed, failed, resetCounters } from './helpers';
import { run as runCompletion } from './completion.test';
import { run as runSignature } from './signature.test';
import { run as runDefinition } from './definition.test';
import { run as runRename } from './rename.test';
import { run as runDiagnostics } from './diagnostics.test';

async function main(): Promise<void> {
    console.log('\n=== Lency Extension Regression Tests ===\n');
    resetCounters();

    await runCompletion();
    await runSignature();
    await runDefinition();
    await runRename();
    await runDiagnostics();

    console.log(`\n========================================`);
    console.log(`Results: ${passed} passed, ${failed} failed`);
    console.log(`========================================\n`);

    if (failed > 0) {
        process.exit(1);
    }
}

main().catch((err: unknown) => {
    console.error('测试运行器意外中止:', err);
    process.exit(1);
});
