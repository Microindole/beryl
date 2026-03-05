import * as vscode from 'vscode';

import { BUILTIN_SPECS, KEYWORDS } from '../core/language';

export class LencyCompletionProvider implements vscode.CompletionItemProvider {
    provideCompletionItems(_document: vscode.TextDocument, _position: vscode.Position): vscode.CompletionItem[] {
        const completions: vscode.CompletionItem[] = [];

        for (const keyword of KEYWORDS) {
            const item = new vscode.CompletionItem(keyword, vscode.CompletionItemKind.Keyword);
            completions.push(item);
        }

        for (const [func, spec] of Object.entries(BUILTIN_SPECS)) {
            const item = new vscode.CompletionItem(func, vscode.CompletionItemKind.Function);
            item.detail = 'Lency Built-in';
            item.documentation = new vscode.MarkdownString(spec.markdown);
            item.insertText = new vscode.SnippetString(`${func}($0)`);
            completions.push(item);
        }

        return completions;
    }
}
