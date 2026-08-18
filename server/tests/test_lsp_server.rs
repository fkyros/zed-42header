use header42_lsp::backend::Backend;
use tower_lsp::lsp_types::*;
use tower_lsp::LanguageServer;

#[tokio::test]
async fn test_lsp_initialize_and_capabilities() {
    let (service, _socket) = tower_lsp::LspService::new(|client| Backend::new(client));
    let backend = service.inner();

    let init_params = InitializeParams {
        initialization_options: Some(serde_json::json!({
            "user": "testuser",
            "mail": "testuser@student.42.fr"
        })),
        ..Default::default()
    };

    let result = backend.initialize(init_params).await.unwrap();
    assert!(result.capabilities.code_action_provider.is_some());
    assert!(result.capabilities.document_formatting_provider.is_some());
}

#[tokio::test]
async fn test_lsp_code_action_insert_header() {
    let (service, _socket) = tower_lsp::LspService::new(|client| Backend::new(client));
    let backend = service.inner();

    let uri = Url::parse("file:///workspace/test.c").unwrap();
    backend
        .did_open(DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: uri.clone(),
                language_id: "c".to_string(),
                version: 1,
                text: "int main() { return 0; }\n".to_string(),
            },
        })
        .await;

    let actions = backend
        .code_action(CodeActionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            range: Range::default(),
            context: CodeActionContext::default(),
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        })
        .await
        .unwrap();

    assert!(actions.is_some());
    let action_list = actions.unwrap();
    assert_eq!(action_list.len(), 1);
    match &action_list[0] {
        CodeActionOrCommand::CodeAction(action) => {
            assert_eq!(action.title, "Insert 42 Header");
            let changes = action.edit.as_ref().unwrap().changes.as_ref().unwrap();
            let edits = changes.get(&uri).unwrap();
            assert_eq!(edits.len(), 1);
            assert!(edits[0].new_text.contains("/* ************************************************************************** */"));
            assert!(edits[0].new_text.contains("test.c"));
        }
        _ => panic!("Expected CodeAction"),
    }
}

#[tokio::test]
async fn test_lsp_formatting_updates_existing_header() {
    let (service, _socket) = tower_lsp::LspService::new(|client| Backend::new(client));
    let backend = service.inner();

    let uri = Url::parse("file:///workspace/test.c").unwrap();
    let delim = header42_lsp::comments::get_delimiters("test.c");
    let old_header = header42_lsp::header::generate_header(
        "test.c",
        "login",
        "login@student.42.fr",
        "2020/01/01 00:00:00",
        "2020/01/01 00:00:00",
        &delim,
    );
    let doc_content = format!("{}\n\nint main() {{ return 0; }}\n", old_header);

    backend
        .did_open(DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: uri.clone(),
                language_id: "c".to_string(),
                version: 1,
                text: doc_content,
            },
        })
        .await;

    let formatting_edits = backend
        .formatting(DocumentFormattingParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            options: FormattingOptions {
                tab_size: 4,
                insert_spaces: false,
                ..Default::default()
            },
            work_done_progress_params: Default::default(),
        })
        .await
        .unwrap();

    assert!(formatting_edits.is_some());
    let edits = formatting_edits.unwrap();
    assert_eq!(edits.len(), 1, "Formatting should return 1 edit targeting the Updated line");
    assert_eq!(edits[0].range.start.line, 8);
    assert_eq!(edits[0].range.end.line, 8);
    assert!(edits[0].new_text.contains("Updated: "));
    assert!(!edits[0].new_text.contains("2020/01/01"));
}
