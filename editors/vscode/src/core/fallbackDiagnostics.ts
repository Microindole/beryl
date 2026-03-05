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

export function registerFallbackDiagnostics(context: vscode.ExtensionContext): vscode.Disposable {
    const localDiagnostics = vscode.languages.createDiagnosticCollection('lency-local');

    const refresh = (doc: vscode.TextDocument): void => {
        if (doc.languageId !== 'lency') {
            return;
        }
        validateBraces(doc, localDiagnostics);
    };

    const openDisposable = vscode.workspace.onDidOpenTextDocument(refresh);
    const changeDisposable = vscode.workspace.onDidChangeTextDocument(e => refresh(e.document));
    const saveDisposable = vscode.workspace.onDidSaveTextDocument(refresh);
    const closeDisposable = vscode.workspace.onDidCloseTextDocument(doc => localDiagnostics.delete(doc.uri));

    for (const document of vscode.workspace.textDocuments) {
        refresh(document);
    }

    const disposable = vscode.Disposable.from(
        localDiagnostics,
        openDisposable,
        changeDisposable,
        saveDisposable,
        closeDisposable
    );
    context.subscriptions.push(disposable);
    return disposable;
}
