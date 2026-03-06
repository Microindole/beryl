/* eslint-disable @typescript-eslint/no-require-imports */
/* eslint-disable @typescript-eslint/no-explicit-any */

import * as assert from 'assert';
import { test, makeDoc, makePosition } from './helpers';

export async function run(): Promise<void> {
    console.log('\n[signature]');

    await test('print( 触发签名，activeParameter=0', () => {
        const { LencySignatureHelpProvider } = require('../providers/signatureProvider');
        const provider = new LencySignatureHelpProvider();
        const help = provider.provideSignatureHelp(makeDoc(['print(']), makePosition(0, 6));
        assert.ok(help !== null, '期望返回 SignatureHelp');
        assert.strictEqual(help.signatures[0].label, 'print(msg)');
        assert.strictEqual(help.activeParameter, 0);
    });

    await test('write_file( 第二个参数时 activeParameter=1', () => {
        const { LencySignatureHelpProvider } = require('../providers/signatureProvider');
        const provider = new LencySignatureHelpProvider();
        const help = provider.provideSignatureHelp(makeDoc(['write_file("a.txt",']), makePosition(0, 19));
        assert.ok(help !== null, '期望返回 SignatureHelp');
        assert.strictEqual(help.activeParameter, 1);
    });

    await test('无 ( 时返回 null', () => {
        const { LencySignatureHelpProvider } = require('../providers/signatureProvider');
        const provider = new LencySignatureHelpProvider();
        const help = provider.provideSignatureHelp(makeDoc(['var x = 1']), makePosition(0, 5));
        assert.strictEqual(help, null);
    });

    await test('char_to_string( 触发签名', () => {
        const { LencySignatureHelpProvider } = require('../providers/signatureProvider');
        const provider = new LencySignatureHelpProvider();
        const help = provider.provideSignatureHelp(makeDoc(['char_to_string(']), makePosition(0, 15));
        assert.ok(help !== null, '期望返回 SignatureHelp');
        assert.ok(help.signatures[0].label.includes('char_to_string'));
    });

    await test('format( 触发签名', () => {
        const { LencySignatureHelpProvider } = require('../providers/signatureProvider');
        const provider = new LencySignatureHelpProvider();
        const help = provider.provideSignatureHelp(makeDoc(['format(']), makePosition(0, 7));
        assert.ok(help !== null, '期望返回 SignatureHelp');
        assert.ok(help.signatures[0].label.includes('format'));
    });
}
