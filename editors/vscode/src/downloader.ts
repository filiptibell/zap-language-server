import * as fs from "fs/promises";
import * as vscode from "vscode";
import * as os from "os";
import JSZip from "jszip";

import { BINARY_NAME, BINARY_ROOT_DIR, GITHUB_REPO } from "./constants";

// Types

type GithubReleaseAsset = {
	name: string;
	browser_download_url: string;
};

type GithubRelease = {
	tag_name: string;
	assets: GithubReleaseAsset[];
};

type ParsedRelease = {
	version: string;
	downloadUrl: string;
};

// System helpers

function systemOs(): string {
	switch (os.platform()) {
		case "win32":
			return "windows";
		case "darwin":
			return "macos";
		case "linux":
			return "linux";
		default:
			throw new Error(`Unsupported OS: ${os.platform()}`);
	}
}

function systemArch(): string {
	switch (os.arch()) {
		case "x64":
			return "x64";
		case "arm64":
			return "aarch64";
		default:
			throw new Error(`Unsupported architecture: ${os.arch()}`);
	}
}

// Downloader class

export class Downloader {
	private latestVersion: string | null = null;
	private latestDownloaded: boolean = false;

	private readonly exeName: string;

	constructor(private readonly context: vscode.ExtensionContext) {
		if (os.platform() === "win32") {
			this.exeName = `${BINARY_NAME}.exe`;
		} else {
			this.exeName = BINARY_NAME;
		}
	}

	// Private

	private dirForVersions(): vscode.Uri {
		const base = this.context.extensionUri;
		return vscode.Uri.joinPath(base, BINARY_ROOT_DIR);
	}

	private dirForVersion(version: string): vscode.Uri {
		return vscode.Uri.joinPath(this.dirForVersions(), version);
	}

	private fileForVersion(version: string): vscode.Uri {
		const dir = this.dirForVersion(version);
		return vscode.Uri.joinPath(dir, this.exeName);
	}

	private async cleanupAllVersionsExcept(version: string): Promise<void> {
		const vdir = this.dirForVersions();
		try {
			const entries = await vscode.workspace.fs.readDirectory(vdir);
			for (const [name, type] of entries) {
				if (type === vscode.FileType.Directory && name !== version) {
					await vscode.workspace.fs.delete(
						vscode.Uri.joinPath(vdir, name),
						{ recursive: true, useTrash: false },
					);
				}
			}
		} catch {
			// Ignore if bin directory doesn't exist yet
		}
	}

	private async findLatestDownloadableGithubRelease(): Promise<ParsedRelease> {
		const response = await fetch(
			`https://api.github.com/repos/${GITHUB_REPO}/releases/latest`,
		);

		const data = (await response.json()) as GithubRelease;
		const version = data.tag_name;
		const systemId = `${systemOs()}-${systemArch()}`;

		let downloadUrl: string | null = null;
		for (const asset of data.assets) {
			if (asset.name.includes(systemId)) {
				downloadUrl = asset.browser_download_url;
				break;
			}
		}
		if (!downloadUrl) {
			const assetNames = data.assets.map((a) => a.name).join(", ");
			throw new Error(
				`No release asset was found matching "${systemId}".` +
					`Found release assets: ${assetNames}`,
			);
		}

		return {
			version,
			downloadUrl,
		};
	}

	private async downloadAndExtractGithubReleaseAsset(
		release: ParsedRelease,
	): Promise<ArrayBuffer> {
		const response = await fetch(release.downloadUrl);

		const zipBytes = await response.arrayBuffer();
		const zipFile = await JSZip.loadAsync(zipBytes);

		for (const relativePath in zipFile.files) {
			const entry = zipFile.files[relativePath];
			if (entry.name === this.exeName) {
				const fileContent = await entry.async("nodebuffer");
				return fileContent;
			}
		}

		throw new Error(
			`Failed to find "${this.exeName}" in the latest release asset`,
		);
	}

	// Public

	public async download(): Promise<void> {
		if (this.latestDownloaded) {
			return;
		}

		// 1a. Find the latest downloadable GitHub release
		const latest = await this.findLatestDownloadableGithubRelease();

		// 1b. Skip download if the latest version is already downloaded
		const existing =
			this.context.globalState.get<string>("downloadedVersion");
		if (existing === latest.version) {
			this.latestVersion = latest.version;
			this.latestDownloaded = true;
			return;
		}

		// 2. Download the latest version & extract the raw binary from the zip
		const binary = await this.downloadAndExtractGithubReleaseAsset(latest);

		// 3a. Write the binary to disk at the correct location
		const dir = this.dirForVersion(latest.version);
		const file = vscode.Uri.joinPath(dir, this.exeName);
		await vscode.workspace.fs.createDirectory(dir);
		await vscode.workspace.fs.writeFile(file, new Uint8Array(binary));

		// 3b. Make the binary executable on Unix systems, note
		//     that the VSCODE fs API doesn't support chmod
		if (os.platform() !== "win32") {
			await fs.chmod(file.fsPath, 0o755);
		}

		// 4. Update the cache with the downloaded version
		this.latestVersion = latest.version;
		this.latestDownloaded = true;
		this.context.globalState.update("downloadedVersion", latest.version);

		// 5. Finally, remove old versions that are no longer necessary (if any)
		this.cleanupAllVersionsExcept(latest.version);
	}

	public path(): string {
		if (this.latestVersion && this.latestDownloaded) {
			return this.fileForVersion(this.latestVersion).fsPath;
		} else {
			throw new Error("Binary has not been downloaded");
		}
	}
}
