/* eslint-disable @typescript-eslint/no-require-imports */
/* eslint-disable @typescript-eslint/no-explicit-any */

import * as assert from 'assert';
import { test, makeDoc, makePosition, mock } from './helpers';

export async function run(): Promise<void> {
    console.log('\n[rename]');

    await test('单文件 foo 出现 2 次，rename 生成 2 条编辑', async () => {
        const { LencyRenameProvider } = require('../providers/renameProvider');
        const provider = new LencyRenameProvider();
        const doc = makeDoc(['var foo = 1', 'print(foo)']);
        const origFindFiles = mock.workspace.findFiles;
        mock.workspace.findFiles = async () => [doc.uri];
        mock.workspace.openTextDocument = async () => doc;
        const edit = await provider.provideRenameEdits(doc, makePosition(0, 5), 'bar');
        assert.strictEqual(edit.size, 2, `期望 2 条编辑，实际: ${edit.size}`);
        mock.workspace.findFiles = origFindFiles;
    });

    await test('prepareRename 拒绝关键字 var', () => {
        const { LencyRenameProvider } = require('../providers/renameProvider');
        const provider = new LencyRenameProvider();
        assert.throws(() => provider.prepareRename(makeDoc(['var x = 1']), makePosition(0, 1)), /不可重命名/);
    });

    await test('this 不再是关键字，prepareRename 不拒绝', () => {
        const { LencyRenameProvider } = require('../providers/renameProvider');
        const provider = new LencyRenameProvider();
        assert.doesNotThrow(() => provider.prepareRename(makeDoc(['this.count = 1']), makePosition(0, 2)));
    });
}
