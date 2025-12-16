"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ChatViewProvider = void 0;
const vscode = require("vscode");
class ChatViewProvider {
    constructor(_extensionUri, _context // Main extension context
    ) {
        this._extensionUri = _extensionUri;
        this._context = _context;
    }
    resolveWebviewView(webviewView, context, _token) {
        this._view = webviewView;
        webviewView.webview.options = {
            enableScripts: true,
            localResourceRoots: [this._extensionUri]
        };
        // UI ဒီဇိုင်း (HTML)
        webviewView.webview.html = this._getHtmlForWebview(webviewView.webview);
        // User ဆီက စာ (Message) လက်ခံခြင်း
        webviewView.webview.onDidReceiveMessage(data => {
            switch (data.type) {
                case 'sendMessage':
                    {
                        const prompt = data.value;
                        // User ရိုက်တာကို ပြမယ်
                        vscode.window.showInformationMessage(`Agent received: ${prompt}`);
                        // ပြင်ဆင်ချက်: prompt (project name) ကို ဒုတိယ parameter အနေနဲ့ ထည့်ပေးလိုက်ပါ
                        vscode.commands.executeCommand('neurust.create', prompt);
                        break;
                    }
            }
        });
    }
    _getHtmlForWebview(webview) {
        return `<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <style>
                body { font-family: var(--vscode-font-family); padding: 10px; color: var(--vscode-editor-foreground); }
                .chat-container { display: flex; flex-direction: column; height: 100vh; }
                .messages { flex: 1; overflow-y: auto; margin-bottom: 10px; }
                .input-area { display: flex; gap: 5px; }
                textarea { 
                    width: 100%; 
                    background: var(--vscode-input-background); 
                    color: var(--vscode-input-foreground); 
                    border: 1px solid var(--vscode-input-border);
                    padding: 5px;
                    resize: vertical;
                }
                button { 
                    background: var(--vscode-button-background); 
                    color: var(--vscode-button-foreground); 
                    border: none; padding: 8px; cursor: pointer; 
                }
                button:hover { background: var(--vscode-button-hoverBackground); }
            </style>
        </head>
        <body>
            <div class="chat-container">
                <div class="messages" id="messages">
                    <p>🤖 <b>Neurust:</b> Hello! Describe your project, and I will build it.</p>
                </div>
                <div class="input-area">
                    <textarea id="promptInput" rows="3" placeholder="e.g., Create a Solana Todo App..."></textarea>
                </div>
                <div style="margin-top:5px;">
                     <button id="sendBtn" style="width:100%">Generate Project 🚀</button>
                </div>
            </div>

            <script>
                const vscode = acquireVsCodeApi();
                const sendBtn = document.getElementById('sendBtn');
                const promptInput = document.getElementById('promptInput');

                sendBtn.addEventListener('click', () => {
                    const text = promptInput.value;
                    if (text) {
                        // Extension (Backend) ဆီ စာပို့မယ်
                        vscode.postMessage({ type: 'sendMessage', value: text });
                        promptInput.value = ''; // Clear input
                    }
                });
            </script>
        </body>
        </html>`;
    }
}
exports.ChatViewProvider = ChatViewProvider;
ChatViewProvider.viewType = 'neurust.chatView';
//# sourceMappingURL=ChatViewProvider.js.map