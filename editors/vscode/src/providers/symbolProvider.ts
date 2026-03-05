import * as vscode from 'vscode';

import { collectSymbols } from '../core/symbols';

export class LencyDocumentSymbolProvider implements vscode.DocumentSymbolProvider {
    provideDocumentSymbols(document: vscode.TextDocument): vscode.DocumentSymbol[] {
        return collectSymbols(document).map(symbol => {
            return new vscode.DocumentSymbol(
                symbol.name,
                '',
                symbol.kind,
                symbol.range,
                symbol.range
            );
        });
    }
}
