import * as vscode from "vscode";

import * as client from "./server";

let context: vscode.ExtensionContext;

export function getExtensionContext(): vscode.ExtensionContext {
	if (context !== undefined) {
		return context;
	} else {
		throw new Error("Missing extension context");
	}
}

export async function activate(ctx: vscode.ExtensionContext) {
	context = ctx;

	await client.startServer();
}

export async function deactivate() {
	await client.stopServer();
}
