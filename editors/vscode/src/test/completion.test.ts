/* eslint-disable @typescript-eslint/no-require-imports */
/* eslint-disable @typescript-eslint/no-explicit-any */

import * as assert from 'assert';
import { test, makeDoc, makePosition } from './helpers';

export async function run(): Promise<void> {
    console.log('[completion]');

    await test('包含关键字 var/if/return', () => {
        const { LencyCompletionProvider } = require('../providers/completionProvider');
        const provider = new LencyCompletionProvider();
        const items: any[] = provider.provideCompletionItems(makeDoc(['var x = 1']), makePosition(0, 0));
        const labels = items.map((i: any) => i.label);
        assert.ok(labels.includes('var'), `缺少 var`);
        assert.ok(labels.includes('if'), `缺少 if`);
        assert.ok(labels.includes('return'), `缺少 return`);
    });

    await test('包含内置函数 print/read_file', () => {
        const { LencyCompletionProvider } = require('../providers/completionProvider');
        const provider = new LencyCompletionProvider();
        const items: any[] = provider.provideCompletionItems(makeDoc(['']), makePosition(0, 0));
        const labels = items.map((i: any) => i.label);
        assert.ok(labels.includes('print'), `缺少 print`);
        assert.ok(labels.includes('read_file'), `缺少 read_file`);
    });

    await test('包含新增内置函数 char_to_string/panic/format', () => {
        const { LencyCompletionProvider } = require('../providers/completionProvider');
        const provider = new LencyCompletionProvider();
        const items: any[] = provider.provideCompletionItems(makeDoc(['']), makePosition(0, 0));
        const labels = items.map((i: any) => i.label);
        assert.ok(labels.includes('char_to_string'), `缺少 char_to_string`);
        assert.ok(labels.includes('panic'), `缺少 panic`);
        assert.ok(labels.includes('format'), `缺少 format`);
    });

    await test('包含 vec/Ok/Err 关键字', () => {
        const { LencyCompletionProvider } = require('../providers/completionProvider');
        const provider = new LencyCompletionProvider();
        const items: any[] = provider.provideCompletionItems(makeDoc(['']), makePosition(0, 0));
        const labels = items.map((i: any) => i.label);
        assert.ok(labels.includes('vec'), `缺少 vec`);
        assert.ok(labels.includes('Ok'), `缺少 Ok`);
        assert.ok(labels.includes('Err'), `缺少 Err`);
    });
}
