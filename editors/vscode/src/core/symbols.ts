import * as vscode from 'vscode';

import { SimpleSymbol } from './types';

export function collectSymbols(document: vscode.TextDocument): SimpleSymbol[] {
    const symbols: SimpleSymbol[] = [];

    for (let lineNumber = 0; lineNumber < document.lineCount; lineNumber++) {
        const lineText = document.lineAt(lineNumber).text;

        const declarations: Array<{ regex: RegExp; kind: vscode.SymbolKind }> = [
            { regex: /^\s*(?:struct|enum)\s+([a-zA-Z_][a-zA-Z0-9_]*)\b/, kind: vscode.SymbolKind.Struct },
            { regex: /^\s*trait\s+([a-zA-Z_][a-zA-Z0-9_]*)\b/, kind: vscode.SymbolKind.Interface },
            { regex: /^\s*impl\s+([a-zA-Z_][a-zA-Z0-9_]*)\b/, kind: vscode.SymbolKind.Interface },
            { regex: /^\s*(?:void|int|float|bool|string|[A-Z][a-zA-Z0-9_]*)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/, kind: vscode.SymbolKind.Function },
            { regex: /^\s*(?:var|const)\s+([a-zA-Z_][a-zA-Z0-9_]*)\b/, kind: vscode.SymbolKind.Variable }
        ];

        for (const declaration of declarations) {
            const match = lineText.match(declaration.regex);
            if (!match) {
                continue;
            }
            const name = match[1];
            const startChar = lineText.indexOf(name);
            if (startChar < 0) {
                continue;
            }
            symbols.push({
                name,
                kind: declaration.kind,
                range: new vscode.Range(lineNumber, startChar, lineNumber, startChar + name.length)
            });
            break;
        }
    }

    return symbols;
}
