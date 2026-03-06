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

    async provideRenameEdits(document: vscode.TextDocument, position: vscode.Position, newName: string): Promise<vscode.WorkspaceEdit> {
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

        const uris = await vscode.workspace.findFiles('**/*.lcy');
        for (const uri of uris) {
            const doc = await vscode.workspace.openTextDocument(uri);
            const text = doc.getText();
            let match: RegExpExecArray | null;
            while ((match = regex.exec(text)) !== null) {
                const startPos = doc.positionAt(match.index);
                const endPos = doc.positionAt(match.index + word.length);
                workspaceEdit.replace(uri, new vscode.Range(startPos, endPos), newName);
            }
        }

        return workspaceEdit;
    }
}

