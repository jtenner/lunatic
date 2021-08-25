;; This file is used for testing import signatures.

(module
    (import "lunatic::error" "string_size" (func (param i64) (result i32)))
    (import "lunatic::error" "to_string" (func (param i64 i32)))
    (import "lunatic::error" "drop" (func (param i64)))

    (import "lunatic::message" "create_data" (func (param i64 i64)))
    (import "lunatic::message" "write_data" (func (param i32 i32) (result i32)))
    (import "lunatic::message" "read_data" (func (param i32 i32) (result i32)))
    (import "lunatic::message" "seek_data" (func (param i64)))
    (import "lunatic::message" "get_tag" (func (result i64)))
    (import "lunatic::message" "data_size" (func (result i64)))
    (import "lunatic::message" "push_process" (func (param i64) (result i64)))
    (import "lunatic::message" "take_process" (func (param i64) (result i64)))
    (import "lunatic::message" "push_tcp_stream" (func (param i64) (result i64)))
    (import "lunatic::message" "take_tcp_stream" (func (param i64) (result i64)))
    (import "lunatic::message" "send" (func (param i64)))
    (import "lunatic::message" "send_receive_skip_search" (func (param i64 i32) (result i32)))
    (import "lunatic::message" "receive" (func (param i64 i32) (result i32)))

    (import "lunatic::networking" "resolve" (func (param i32 i32 i32 i32) (result i32)))
    (import "lunatic::networking" "drop_dns_iterator" (func (param i64)))
    (import "lunatic::networking" "resolve_next" (func (param i64 i32 i32 i32 i32 i32) (result i32)))
    (import "lunatic::networking" "tcp_bind" (func (param i32 i32 i32 i32 i32 i32) (result i32)))
    (import "lunatic::networking" "drop_tcp_listener" (func (param i64)))
    (import "lunatic::networking" "tcp_accept" (func (param i64 i32 i32) (result i32)))
    (import "lunatic::networking" "tcp_connect" (func (param i32 i32 i32 i32 i32 i32 i32) (result i32)))
    (import "lunatic::networking" "drop_tcp_stream" (func (param i64)))
    (import "lunatic::networking" "clone_tcp_stream" (func (param i64) (result i64)))
    (import "lunatic::networking" "tcp_write_vectored" (func (param i64 i32 i32 i32 i32) (result i32)))
    (import "lunatic::networking" "tcp_read" (func (param i64 i32 i32 i32 i32) (result i32)))
    (import "lunatic::networking" "tcp_flush" (func (param i64 i32) (result i32)))
    
    (import "lunatic::process" "create_config" (func (param i64 i64) (result i64)))
    (import "lunatic::process" "drop_config" (func (param i64)))
    (import "lunatic::process" "allow_namespace" (func (param i64 i32 i32)))
    (import "lunatic::process" "add_plugin" (func (param i64 i32 i32 i32) (result i32)))
    (import "lunatic::process" "create_environment" (func (param i64 i32) (result i32)))
    (import "lunatic::process" "drop_environment" (func (param i64)))
    (import "lunatic::process" "add_module" (func (param i64 i32 i32 i32) (result i32)))
    (import "lunatic::process" "add_this_module" (func (param i64 i32) (result i32)))
    (import "lunatic::process" "drop_module" (func (param i64)))
    (import "lunatic::process" "spawn" (func (param i64 i64 i32 i32 i32 i32 i32) (result i32)))
    (import "lunatic::process" "inherit_spawn" (func (param i64 i32 i32 i32 i32  i32) (result i32)))
    (import "lunatic::process" "drop_process" (func (param i64)))
    (import "lunatic::process" "clone_process" (func (param i64) (result i64)))
    (import "lunatic::process" "sleep_ms" (func (param i64)))
    (import "lunatic::process" "die_when_link_dies" (func (param i32)))
    (import "lunatic::process" "this" (func (result i64)))
    (import "lunatic::process" "id" (func (param i64 i32)))
    (import "lunatic::process" "this_env" (func (result i64)))
    (import "lunatic::process" "link" (func (param i64 i64)))
    (import "lunatic::process" "unlink" (func (param i64)))
    (import "lunatic::process" "register" (func (param i32 i32 i32 i32 i64 i64)))
    (import "lunatic::process" "unregister" (func (param i32 i32 i32 i32 i64) (result i32)))
    (import "lunatic::process" "lookup" (func (param i32 i32 i32 i32 i32) (result i32)))

    ;; TODO: Add all WASI imports

    (func (export "hello") nop)
)