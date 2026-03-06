/**
 * 测试共享工具：mock 注入、fake TextDocument、运行器。
 */

/* eslint-disable @typescript-eslint/no-require-imports */
/* eslint-disable @typescript-eslint/no-explicit-any */

import * as path from 'path';

// ---------- mock-vscode 注入 ----------
export const mockPath = path.resolve(__dirname, 'mock-vscode.js');
export const mock = require(mockPath);

const fakeVscodeModule = {
    id: 'vscode', filename: 'vscode', loaded: true,
    exports: mock, parent: null, children: [] as any[], paths: [] as string[],
};
require.cache['vscode'] = fakeVscodeModule as any;

try {
    const Module = require('module');
    const original = Module._resolveFilename;
    Object.defineProperty(Module, '_resolveFilename', {
        writable: true, configurable: true,
        value: (request: string, ...args: unknown[]) => {
            if (request === 'vscode') { return 'vscode'; }
            return original.call(Module, request, ...args);
        }
    });
} catch { /* 依赖 require.cache 直接命中 */ }

// ---------- 工具函数 ----------

export function makeDoc(lines: string[], uri: string = 'file:///test.lcy'): any {
    const fullText = lines.join('\n');
    return {
        uri,
        languageId: 'lency',
        lineCount: lines.length,
        lineAt: (n: number) => ({ text: lines[n], range: makeRange(n, 0, n, lines[n].length) }),
        getText: (range?: any) => {
            if (!range) { return fullText; }
            const line = lines[range.start.line] ?? '';
            return line.substring(range.start.character, range.end.character);
        },
        getWordRangeAtPosition: (pos: any) => {
            const line = lines[pos.line] ?? '';
            const wordRegex = /[a-zA-Z_][a-zA-Z0-9_]*/g;
            let m: RegExpExecArray | null;
            while ((m = wordRegex.exec(line)) !== null) {
                if (m.index <= pos.character && pos.character <= m.index + m[0].length) {
                    return makeRange(pos.line, m.index, pos.line, m.index + m[0].length);
                }
            }
            return undefined;
        },
        positionAt: (offset: number) => {
            let remaining = offset;
            for (let i = 0; i < lines.length; i++) {
                const len = lines[i].length + 1;
                if (remaining < len) { return new mock.Position(i, remaining); }
                remaining -= len;
            }
            return new mock.Position(lines.length - 1, lines[lines.length - 1].length);
        }
    };
}

export function makeRange(sl: number, sc: number, el: number, ec: number): any {
    return new mock.Range(new mock.Position(sl, sc), new mock.Position(el, ec));
}

export function makePosition(line: number, character: number): any {
    return new mock.Position(line, character);
}

// ---------- 测试运行器 ----------

export let passed = 0;
export let failed = 0;

export function resetCounters(): void { passed = 0; failed = 0; }

export async function test(name: string, fn: () => void | Promise<void>): Promise<void> {
    return Promise.resolve(fn()).then(() => {
        console.log(`  [PASS] ${name}`);
        passed++;
    }).catch((err: unknown) => {
        console.error(`  [FAIL] ${name}`);
        console.error(`         ${err instanceof Error ? err.message : String(err)}`);
        failed++;
    });
}
