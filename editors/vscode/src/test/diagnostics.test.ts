/* eslint-disable @typescript-eslint/no-require-imports */
/* eslint-disable @typescript-eslint/no-explicit-any */

import * as assert from 'assert';
import { test, makeDoc, mock } from './helpers';

export async function run(): Promise<void> {
    console.log('\n[diagnostics - braces]');

    await test('{ 未闭合时生成 Error 诊断', () => {
        const { validateBraces } = require('../core/fallbackDiagnostics');
        const diags = validateBraces(makeDoc(['void main() {', '    var x = 1']));
        assert.ok(diags.length > 0, '期望有诊断');
        assert.strictEqual(diags[0].severity, mock.DiagnosticSeverity.Error);
    });

    await test('括号匹配时无诊断', () => {
        const { validateBraces } = require('../core/fallbackDiagnostics');
        const diags = validateBraces(makeDoc(['void main() {', '    var x = 1', '}']));
        assert.strictEqual(diags.length, 0, `期望无诊断，实际: ${diags.length}`);
    });

    await test(') 无对应 ( 时生成 Error 诊断', () => {
        const { validateBraces } = require('../core/fallbackDiagnostics');
        const diags = validateBraces(makeDoc(['var x = 1)']));
        assert.ok(diags.length > 0, '期望有诊断');
    });

    await test('字符串内括号不触发诊断', () => {
        const { validateBraces } = require('../core/fallbackDiagnostics');
        const diags = validateBraces(makeDoc(['var s = "hello {world}"']));
        assert.strictEqual(diags.length, 0, `字符串内花括号不应触发，实际: ${diags.length}`);
    });

    console.log('\n[diagnostics - unclosed strings]');

    await test('未闭合字符串生成 Error 诊断', () => {
        const { validateUnclosedStrings } = require('../core/fallbackDiagnostics');
        const diags = validateUnclosedStrings(makeDoc(['var s = "hello']));
        assert.ok(diags.length > 0, '期望有诊断');
        assert.strictEqual(diags[0].severity, mock.DiagnosticSeverity.Error);
        assert.ok(diags[0].message.includes('未闭合'));
    });

    await test('正常闭合字符串无诊断', () => {
        const { validateUnclosedStrings } = require('../core/fallbackDiagnostics');
        const diags = validateUnclosedStrings(makeDoc(['var s = "hello"']));
        assert.strictEqual(diags.length, 0, `期望无诊断，实际: ${diags.length}`);
    });

    await test('转义引号不触发未闭合诊断', () => {
        const { validateUnclosedStrings } = require('../core/fallbackDiagnostics');
        const diags = validateUnclosedStrings(makeDoc(['var s = "say \\"hi\\""']));
        assert.strictEqual(diags.length, 0, `转义引号不应误报，实际: ${diags.length}`);
    });

    console.log('\n[diagnostics - fullwidth]');

    await test('全角逗号生成 Warning 诊断', () => {
        const { validateFullwidthPunctuation } = require('../core/fallbackDiagnostics');
        const diags = validateFullwidthPunctuation(makeDoc(['print(a，b)']));
        assert.ok(diags.length > 0, '期望有诊断');
        assert.strictEqual(diags[0].severity, mock.DiagnosticSeverity.Warning);
    });

    await test('纯 ASCII 代码无全角诊断', () => {
        const { validateFullwidthPunctuation } = require('../core/fallbackDiagnostics');
        const diags = validateFullwidthPunctuation(makeDoc(['var x = 1', 'print(x)']));
        assert.strictEqual(diags.length, 0, `期望无诊断，实际: ${diags.length}`);
    });

    await test('字符串内全角标点不触发诊断', () => {
        const { validateFullwidthPunctuation } = require('../core/fallbackDiagnostics');
        const diags = validateFullwidthPunctuation(makeDoc(['var s = "你好，世界"']));
        assert.strictEqual(diags.length, 0, `字符串内全角不应触发，实际: ${diags.length}`);
    });
}
