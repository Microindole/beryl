import * as vscode from 'vscode';

export function validateBraces(document: vscode.TextDocument, collection: vscode.DiagnosticCollection): void {
    const diagnostics: vscode.Diagnostic[] = [];
    const stack: Array<{ char: string; pos: vscode.Position }> = [];
    const pairs: Record<string, string> = { ')': '(', ']': '[', '}': '{' };

    for (let lineNumber = 0; lineNumber < document.lineCount; lineNumber++) {
        const line = document.lineAt(lineNumber).text;
        for (let i = 0; i < line.length; i++) {
            const ch = line[i];
            if (ch === '{' || ch === '[' || ch === '(') {
                stack.push({ char: ch, pos: new vscode.Position(lineNumber, i) });
                continue;
            }
            if (!(ch in pairs)) {
                continue;
            }
            const top = stack.pop();
            if (!top || top.char !== pairs[ch]) {
                diagnostics.push(new vscode.Diagnostic(
                    new vscode.Range(lineNumber, i, lineNumber, i + 1),
                    `括号不匹配: '${ch}'`,
                    vscode.DiagnosticSeverity.Error
                ));
            }
        }
    }

    while (stack.length > 0) {
        const item = stack.pop();
        if (!item) {
            break;
        }
        diagnostics.push(new vscode.Diagnostic(
            new vscode.Range(item.pos, item.pos.translate(0, 1)),
            `括号未闭合: '${item.char}'`,
            vscode.DiagnosticSeverity.Error
        ));
    }

    collection.set(document.uri, diagnostics);
}

export function registerFallbackDiagnostics(context: vscode.ExtensionContext): void {
    const localDiagnostics = vscode.languages.createDiagnosticCollection('lency-local');
    context.subscriptions.push(localDiagnostics);

    const refresh = (doc: vscode.TextDocument): void => {
        if (doc.languageId !== 'lency') {
            return;
        }
        validateBraces(doc, localDiagnostics);
    };

    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument(refresh),
        vscode.workspace.onDidChangeTextDocument(e => refresh(e.document)),
        vscode.workspace.onDidSaveTextDocument(refresh),
        vscode.workspace.onDidCloseTextDocument(doc => localDiagnostics.delete(doc.uri))
    );

    for (const document of vscode.workspace.textDocuments) {
        refresh(document);
    }
}
