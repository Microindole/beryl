import * as vscode from 'vscode';

import { KEYWORDS } from '../core/language';
import { escapeRegExp, isIdentifier } from '../core/text';

export class LencyRenameProvider implements vscode.RenameProvider {
    prepareRename(document: vscode.TextDocument, position: vscode.Position): vscode.ProviderResult<vscode.Range> {
        const range = document.getWordRangeAtPosition(position);
        if (!range) {
            return null;
        }
        const word = document.getText(range);
        if (!isIdentifier(word) || KEYWORDS.has(word)) {
            throw new Error('当前标识符不可重命名');
        }
        return range;
    }

    provideRenameEdits(document: vscode.TextDocument, position: vscode.Position, newName: string): vscode.ProviderResult<vscode.WorkspaceEdit> {
        const range = document.getWordRangeAtPosition(position);
        if (!range) {
            return new vscode.WorkspaceEdit();
        }

        const word = document.getText(range);
        if (!isIdentifier(word) || !isIdentifier(newName)) {
            throw new Error('仅支持合法标识符重命名');
        }

        const workspaceEdit = new vscode.WorkspaceEdit();
        const escaped = escapeRegExp(word);
        const regex = new RegExp(`\\b${escaped}\\b`, 'g');
        const text = document.getText();

        let match: RegExpExecArray | null;
        while ((match = regex.exec(text)) !== null) {
            const startPos = document.positionAt(match.index);
            const endPos = document.positionAt(match.index + word.length);
            workspaceEdit.replace(document.uri, new vscode.Range(startPos, endPos), newName);
        }

        // TODO: 后续迁移到 LSP 全项目符号索引后，支持跨文件安全重命名。
        return workspaceEdit;
    }
}
