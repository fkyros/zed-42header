use crate::comments::get_delimiters;
use crate::config::resolve_identity;
use crate::header::{current_formatted_date, detect_header, generate_header, generate_line};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

pub struct Backend {
    pub client: Client,
    pub documents: Arc<RwLock<HashMap<Url, String>>>,
    pub user: Arc<RwLock<String>>,
    pub mail: Arc<RwLock<String>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        let (default_user, default_mail) = resolve_identity(None, None);
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
            user: Arc::new(RwLock::new(default_user)),
            mail: Arc::new(RwLock::new(default_mail)),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let mut settings_user: Option<String> = None;
        let mut settings_mail: Option<String> = None;

        if let Some(options) = params.initialization_options {
            if let Some(user_val) = options.get("user").and_then(|v| v.as_str()) {
                settings_user = Some(user_val.to_string());
            }
            if let Some(mail_val) = options.get("mail").and_then(|v| v.as_str()) {
                settings_mail = Some(mail_val.to_string());
            }
        }

        let (resolved_user, resolved_mail) =
            resolve_identity(settings_user.as_deref(), settings_mail.as_deref());

        *self.user.write().await = resolved_user;
        *self.mail.write().await = resolved_mail;

        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "header42-lsp".to_string(),
                version: Some("0.1.0".to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "header42-lsp initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.documents
            .write()
            .await
            .insert(params.text_document.uri, params.text_document.text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().last() {
            self.documents
                .write()
                .await
                .insert(params.text_document.uri, change.text);
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents
            .write()
            .await
            .remove(&params.text_document.uri);
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = &params.text_document.uri;
        let content = match self.documents.read().await.get(uri) {
            Some(doc) => doc.clone(),
            None => String::new(),
        };

        let file_path = uri.path();
        let filename = Path::new(file_path)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("< new >");

        let delimiters = get_delimiters(file_path);
        let user = self.user.read().await.clone();
        let mail = self.mail.read().await.clone();
        let now = current_formatted_date();

        let mut actions = Vec::new();

        if let Some(header_info) = detect_header(&content, &delimiters) {
            // Update Code Action
            let new_line_9 = generate_line(
                9,
                &header_info.filename,
                &user,
                &header_info.author_mail,
                &now,
                &delimiters,
            );

            let lines: Vec<&str> = content.lines().collect();
            let line_8_len = lines.get(8).map(|l| l.len() as u32).unwrap_or(80);

            let mut changes = HashMap::new();
            changes.insert(
                uri.clone(),
                vec![TextEdit {
                    range: Range::new(Position::new(8, 0), Position::new(8, line_8_len)),
                    new_text: new_line_9,
                }],
            );

            actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                title: "Update 42 Header".to_string(),
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: None,
                edit: Some(WorkspaceEdit {
                    changes: Some(changes),
                    document_changes: None,
                    change_annotations: None,
                }),
                command: None,
                is_preferred: Some(true),
                disabled: None,
                data: None,
            }));
        } else {
            // Insert Code Action
            let header = generate_header(filename, &user, &mail, &now, &now, &delimiters);
            let new_text = if content.is_empty() {
                format!("{}\n", header)
            } else {
                format!("{}\n\n", header)
            };

            let mut changes = HashMap::new();
            changes.insert(
                uri.clone(),
                vec![TextEdit {
                    range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                    new_text,
                }],
            );

            actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                title: "Insert 42 Header".to_string(),
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: None,
                edit: Some(WorkspaceEdit {
                    changes: Some(changes),
                    document_changes: None,
                    change_annotations: None,
                }),
                command: None,
                is_preferred: Some(true),
                disabled: None,
                data: None,
            }));
        }

        Ok(Some(actions))
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = &params.text_document.uri;
        let content = match self.documents.read().await.get(uri) {
            Some(doc) => doc.clone(),
            None => return Ok(Some(vec![])),
        };

        let file_path = uri.path();
        let delimiters = get_delimiters(file_path);

        if let Some(header_info) = detect_header(&content, &delimiters) {
            let user = self.user.read().await.clone();
            let now = current_formatted_date();
            let new_line_9 = generate_line(
                9,
                &header_info.filename,
                &user,
                &header_info.author_mail,
                &now,
                &delimiters,
            );

            let lines: Vec<&str> = content.lines().collect();
            if lines.len() > 8 && lines[8] != new_line_9 {
                let line_8_len = lines[8].len() as u32;
                return Ok(Some(vec![TextEdit {
                    range: Range::new(Position::new(8, 0), Position::new(8, line_8_len)),
                    new_text: new_line_9,
                }]));
            }
        }

        Ok(Some(vec![]))
    }
}
