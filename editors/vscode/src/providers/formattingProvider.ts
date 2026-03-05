import * as vscode from 'vscode';

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

            if (trimmed.startsWith('}')) {
                indentLevel = Math.max(0, indentLevel - 1);
            }

            const expectedIndent = ' '.repeat(indentLevel * tabSize);
            const expectedLine = expectedIndent + trimmed;
            if (line.text !== expectedLine) {
                edits.push(vscode.TextEdit.replace(line.range, expectedLine));
            }

            if (trimmed.endsWith('{')) {
                indentLevel++;
            }

            // FIXME: 当前格式化器未处理字符串/注释中的花括号，会在复杂行上误缩进。
        }

        return edits;
    }
}
