import * as vscode from 'vscode';

/**
 * 将一行代码中字符串字面量内容和行注释部分替换为空格，
 * 仅保留代码结构部分，用于花括号缩进判断。
 *
 * Lency 语法约束：
 *   - 注释为 // 单行注释，无块注释。
 *   - 字符串为 "..." 双引号，支持 \" 转义，不跨行。
 */
function stripCommentsAndStrings(line: string): string {
    const chars = line.split('');
    let i = 0;
    while (i < chars.length) {
        if (chars[i] === '"') {
            chars[i] = ' ';
            i++;
            while (i < chars.length) {
                if (chars[i] === '\\') {
                    chars[i] = ' ';
                    i++;
                    if (i < chars.length) {
                        chars[i] = ' ';
                        i++;
                    }
                } else if (chars[i] === '"') {
                    chars[i] = ' ';
                    i++;
                    break;
                } else {
                    chars[i] = ' ';
                    i++;
                }
            }
        } else if (chars[i] === '/' && i + 1 < chars.length && chars[i + 1] === '/') {
            for (let j = i; j < chars.length; j++) {
                chars[j] = ' ';
            }
            break;
        } else {
            i++;
        }
    }
    return chars.join('');
}

export class LencyFormattingProvider implements vscode.DocumentFormattingEditProvider {
    provideDocumentFormattingEdits(document: vscode.TextDocument): vscode.TextEdit[] {
        const edits: vscode.TextEdit[] = [];
        let indentLevel = 0;
        const tabSize = 4;

        for (let i = 0; i < document.lineCount; i++) {
            const line = document.lineAt(i);
            const trimmed = line.text.trim();

            if (trimmed.length === 0) {
                continue;
            }

            const stripped = stripCommentsAndStrings(trimmed);
            const strippedTrimmed = stripped.trim();

            if (strippedTrimmed.startsWith('}')) {
                indentLevel = Math.max(0, indentLevel - 1);
            }

            const expectedIndent = ' '.repeat(indentLevel * tabSize);
            const expectedLine = expectedIndent + trimmed;
            if (line.text !== expectedLine) {
                edits.push(vscode.TextEdit.replace(line.range, expectedLine));
            }

            if (strippedTrimmed[strippedTrimmed.length - 1] === '{') {
                indentLevel++;
            }
        }

        return edits;
    }
}

