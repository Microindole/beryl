import * as vscode from 'vscode';

import { LencyCompletionProvider } from './completionProvider';
import { LencyDefinitionProvider } from './definitionProvider';
import { LencyFormattingProvider } from './formattingProvider';
import { LencyDocumentHighlightProvider } from './highlightProvider';
import { LencyHoverProvider } from './hoverProvider';
import { LencyRenameProvider } from './renameProvider';
import { LencySignatureHelpProvider } from './signatureProvider';
import { LencyDocumentSymbolProvider } from './symbolProvider';

export function registerProviders(context: vscode.ExtensionContext, selector: vscode.DocumentSelector): void {
    context.subscriptions.push(
        vscode.languages.registerDocumentSymbolProvider(selector, new LencyDocumentSymbolProvider()),
        vscode.languages.registerHoverProvider(selector, new LencyHoverProvider()),
        vscode.languages.registerDocumentHighlightProvider(selector, new LencyDocumentHighlightProvider()),
        vscode.languages.registerCompletionItemProvider(selector, new LencyCompletionProvider(), '.', '<'),
        vscode.languages.registerSignatureHelpProvider(selector, new LencySignatureHelpProvider(), '(', ','),
        vscode.languages.registerDocumentFormattingEditProvider(selector, new LencyFormattingProvider()),
        vscode.languages.registerDefinitionProvider(selector, new LencyDefinitionProvider()),
        vscode.languages.registerRenameProvider(selector, new LencyRenameProvider())
    );
}
