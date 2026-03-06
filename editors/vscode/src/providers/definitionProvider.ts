import * as vscode from 'vscode';

import { collectSymbols } from '../core/symbols';
import { isIdentifier } from '../core/text';

export class LencyDefinitionProvider implements vscode.DefinitionProvider {
    async provideDefinition(document: vscode.TextDocument, position: vscode.Position): Promise<vscode.Definition | null> {
        const range = document.getWordRangeAtPosition(position);
        if (!range) {
            return null;
        }
        const word = document.getText(range);
        if (!isIdentifier(word)) {
            return null;
        }

        // 优先在当前文件中查找声明。
        const localSymbols = collectSymbols(document)
            .filter(symbol => symbol.name === word)
            .sort((a, b) => a.range.start.line - b.range.start.line);

        if (localSymbols.length > 0) {
            const prior = localSymbols.filter(symbol => symbol.range.start.isBeforeOrEqual(position));
            const target = prior.length > 0 ? prior[prior.length - 1] : localSymbols[0];
            return new vscode.Location(document.uri, target.range.start);
        }

        // 当前文件未找到时，遍历工作区所有 .lcy 文件。
        const uris = await vscode.workspace.findFiles('**/*.lcy');
        for (const uri of uris) {
            if (uri.toString() === document.uri.toString()) {
                continue;
            }
            const otherDoc = await vscode.workspace.openTextDocument(uri);
            const symbols = collectSymbols(otherDoc)
                .filter(symbol => symbol.name === word)
                .sort((a, b) => a.range.start.line - b.range.start.line);
            if (symbols.length > 0) {
                return new vscode.Location(uri, symbols[0].range.start);
            }
        }

        return null;
    }
}

