import * as vscode from 'vscode';

import { collectSymbols } from '../core/symbols';
import { isIdentifier } from '../core/text';

export class LencyDefinitionProvider implements vscode.DefinitionProvider {
    provideDefinition(document: vscode.TextDocument, position: vscode.Position): vscode.ProviderResult<vscode.Definition> {
        const range = document.getWordRangeAtPosition(position);
        if (!range) {
            return null;
        }
        const word = document.getText(range);
        if (!isIdentifier(word)) {
            return null;
        }

        const symbols = collectSymbols(document)
            .filter(symbol => symbol.name === word)
            .sort((a, b) => a.range.start.line - b.range.start.line);

        if (symbols.length === 0) {
            return null;
        }

        const prior = symbols.filter(symbol => symbol.range.start.isBeforeOrEqual(position));
        const target = prior.length > 0 ? prior[prior.length - 1] : symbols[0];
        return new vscode.Location(document.uri, target.range.start);
    }
}
