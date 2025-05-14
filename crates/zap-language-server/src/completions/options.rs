use std::path::{Path, PathBuf};

use async_language_server::{
    lsp_types::{CompletionItemKind, Position, Url},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::{find_child, find_nearest, ts_range_contains_lsp_position},
};

use zap_language::docs::{find_option, find_variants, get_option_names};

pub async fn completion(
    doc: &Document,
    pos: Position,
    node: Node<'_>,
) -> Vec<(CompletionItemKind, String)> {
    let Some(node) = find_nearest(node, pos, |n| n.kind() == "option_declaration") else {
        return Vec::new();
    };

    // The option declaration should have the option identifier,
    // and the value, which can not be the same node as identifier
    let opt_ident = find_child(node, is_opt_ident);
    let Some(opt_ident) = opt_ident else {
        return Vec::new();
    };

    let opt_name = doc.text().byte_slice(opt_ident.byte_range()).to_string();
    let opt_value = find_child(node, |c| c != opt_ident && is_opt_value(c));

    let mut items = Vec::new();

    if ts_range_contains_lsp_position(opt_ident.range(), pos) {
        // We are currently inside the identifier, complete option names
        items.extend(get_option_names().map(|opt| (CompletionItemKind::PROPERTY, opt.to_string())));
    } else if opt_value.is_some_and(|v| ts_range_contains_lsp_position(v.range(), pos)) {
        // We are currently inside the value, try to complete variants
        let Some(opt_value) = opt_value else {
            return Vec::new();
        };
        if let Some((_, typ, _)) = find_option([opt_name.trim()]) {
            if typ == "boolean" {
                // Plain booleans - will be categorized as "identifier" when incomplete
                if matches!(opt_value.kind(), "boolean" | "identifier") {
                    items.push((CompletionItemKind::CONSTANT, String::from("true")));
                    items.push((CompletionItemKind::CONSTANT, String::from("false")));
                }
            } else if typ == "variant" {
                // Option variants - must also be enclosed in strings
                if opt_value.kind() == "string" {
                    if let Some((true, variants)) = find_variants([&opt_name]) {
                        items.extend(variants.iter().map(|variant| {
                            (CompletionItemKind::ENUM_MEMBER, (*variant).to_string())
                        }));
                    }
                }
            } else if typ == "path" {
                // File paths - don't have to exist, but completions
                // for existing directories is probably nice to have
                if opt_value.kind() == "string" {
                    let path = doc.text().byte_slice(opt_value.byte_range());
                    let path = PathBuf::from(
                        path.to_string()
                            .trim_start_matches('"')
                            .trim_end_matches('"'),
                    );
                    items.extend(
                        gather_cwd_completion_directories(doc.url(), &path)
                            .await
                            .into_iter()
                            .flatten()
                            .map(|variant| (CompletionItemKind::FOLDER, variant.to_string())),
                    );
                }
            } else if typ == "number" {
                // Numbers - completions are not relevant here
            }
        }
    }

    items
}

fn is_opt_ident(node: Node) -> bool {
    matches!(node.kind(), "identifier")
}
fn is_opt_value(node: Node) -> bool {
    matches!(node.kind(), "string" | "number" | "boolean" | "identifier")
}

async fn gather_cwd_completion_directories(uri: &Url, path: &Path) -> Option<Vec<String>> {
    let file = uri.to_file_path().ok()?;
    let dir = file.parent()?;
    let path = dir.join(path);

    let mut items = Vec::new();
    let mut reader = tokio::fs::read_dir(path).await.ok()?;

    while let Ok(Some(item)) = reader.next_entry().await {
        if let Ok(meta) = item.metadata().await {
            if meta.is_dir() {
                if let Some(name) = item.file_name().to_str() {
                    items.push(name.to_string());
                }
            }
        }
    }

    Some(items)
}
