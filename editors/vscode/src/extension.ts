import * as vscode from 'vscode';
import { LanguageClient } from 'vscode-languageclient/node';

import { registerFallbackDiagnostics } from './core/fallbackDiagnostics';
import { startLanguageClient } from './core/lsp';
import { registerProviders } from './providers';

let client: LanguageClient | undefined;
let modeStatusBarItem: vscode.StatusBarItem | undefined;

function updateModeStatus(mode: 'LSP' | 'Fallback'): void {
    if (!modeStatusBarItem) {
        modeStatusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
    }
    modeStatusBarItem.text = `Lency: ${mode}`;
    modeStatusBarItem.tooltip = mode === 'LSP'
        ? 'Lency 语言服务运行中（Language Server）'
        : 'Lency 运行在本地 fallback 模式（仅基础单文件能力）';
    modeStatusBarItem.show();
}

export function activate(context: vscode.ExtensionContext): void {
    const selector: vscode.DocumentSelector = [{ language: 'lency', scheme: 'file' }];

    registerProviders(context, selector);

    const lspResult = startLanguageClient(context);
    client = lspResult.client;

    if (lspResult.started) {
        updateModeStatus('LSP');
    } else {
        updateModeStatus('Fallback');
        // TODO: 接入独立设置项，允许用户明确指定 lency_ls 路径。
        void vscode.window.showWarningMessage('未找到 lency_ls，可用本地降级能力已启用（单文件语义）。');
        registerFallbackDiagnostics(context);
    }

    if (modeStatusBarItem) {
        context.subscriptions.push(modeStatusBarItem);
    }
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
