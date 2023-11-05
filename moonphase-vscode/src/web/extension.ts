// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from 'vscode';
// import * as moonPhase from '../../../out';
import { Moon } from "lunarphase-js";

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {

	// Use the console to output diagnostic information (console.log) and errors (console.error)
	// This line of code will only be executed once when your extension is activated
	console.log('Congratulations, your extension "moonphase" is now active in the web extension host!');

	// The command has been defined in the package.json file
	// Now provide the implementation of the command with registerCommand
	// The commandId parameter must match the command field in package.json
	let disposable = vscode.commands.registerCommand('moonphase.helloWorld', () => {
		// The code you place here will be executed every time your command is executed

		// Display a message box to the user
		vscode.window.showInformationMessage('Hello World from moonphase in a web extension host!');
	});

	context.subscriptions.push(disposable);

	disposable = vscode.commands.registerCommand('moonphase.insertPhase', () => {
		if (vscode.window.activeTextEditor !== undefined) {
			let pos = vscode.window.activeTextEditor.selection.anchor;
			vscode.window.activeTextEditor?.edit((eb) => {
				// let mp = moonPhase.get_current_moon_phase();
				eb.insert(pos, Moon.lunarPhaseEmoji() + " " + Moon.lunarPhase());
			});
		}
	});

	context.subscriptions.push(disposable);
}

// This method is called when your extension is deactivated
export function deactivate() { }
