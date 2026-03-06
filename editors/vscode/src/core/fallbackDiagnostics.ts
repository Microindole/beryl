import * as vscode from 'vscode';

const FULLWIDTH_CHARS = '，。；：\u201c\u201d\u2018\u2019（）【】！？';
const FULLWIDTH_HALF: Record<string, string> = {
    '，': ',', '。': '.', '；': ';', '：': ':',
    '\u201c': '"', '\u201d': '"', '\u2018': "'", '\u2019': "'",
    '（': '(', '）': ')', '【': '[', '】': ']', '！': '!', '？': '?'
};

/** 剥离行中字符串和注释内容（用空格占位），供诊断扫描使用。 */
function stripForScan(line: string): string {
    const chars = line.split('');
    let i = 0;
    while (i < chars.length) {
        if (chars[i] === '"') {
            chars[i] = ' ';
            i++;
            while (i < chars.length) {
                if (chars[i] === '\\') {
                    chars[i] = ' ';
                    i++;
                    if (i < chars.length) { chars[i] = ' '; i++; }
                } else if (chars[i] === '"') {
                    chars[i] = ' ';
                    i++;
                    break;
                } else {
                    chars[i] = ' ';
                    i++;
                }
            }
        } else if (chars[i] === '/' && i + 1 < chars.length && chars[i + 1] === '/') {
            for (let j = i; j < chars.length; j++) { chars[j] = ' '; }
            break;
        } else {
            i++;
        }
    }
    return chars.join('');
}

/** 括号匹配诊断。返回 Diagnostic 列表。 */
export function validateBraces(document: vscode.TextDocument): vscode.Diagnostic[] {
    const diagnostics: vscode.Diagnostic[] = [];
    const stack: Array<{ char: string; pos: vscode.Position }> = [];
    const pairs: Record<string, string> = { ')': '(', ']': '[', '}': '{' };

    for (let lineNumber = 0; lineNumber < document.lineCount; lineNumber++) {
        const line = stripForScan(document.lineAt(lineNumber).text);
        for (let i = 0; i < line.length; i++) {
            const ch = line[i];
            if (ch === '{' || ch === '[' || ch === '(') {
                stack.push({ char: ch, pos: new vscode.Position(lineNumber, i) });
                continue;
            }
            if (!(ch in pairs)) { continue; }
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
        if (!item) { break; }
        diagnostics.push(new vscode.Diagnostic(
            new vscode.Range(item.pos, item.pos.translate(0, 1)),
            `括号未闭合: '${item.char}'`,
            vscode.DiagnosticSeverity.Error
        ));
    }

    return diagnostics;
}

/** 未闭合字符串诊断。返回 Diagnostic 列表。 */
export function validateUnclosedStrings(document: vscode.TextDocument): vscode.Diagnostic[] {
    const diagnostics: vscode.Diagnostic[] = [];

    for (let lineNumber = 0; lineNumber < document.lineCount; lineNumber++) {
        const line = document.lineAt(lineNumber).text;
        let inString = false;
        let stringStart = -1;
        let i = 0;

        while (i < line.length) {
            if (!inString && line[i] === '/' && i + 1 < line.length && line[i + 1] === '/') {
                break;
            }
            if (line[i] === '\\' && inString) {
                i += 2;
                continue;
            }
            if (line[i] === '"') {
                if (inString) {
                    inString = false;
                    stringStart = -1;
                } else {
                    inString = true;
                    stringStart = i;
                }
            }
            i++;
        }

        if (inString && stringStart >= 0) {
            diagnostics.push(new vscode.Diagnostic(
                new vscode.Range(lineNumber, stringStart, lineNumber, line.length),
                '字符串未闭合',
                vscode.DiagnosticSeverity.Error
            ));
        }
    }

    return diagnostics;
}

/** 全角/中文标点误用诊断。返回 Diagnostic 列表。 */
export function validateFullwidthPunctuation(document: vscode.TextDocument): vscode.Diagnostic[] {
    const diagnostics: vscode.Diagnostic[] = [];

    for (let lineNumber = 0; lineNumber < document.lineCount; lineNumber++) {
        const line = document.lineAt(lineNumber).text;
        const stripped = stripForScan(line);

        for (let i = 0; i < stripped.length; i++) {
            const ch = stripped[i];
            if (FULLWIDTH_CHARS.includes(ch)) {
                const half = FULLWIDTH_HALF[ch] ?? '?';
                diagnostics.push(new vscode.Diagnostic(
                    new vscode.Range(lineNumber, i, lineNumber, i + 1),
                    `疑似全角标点 '${ch}'，请改用半角 '${half}'`,
                    vscode.DiagnosticSeverity.Warning
                ));
            }
        }
    }

    return diagnostics;
}

export function registerFallbackDiagnostics(context: vscode.ExtensionContext): vscode.Disposable {
    const localDiagnostics = vscode.languages.createDiagnosticCollection('lency-local');

    const refresh = (doc: vscode.TextDocument): void => {
        if (doc.languageId !== 'lency') { return; }
        const diagnostics = [
            ...validateBraces(doc),
            ...validateUnclosedStrings(doc),
            ...validateFullwidthPunctuation(doc),
        ];
        localDiagnostics.set(doc.uri, diagnostics);
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
