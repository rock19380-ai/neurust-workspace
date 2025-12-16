import { ChatViewProvider } from './ChatViewProvider';
import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';
import * as fs from 'fs';

export function activate(context: vscode.ExtensionContext) {
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
   let createDisposable = vscode.commands.registerCommand('neurust.create', async (prompt?: string) => {
        // Chat Box ကနေ နာမည်မပါလာမှသာ Input Box နဲ့ မေးမယ်
        if (!prompt) {
            prompt = await vscode.window.showInputBox({
                placeHolder: 'Ask Neurust anything (e.g., "Airdrop 2 SOL", "Create dapp")',
                prompt: 'What do you want to do?'
            });
        }

        if (!prompt) { return; }

        vscode.window.showInformationMessage(`Neurust Agent: Processing '${prompt}'...`);

        // CLI ကို လှမ်းခေါ်မယ်
        runCliCommand(['ask', prompt], context);
    });

    // Chat View Provider ကို Register လုပ်မယ်
    const provider = new ChatViewProvider(context.extensionUri, context);
    context.subscriptions.push(
        vscode.window.registerWebviewViewProvider(ChatViewProvider.viewType, provider)
    );

    context.subscriptions.push(auditDisposable);
    context.subscriptions.push(loginDisposable);
    context.subscriptions.push(createDisposable);

}

// context parameter အသစ်ပါလာပါတယ်
function runCliCommand(args: string[], context: vscode.ExtensionContext) {
    
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
            } else {
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

export function deactivate() {}