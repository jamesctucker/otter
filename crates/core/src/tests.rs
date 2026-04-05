//! Tests for core types.

use crate::{
    DaemonEvent, InferenceEvent, InferenceRequest, InferenceUsage, Message, MessageId, MessageRole,
    ModelId, ResponseMode, SessionId, SessionRecord,
};

#[test]
fn daemon_event_serde_round_trip() {
    let events = vec![
        DaemonEvent::SessionStarted {
            session_id: SessionId("s-1".into()),
        },
        DaemonEvent::TokenReceived {
            message_id: MessageId("m-1".into()),
            token: "hello".into(),
        },
        DaemonEvent::MessageCompleted {
            message_id: MessageId("m-1".into()),
        },
        DaemonEvent::ToolCallRequested {
            tool_name: "read_file".into(),
            params: serde_json::json!({"path": "src/main.rs"}),
        },
        DaemonEvent::ToolCallCompleted {
            tool_name: "read_file".into(),
            result: serde_json::json!({"content": "fn main() {}"}),
        },
        DaemonEvent::ModelSwitched {
            model_id: ModelId("llama-7b".into()),
        },
        DaemonEvent::ResponseModeChanged {
            mode: ResponseMode::Plan,
        },
        DaemonEvent::Error {
            message: "something broke".into(),
        },
        DaemonEvent::SessionEnded {
            session_id: SessionId("s-1".into()),
        },
    ];

    for event in &events {
        let json = serde_json::to_string(event).expect("serialize");
        let back: DaemonEvent = serde_json::from_str(&json).expect("deserialize");
        // Re-serialize to confirm structural equality.
        let json2 = serde_json::to_string(&back).expect("re-serialize");
        assert_eq!(json, json2, "round-trip mismatch for {json}");
    }
}

#[test]
fn inference_event_serde_round_trip() {
    let events = vec![
        InferenceEvent::Token {
            delta: "fn ".into(),
        },
        InferenceEvent::Done {
            usage: InferenceUsage {
                prompt_tokens: 10,
                completion_tokens: 5,
            },
        },
        InferenceEvent::Error {
            message: "timeout".into(),
        },
    ];

    for event in &events {
        let json = serde_json::to_string(event).expect("serialize");
        let back: InferenceEvent = serde_json::from_str(&json).expect("deserialize");
        let json2 = serde_json::to_string(&back).expect("re-serialize");
        assert_eq!(json, json2, "round-trip mismatch for {json}");
    }
}

#[test]
fn inference_request_serde_round_trip() {
    let req = InferenceRequest {
        model_id: ModelId("llama-7b".into()),
        messages: vec![
            Message {
                id: MessageId("m-1".into()),
                role: MessageRole::System,
                content: "You are terse.".into(),
            },
            Message {
                id: MessageId("m-2".into()),
                role: MessageRole::User,
                content: "Explain closures.".into(),
            },
        ],
        max_tokens: Some(256),
        temperature: Some(0.7),
        response_mode: Some(ResponseMode::Act),
        stream: true,
    };

    let json = serde_json::to_string(&req).expect("serialize");
    let back: InferenceRequest = serde_json::from_str(&json).expect("deserialize");
    let json2 = serde_json::to_string(&back).expect("re-serialize");
    assert_eq!(json, json2);
}

#[test]
fn session_record_serde_round_trip() {
    let record = SessionRecord {
        id: SessionId("s-1".into()),
        messages: vec![Message {
            id: MessageId("m-1".into()),
            role: MessageRole::User,
            content: "hi".into(),
        }],
        response_mode: ResponseMode::Ack,
    };

    let json = serde_json::to_string(&record).expect("serialize");
    let back: SessionRecord = serde_json::from_str(&json).expect("deserialize");
    let json2 = serde_json::to_string(&back).expect("re-serialize");
    assert_eq!(json, json2);
}
