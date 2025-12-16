"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const ChatViewProvider_1 = require("./ChatViewProvider");
const vscode = require("vscode");
const cp = require("child_process");
const path = require("path");
const fs = require("fs");
function activate(context) {
    console.log('Neurust Extension is active!');
    let auditDisposable = vscode.commands.registerCommand('neurust.audit', () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No active file found!');
            return;
        }
        const filePath = editor.document.fileName;
        vscode.window.showInformationMessage(`Neurust: Auditing...`);
        // Context ကို ထည့်ပေးလိုက်တယ်
        runCliCommand(['audit', filePath], context);
    });
    let loginDisposable = vscode.commands.registerCommand('neurust.login', () => {
        vscode.window.showInformationMessage('Neurust: Authenticating with Solana...');
        // Context ကို ထည့်ပေးလိုက်တယ်
        runCliCommand(['login'], context);
    });
    // Command 3: Create Project
    let createDisposable = vscode.commands.registerCommand('neurust.create', (prompt) => __awaiter(this, void 0, void 0, function* () {
        // Chat Box ကနေ နာမည်မပါလာမှသာ Input Box နဲ့ မေးမယ်
        if (!prompt) {
            prompt = yield vscode.window.showInputBox({
                placeHolder: 'Ask Neurust anything (e.g., "Airdrop 2 SOL", "Create dapp")',
                prompt: 'What do you want to do?'
            });
        }
        if (!prompt) {
            return;
        }
        vscode.window.showInformationMessage(`Neurust Agent: Processing '${prompt}'...`);
        // CLI ကို လှမ်းခေါ်မယ်
        runCliCommand(['ask', prompt], context);
    }));
    // Chat View Provider ကို Register လုပ်မယ်
    const provider = new ChatViewProvider_1.ChatViewProvider(context.extensionUri, context);
    context.subscriptions.push(vscode.window.registerWebviewViewProvider(ChatViewProvider_1.ChatViewProvider.viewType, provider));
    context.subscriptions.push(auditDisposable);
    context.subscriptions.push(loginDisposable);
    context.subscriptions.push(createDisposable);
}
// context parameter အသစ်ပါလာပါတယ်
function runCliCommand(args, context) {
    // နည်းလမ်းသစ်: Extension ရှိတဲ့နေရာကနေ နောက်ကို တဆုတ်ပြီး Binary ရှာမယ်
    // User က Folder ဖွင့်ထားထား၊ မဖွင့်ထားထား ကိစ္စမရှိတော့ပါဘူး
    const extensionPath = context.extensionUri.fsPath;
    const workspaceRoot = path.join(extensionPath, '..'); // neurust-workspace folder
    // Binary Path (Linux/Mac)
    const command = path.join(workspaceRoot, 'target', 'debug', 'neurust-cli');
    // Output Channel
    const outputChannel = vscode.window.createOutputChannel("Neurust AI");
    outputChannel.show();
    outputChannel.appendLine(`🚀 Binary Path: ${command}`);
    if (!fs.existsSync(command)) {
        outputChannel.appendLine("❌ Binary not found! Try running 'cargo build -p neurust-cli' in terminal.");
        vscode.window.showErrorMessage("Neurust Binary not found. Check Output.");
        return;
    }
    const options = {
        cwd: workspaceRoot,
        env: process.env // Wallet ရှာဖို့အတွက် မဖြစ်မနေလိုပါတယ်
    };
    cp.execFile(command, args, options, (err, stdout, stderr) => {
        if (err) {
            console.error(stderr);
            outputChannel.appendLine(`❌ Error: ${stderr || err.message}`);
            // Timeout Error ဆိုရင် သီးသန့်ပြမယ်
            if (stderr.includes("timed out") || err.message.includes("timed out")) {
                vscode.window.showErrorMessage("Server Connection Timeout. Is 'neurust-server' running?");
            }
            else {
                vscode.window.showErrorMessage("Neurust Error. Check Output.");
            }
            return;
        }
        outputChannel.appendLine(`Output:\n${stdout}`);
        if (stdout.includes("Login Successful") || stdout.includes("✅")) {
            vscode.window.showInformationMessage("Login Successful");
        }
    });
}
function deactivate() { }
//# sourceMappingURL=extension.js.map