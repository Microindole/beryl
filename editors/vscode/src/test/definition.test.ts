/* eslint-disable @typescript-eslint/no-require-imports */
/* eslint-disable @typescript-eslint/no-explicit-any */

import * as assert from 'assert';
import { test, makeDoc, makePosition } from './helpers';

export async function run(): Promise<void> {
    console.log('\n[definition]');

    await test('var foo 声明，光标在使用处跳转到声明行', async () => {
        const { LencyDefinitionProvider } = require('../providers/definitionProvider');
        const provider = new LencyDefinitionProvider();
        const doc = makeDoc(['var foo = 1', 'print(foo)']);
        const loc = await provider.provideDefinition(doc, makePosition(1, 7));
        assert.ok(loc !== null, '期望返回 Location');
        assert.strictEqual(loc.range.line, 0, `期望跳到第0行，实际: ${loc.range.line}`);
    });

    await test('未定义符号返回 null', async () => {
        const { LencyDefinitionProvider } = require('../providers/definitionProvider');
        const provider = new LencyDefinitionProvider();
        const doc = makeDoc(['print(bar)']);
        const loc = await provider.provideDefinition(doc, makePosition(0, 7));
        assert.strictEqual(loc, null);
    });
}
