/**
 * 轻量 vscode mock，仅供回归测试使用。
 * 仅实现测试所需的纯数据类和最小桩，不引入任何额外依赖。
 */

export class Position {
    constructor(public readonly line: number, public readonly character: number) { }

    isBeforeOrEqual(other: Position): boolean {
        if (this.line !== other.line) {
            return this.line < other.line;
        }
        return this.character <= other.character;
    }

    translate(lineDelta: number, characterDelta: number): Position {
        return new Position(this.line + lineDelta, this.character + characterDelta);
    }
}

export class Range {
    public readonly start: Position;
    public readonly end: Position;

    constructor(
        startOrStartLine: Position | number,
        startCharacterOrEnd: Position | number,
        endLine?: number,
        endCharacter?: number
    ) {
        if (typeof startOrStartLine === 'number') {
            this.start = new Position(startOrStartLine, startCharacterOrEnd as number);
            this.end = new Position(endLine!, endCharacter!);
        } else {
            this.start = startOrStartLine;
            this.end = startCharacterOrEnd as Position;
        }
    }
}

export class Location {
    constructor(
        public readonly uri: unknown,
        public readonly range: Position | Range
    ) { }
}

export enum DiagnosticSeverity {
    Error = 0,
    Warning = 1,
    Information = 2,
    Hint = 3
}

export class Diagnostic {
    constructor(
        public readonly range: Range,
        public readonly message: string,
        public readonly severity: DiagnosticSeverity = DiagnosticSeverity.Error
    ) { }
}

export class MarkdownString {
    constructor(public readonly value: string = '') { }
}

export class SnippetString {
    constructor(public readonly value: string = '') { }
}

export enum SymbolKind {
    File = 0,
    Module = 1,
    Namespace = 2,
    Package = 3,
    Class = 4,
    Method = 5,
    Property = 6,
    Field = 7,
    Constructor = 8,
    Enum = 9,
    Interface = 10,
    Function = 11,
    Variable = 12,
    Constant = 13,
    String = 14,
    Number = 15,
    Boolean = 16,
    Array = 17,
    Object = 18,
    Key = 19,
    Null = 20,
    EnumMember = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
}

export enum CompletionItemKind {
    Keyword = 14,
    Function = 2,
    Variable = 5
}

export class CompletionItem {
    public detail?: string;
    public documentation?: MarkdownString;
    public insertText?: SnippetString;
    constructor(public readonly label: string, public readonly kind?: CompletionItemKind) { }
}

export class ParameterInformation {
    constructor(public readonly label: string) { }
}

export class SignatureInformation {
    public parameters: ParameterInformation[] = [];
    constructor(public readonly label: string, public readonly documentation?: MarkdownString) { }
}

export class SignatureHelp {
    public signatures: SignatureInformation[] = [];
    public activeSignature: number = 0;
    public activeParameter: number = 0;
}

export class WorkspaceEdit {
    private readonly _edits: Array<{ uri: unknown; range: Range; newText: string }> = [];

    replace(uri: unknown, range: Range, newText: string): void {
        this._edits.push({ uri, range, newText });
    }

    entries(): Array<{ uri: unknown; range: Range; newText: string }> {
        return this._edits;
    }

    get size(): number {
        return this._edits.length;
    }
}

export type DiagnosticCollectionStore = Map<unknown, Diagnostic[]>;

export class DiagnosticCollection {
    private readonly _store: DiagnosticCollectionStore = new Map();

    set(uri: unknown, diagnostics: Diagnostic[]): void {
        this._store.set(uri, diagnostics);
    }

    get(uri: unknown): Diagnostic[] {
        return this._store.get(uri) ?? [];
    }

    delete(uri: unknown): void {
        this._store.delete(uri);
    }
}

// 桩：测试中不直接调用，仅保证 import 不报错。
export const workspace = {
    findFiles: async (_pattern: string) => [] as unknown[],
    openTextDocument: async (_uri: unknown) => null as unknown,
    textDocuments: [] as unknown[],
    onDidOpenTextDocument: () => ({ dispose: () => undefined }),
    onDidChangeTextDocument: () => ({ dispose: () => undefined }),
    onDidSaveTextDocument: () => ({ dispose: () => undefined }),
    onDidCloseTextDocument: () => ({ dispose: () => undefined }),
    onDidChangeConfiguration: () => ({ dispose: () => undefined }),
    getConfiguration: (_section: string) => ({ get: (_key: string) => undefined }),
    workspaceFolders: undefined,
    createFileSystemWatcher: (_pattern: string) => ({ dispose: () => undefined }),
};

export const window = {
    createStatusBarItem: () => ({ text: '', tooltip: '', show: () => undefined, dispose: () => undefined }),
    showWarningMessage: async (_msg: string) => undefined,
};

export const languages = {
    createDiagnosticCollection: (_name: string) => new DiagnosticCollection(),
    registerDocumentSymbolProvider: () => ({ dispose: () => undefined }),
    registerHoverProvider: () => ({ dispose: () => undefined }),
    registerDocumentHighlightProvider: () => ({ dispose: () => undefined }),
    registerCompletionItemProvider: () => ({ dispose: () => undefined }),
    registerSignatureHelpProvider: () => ({ dispose: () => undefined }),
    registerDocumentFormattingEditProvider: () => ({ dispose: () => undefined }),
    registerDefinitionProvider: () => ({ dispose: () => undefined }),
    registerRenameProvider: () => ({ dispose: () => undefined }),
};

export const StatusBarAlignment = { Left: 1, Right: 2 };

export const Disposable = {
    from: (..._items: unknown[]) => ({ dispose: () => undefined }),
};
