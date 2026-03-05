import * as vscode from 'vscode';

import { BUILTIN_DOCS } from '../core/language';

export class LencyHoverProvider implements vscode.HoverProvider {
    provideHover(document: vscode.TextDocument, position: vscode.Position): vscode.Hover | null {
        const range = document.getWordRangeAtPosition(position);
        if (!range) {
            return null;
        }
        const word = document.getText(range);
        const markdown = BUILTIN_DOCS[word];
        if (!markdown) {
            return null;
        }
        return new vscode.Hover(new vscode.MarkdownString(markdown));
    }
}
