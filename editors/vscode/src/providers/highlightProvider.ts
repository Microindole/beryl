import * as vscode from 'vscode';

import { escapeRegExp, isIdentifier } from '../core/text';

export class LencyDocumentHighlightProvider implements vscode.DocumentHighlightProvider {
    provideDocumentHighlights(document: vscode.TextDocument, position: vscode.Position): vscode.DocumentHighlight[] {
        const range = document.getWordRangeAtPosition(position);
        if (!range) {
            return [];
        }

        const word = document.getText(range);
        if (!isIdentifier(word)) {
            return [];
        }

        const escaped = escapeRegExp(word);
        const regex = new RegExp(`\\b${escaped}\\b`, 'g');
        const text = document.getText();
        const highlights: vscode.DocumentHighlight[] = [];

        let match: RegExpExecArray | null;
        while ((match = regex.exec(text)) !== null) {
            const startPos = document.positionAt(match.index);
            const endPos = document.positionAt(match.index + word.length);
            highlights.push(new vscode.DocumentHighlight(
                new vscode.Range(startPos, endPos),
                vscode.DocumentHighlightKind.Text
            ));
        }

        return highlights;
    }
}
