use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use async_language_server::{
    lsp_types::{CompletionItemKind, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_contains_lsp_position,
};

use crate::docs::{find_option, find_variants, get_option_names};

pub async fn completion(
    doc: &Document,
    pos: &Position,
    node: &Node<'_>,
    parent: Option<&Node<'_>>,
) -> Vec<(CompletionItemKind, String)> {
    let pos = pos.clone();

    let mut node = node.clone();
    let mut parent = parent.cloned();

    // If our current node is the top-level "source file" we can
    // probably drill down to something a bit more specific & useful
    if node.kind() == "source_file" {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "option_declaration" {
                if ts_range_contains_lsp_position(child.range(), pos) {
                    parent = Some(node);
                    node = child;
                    break;
                }
            }
        }
    }

    // If we are inside *a child* of an option declaration, traverse one up
    if parent.is_some_and(|p| p.kind() == "option_declaration") {
        node = parent.unwrap();
        parent = doc.node_at_root();
    }

    // If we are inside an option declaration, find the identifier node,
    // as well as the node we are currently inside (ident or value child)
    let mut ident = None;
    if node.kind() == "option_declaration" {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "identifier" {
                // First identifier is option name, second
                // might be an incomplete option value
                if ident.is_none() {
                    ident = Some(child);
                }
            } else if !matches!(child.kind(), "string" | "boolean") {
                continue; // Ignore spacing, equals, etc
            }
            if ts_range_contains_lsp_position(child.range(), pos) {
                parent = Some(node);
                node = child;
            }
        }
    }

    // We might have a parent by now, and if we do, it must
    // also be an option declaration to provide completions
    if parent.is_none_or(|p| p.kind() != "option_declaration") {
        return Vec::new();
    }

    let mut items = Vec::new();

    let ident = ident.expect("valid options have idents");
    if node == ident {
        // We are currently inside the identifier, complete option names
        let ident = doc.text().byte_slice(node.byte_range());
        if let Some(ident) = ident.as_str() {
            items.extend(
                get_option_names()
                    .filter(|opt| opt.contains(ident))
                    .map(|opt| (CompletionItemKind::PROPERTY, opt.to_string())),
            );
        }
    } else {
        // We are currently inside the value, try to complete variants
        let ident = doc.text().byte_slice(ident.byte_range());
        if let Some(ident) = ident.as_str() {
            if let Some((_, typ, _)) = find_option([ident]) {
                if typ == "boolean" {
                    // Plain booleans
                    items.push((CompletionItemKind::CONSTANT, String::from("true")));
                    items.push((CompletionItemKind::CONSTANT, String::from("false")));
                } else if node.kind() == "string" {
                    if typ == "variant" {
                        // Option variants - must also be enclosed in strings
                        if let Some((true, variants)) = find_variants([ident]) {
                            if node.kind() == "string" {
                                items.extend(variants.into_iter().map(|variant| {
                                    (CompletionItemKind::ENUM_MEMBER, variant.to_string())
                                }));
                            }
                        }
                    } else if typ == "path" {
                        // File paths - don't have to exist, but completions
                        // for existing directories is probably nice to have
                        let path = doc.text().byte_slice(node.byte_range());
                        let path = PathBuf::from(
                            path.to_string()
                                .trim_start_matches('"')
                                .trim_end_matches('"'),
                        );
                        items.extend(
                            gather_cwd_completion_directories(&path)
                                .await
                                .into_iter()
                                .flatten()
                                .map(|variant| (CompletionItemKind::FOLDER, variant.to_string())),
                        );
                    }
                }
            };
        }
    }

    items
}

async fn gather_cwd_completion_directories(path: &Path) -> Option<Vec<String>> {
    let cwd = current_dir().ok()?;
    let path = cwd.join(path);

    let mut items = Vec::new();
    let mut reader = tokio::fs::read_dir(path).await.ok()?;

    while let Ok(Some(item)) = reader.next_entry().await {
        if let Some(meta) = item.metadata().await.ok() {
            if meta.is_dir() {
                if let Some(name) = item.file_name().to_str() {
                    items.push(name.to_string());
                }
            }
        }
    }

    Some(items)
}
