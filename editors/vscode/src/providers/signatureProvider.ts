import * as vscode from 'vscode';

import { BUILTIN_SPECS } from '../core/language';

export class LencySignatureHelpProvider implements vscode.SignatureHelpProvider {
    provideSignatureHelp(document: vscode.TextDocument, position: vscode.Position): vscode.SignatureHelp | null {
        const textBefore = document.getText(new vscode.Range(new vscode.Position(position.line, 0), position));
        const lastOpenParen = textBefore.lastIndexOf('(');
        if (lastOpenParen === -1) {
            return null;
        }

        const functionNameMatch = textBefore.substring(0, lastOpenParen).match(/([a-zA-Z_][a-zA-Z0-9_]*)\s*$/);
        if (!functionNameMatch) {
            return null;
        }

        const name = functionNameMatch[1];
        const spec = BUILTIN_SPECS[name];
        if (!spec) {
            return null;
        }

        const help = new vscode.SignatureHelp();
        const signature = new vscode.SignatureInformation(
            spec.signatureLabel,
            new vscode.MarkdownString(spec.markdown)
        );
        signature.parameters = spec.parameters.map(param => new vscode.ParameterInformation(param));

        const paramText = textBefore.substring(lastOpenParen + 1);
        const activeParam = (paramText.match(/,/g) || []).length;

        help.signatures = [signature];
        help.activeSignature = 0;
        help.activeParameter = Math.min(activeParam, Math.max(signature.parameters.length - 1, 0));
        return help;
    }
}
