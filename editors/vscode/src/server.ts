/* eslint-disable @typescript-eslint/naming-convention */

import * as vscode from "vscode";
import * as os from "os";

import {
	Executable,
	ExecutableOptions,
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
} from "vscode-languageclient/node";

import { getExtensionContext } from "./extension";

let client: LanguageClient | undefined;
let outputChannel: vscode.OutputChannel;

/**
	Starts the language server.

	Will throw an error if the language server has already been started.
*/
export const startServer = async () => {
	if (client !== undefined) {
		throw new Error("Language server has already been started");
	}

	const context = getExtensionContext();

	// Create persistent output channel if one does not exist

	if (outputChannel === undefined) {
		outputChannel = vscode.window.createOutputChannel(
			"Zap Language Server",
		);
	}

	// Create args for language server

	const server: Executable = {
		command: "zap-language-server",
		options: { env: { PATH: process.env.PATH } },
		args: ["serve"],
	};

	// Create language server & client config

	const serverOptions: ServerOptions = {
		run: server,
		debug: server,
	};

	const clientOptions: LanguageClientOptions = {
		stdioEncoding: "utf8",
		documentSelector: [{ scheme: "file", language: "zap" }],
		outputChannel,
	};

	// Start language server & client

	outputChannel.appendLine("Starting language server");

	client = new LanguageClient(
		"zap-language-server",
		"Zap Language Server",
		serverOptions,
		clientOptions,
	);

	client.start();
};

/**
	Stops the language server.

	Returns `true` if stopped, `false` if the language server was not running.
*/
export const stopServer = async (): Promise<boolean> => {
	const c = client;
	if (c !== undefined) {
		client = undefined;
		await c.stop();
		return true;
	} else {
		return false;
	}
};

/**
	Stops and then starts the language server.

	Should be used only when a language server configuration that requires a full
	restart is needed, other methods such as notifications should be preferred.
*/
export const restartServer = async () => {
	await stopServer();
	await startServer();
};
