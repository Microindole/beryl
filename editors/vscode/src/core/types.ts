import * as vscode from 'vscode';

export interface BuiltinSpec {
    signatureLabel: string;
    markdown: string;
    parameters: string[];
}

export interface SimpleSymbol {
    name: string;
    kind: vscode.SymbolKind;
    range: vscode.Range;
}
