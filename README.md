 # http server rust project, old unfinished C http server redo and continuation in rust

```
http-server
├─ Cargo.lock
├─ Cargo.toml
├─ README.md
├─ src
│  ├─ config.rs
│  ├─ errors.rs
│  ├─ handlers
│  │  ├─ health.rs
│  │  └─ mod.rs
│  ├─ main.rs
│  ├─ models
│  │  └─ mod.rs
│  └─ routes
│     ├─ mod.rs
│     └─ v1_routes.rs
└─ target
   ├─ .rustc_info.json
   ├─ CACHEDIR.TAG
   ├─ debug
   │  ├─ .cargo-build-lock
   │  ├─ .cargo-lock
   │  ├─ .fingerprint
   │  │  ├─ async-trait-10e1815961dd2ce6
   │  │  │  ├─ dep-lib-async_trait
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-async_trait
   │  │  │  └─ lib-async_trait.json
   │  │  ├─ atomic-waker-7312adca75df78f3
   │  │  │  ├─ dep-lib-atomic_waker
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-atomic_waker
   │  │  │  └─ lib-atomic_waker.json
   │  │  ├─ axum-4bea695fa3e55cee
   │  │  │  ├─ dep-lib-axum
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-axum
   │  │  │  └─ lib-axum.json
   │  │  ├─ axum-core-408be2366e9d29a9
   │  │  │  ├─ dep-lib-axum_core
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-axum_core
   │  │  │  └─ lib-axum_core.json
   │  │  ├─ bytes-fc4dbe8282c07cd0
   │  │  │  ├─ dep-lib-bytes
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-bytes
   │  │  │  └─ lib-bytes.json
   │  │  ├─ cfg-if-bc6f99dd165977f0
   │  │  │  ├─ dep-lib-cfg_if
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-cfg_if
   │  │  │  └─ lib-cfg_if.json
   │  │  ├─ dotenvy-655c250c222611db
   │  │  │  ├─ dep-lib-dotenvy
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-dotenvy
   │  │  │  └─ lib-dotenvy.json
   │  │  ├─ errno-29020f48b04b5859
   │  │  │  ├─ dep-lib-errno
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-errno
   │  │  │  └─ lib-errno.json
   │  │  ├─ form_urlencoded-dc785d797d27590b
   │  │  │  ├─ dep-lib-form_urlencoded
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-form_urlencoded
   │  │  │  └─ lib-form_urlencoded.json
   │  │  ├─ futures-channel-43b8c47964d3ba0b
   │  │  │  ├─ dep-lib-futures_channel
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-futures_channel
   │  │  │  └─ lib-futures_channel.json
   │  │  ├─ futures-core-d9e332389104b8b0
   │  │  │  ├─ dep-lib-futures_core
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-futures_core
   │  │  │  └─ lib-futures_core.json
   │  │  ├─ futures-task-8789ce6d8b0288ce
   │  │  │  ├─ dep-lib-futures_task
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-futures_task
   │  │  │  └─ lib-futures_task.json
   │  │  ├─ futures-util-e4e001c6d80a258e
   │  │  │  ├─ dep-lib-futures_util
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-futures_util
   │  │  │  └─ lib-futures_util.json
   │  │  ├─ http-4dea63e9bd91bced
   │  │  │  ├─ dep-lib-http
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-http
   │  │  │  └─ lib-http.json
   │  │  ├─ http-body-491c9b277a8225a1
   │  │  │  ├─ dep-lib-http_body
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-http_body
   │  │  │  └─ lib-http_body.json
   │  │  ├─ http-body-util-a102616318b9aa73
   │  │  │  ├─ dep-lib-http_body_util
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-http_body_util
   │  │  │  └─ lib-http_body_util.json
   │  │  ├─ http-server-71ac178811113b1f
   │  │  │  ├─ bin-http-server
   │  │  │  ├─ bin-http-server.json
   │  │  │  ├─ dep-bin-http-server
   │  │  │  └─ invoked.timestamp
   │  │  ├─ http-server-ae6d27d1cdf774ef
   │  │  │  ├─ dep-test-bin-http-server
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ test-bin-http-server
   │  │  │  └─ test-bin-http-server.json
   │  │  ├─ httparse-3a974d078fc2e56f
   │  │  │  ├─ build-script-build-script-build
   │  │  │  ├─ build-script-build-script-build.json
   │  │  │  ├─ dep-build-script-build-script-build
   │  │  │  └─ invoked.timestamp
   │  │  ├─ httparse-7cffa47faac774d4
   │  │  │  ├─ run-build-script-build-script-build
   │  │  │  └─ run-build-script-build-script-build.json
   │  │  ├─ httparse-a2ddefa97b563b92
   │  │  │  ├─ dep-lib-httparse
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-httparse
   │  │  │  └─ lib-httparse.json
   │  │  ├─ httpdate-f562b7c058b005db
   │  │  │  ├─ dep-lib-httpdate
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-httpdate
   │  │  │  └─ lib-httpdate.json
   │  │  ├─ hyper-220125fd54d14d0e
   │  │  │  ├─ dep-lib-hyper
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-hyper
   │  │  │  └─ lib-hyper.json
   │  │  ├─ hyper-util-603ff0029e8cc086
   │  │  │  ├─ dep-lib-hyper_util
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-hyper_util
   │  │  │  └─ lib-hyper_util.json
   │  │  ├─ itoa-18241bc2233dfc62
   │  │  │  ├─ dep-lib-itoa
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-itoa
   │  │  │  └─ lib-itoa.json
   │  │  ├─ lazy_static-ec720e21de3dd27f
   │  │  │  ├─ dep-lib-lazy_static
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-lazy_static
   │  │  │  └─ lib-lazy_static.json
   │  │  ├─ libc-029dbd8d4c76489b
   │  │  │  ├─ dep-lib-libc
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-libc
   │  │  │  └─ lib-libc.json
   │  │  ├─ libc-5fd41a2d9e97be39
   │  │  │  ├─ run-build-script-build-script-build
   │  │  │  └─ run-build-script-build-script-build.json
   │  │  ├─ libc-7064b32ee345a353
   │  │  │  ├─ build-script-build-script-build
   │  │  │  ├─ build-script-build-script-build.json
   │  │  │  ├─ dep-build-script-build-script-build
   │  │  │  └─ invoked.timestamp
   │  │  ├─ lock_api-4337be56952e5e64
   │  │  │  ├─ dep-lib-lock_api
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-lock_api
   │  │  │  └─ lib-lock_api.json
   │  │  ├─ log-c37e0462752f1357
   │  │  │  ├─ dep-lib-log
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-log
   │  │  │  └─ lib-log.json
   │  │  ├─ matchers-9c5a22dab0df1a1c
   │  │  │  ├─ dep-lib-matchers
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-matchers
   │  │  │  └─ lib-matchers.json
   │  │  ├─ matchit-4f4bb755e11b2779
   │  │  │  ├─ dep-lib-matchit
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-matchit
   │  │  │  └─ lib-matchit.json
   │  │  ├─ memchr-c1f717ae461c60e0
   │  │  │  ├─ dep-lib-memchr
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-memchr
   │  │  │  └─ lib-memchr.json
   │  │  ├─ mime-707fe96655585e0b
   │  │  │  ├─ dep-lib-mime
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-mime
   │  │  │  └─ lib-mime.json
   │  │  ├─ mio-d85a0128fd60a1a7
   │  │  │  ├─ dep-lib-mio
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-mio
   │  │  │  └─ lib-mio.json
   │  │  ├─ nu-ansi-term-4194f93a5f3136ca
   │  │  │  ├─ dep-lib-nu_ansi_term
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-nu_ansi_term
   │  │  │  └─ lib-nu_ansi_term.json
   │  │  ├─ once_cell-68ecb02160056e0a
   │  │  │  ├─ dep-lib-once_cell
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-once_cell
   │  │  │  └─ lib-once_cell.json
   │  │  ├─ parking_lot-7a103d302e26f843
   │  │  │  ├─ dep-lib-parking_lot
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-parking_lot
   │  │  │  └─ lib-parking_lot.json
   │  │  ├─ parking_lot_core-25eead29c8491529
   │  │  │  ├─ build-script-build-script-build
   │  │  │  ├─ build-script-build-script-build.json
   │  │  │  ├─ dep-build-script-build-script-build
   │  │  │  └─ invoked.timestamp
   │  │  ├─ parking_lot_core-28a0c970f772509d
   │  │  │  ├─ run-build-script-build-script-build
   │  │  │  └─ run-build-script-build-script-build.json
   │  │  ├─ parking_lot_core-ed0b2cc0b4013cc6
   │  │  │  ├─ dep-lib-parking_lot_core
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-parking_lot_core
   │  │  │  └─ lib-parking_lot_core.json
   │  │  ├─ percent-encoding-51ffba506c82a2c0
   │  │  │  ├─ dep-lib-percent_encoding
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-percent_encoding
   │  │  │  └─ lib-percent_encoding.json
   │  │  ├─ pin-project-lite-d6ba8a6bee90f552
   │  │  │  ├─ dep-lib-pin_project_lite
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-pin_project_lite
   │  │  │  └─ lib-pin_project_lite.json
   │  │  ├─ proc-macro2-19449fcd8ddca04a
   │  │  │  ├─ run-build-script-build-script-build
   │  │  │  └─ run-build-script-build-script-build.json
   │  │  ├─ proc-macro2-6e361d414403e107
   │  │  │  ├─ dep-lib-proc_macro2
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-proc_macro2
   │  │  │  └─ lib-proc_macro2.json
   │  │  ├─ proc-macro2-e8b85e895052fb77
   │  │  │  ├─ build-script-build-script-build
   │  │  │  ├─ build-script-build-script-build.json
   │  │  │  ├─ dep-build-script-build-script-build
   │  │  │  └─ invoked.timestamp
   │  │  ├─ quote-406fd56f69b1ef5b
   │  │  │  ├─ build-script-build-script-build
   │  │  │  ├─ build-script-build-script-build.json
   │  │  │  ├─ dep-build-script-build-script-build
   │  │  │  └─ invoked.timestamp
   │  │  ├─ quote-e3338ff9e189f247
   │  │  │  ├─ dep-lib-quote
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-quote
   │  │  │  └─ lib-quote.json
   │  │  ├─ quote-f691a7d012986fa8
   │  │  │  ├─ run-build-script-build-script-build
   │  │  │  └─ run-build-script-build-script-build.json
   │  │  ├─ regex-automata-1cd8434d10240732
   │  │  │  ├─ dep-lib-regex_automata
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-regex_automata
   │  │  │  └─ lib-regex_automata.json
   │  │  ├─ regex-syntax-311d98b3bd96f231
   │  │  │  ├─ dep-lib-regex_syntax
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-regex_syntax
   │  │  │  └─ lib-regex_syntax.json
   │  │  ├─ rustversion-59c47f2dc99f6db0
   │  │  │  ├─ run-build-script-build-script-build
   │  │  │  └─ run-build-script-build-script-build.json
   │  │  ├─ rustversion-9345eaa844044ad5
   │  │  │  ├─ build-script-build-script-build
   │  │  │  ├─ build-script-build-script-build.json
   │  │  │  ├─ dep-build-script-build-script-build
   │  │  │  └─ invoked.timestamp
   │  │  ├─ rustversion-a5439b6abf9a0b66
   │  │  │  ├─ dep-lib-rustversion
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-rustversion
   │  │  │  └─ lib-rustversion.json
   │  │  ├─ ryu-d361f931d71c243a
   │  │  │  ├─ dep-lib-ryu
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-ryu
   │  │  │  └─ lib-ryu.json
   │  │  ├─ scopeguard-b5b126953d4d1d57
   │  │  │  ├─ dep-lib-scopeguard
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-scopeguard
   │  │  │  └─ lib-scopeguard.json
   │  │  ├─ serde-1f3cb856bb3a5939
   │  │  │  ├─ dep-lib-serde
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-serde
   │  │  │  └─ lib-serde.json
   │  │  ├─ serde-830168119362d6eb
   │  │  │  ├─ run-build-script-build-script-build
   │  │  │  └─ run-build-script-build-script-build.json
   │  │  ├─ serde-c1aeda48f7697429
   │  │  │  ├─ build-script-build-script-build
   │  │  │  ├─ build-script-build-script-build.json
   │  │  │  ├─ dep-build-script-build-script-build
   │  │  │  └─ invoked.timestamp
   │  │  ├─ serde_core-65eed7f7203d3051
   │  │  │  ├─ run-build-script-build-script-build
   │  │  │  └─ run-build-script-build-script-build.json
   │  │  ├─ serde_core-818d123c0c2cd104
   │  │  │  ├─ dep-lib-serde_core
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-serde_core
   │  │  │  └─ lib-serde_core.json
   │  │  ├─ serde_core-f3a33c7aca6d46aa
   │  │  │  ├─ build-script-build-script-build
   │  │  │  ├─ build-script-build-script-build.json
   │  │  │  ├─ dep-build-script-build-script-build
   │  │  │  └─ invoked.timestamp
   │  │  ├─ serde_derive-f29c52cf385f4baa
   │  │  │  ├─ dep-lib-serde_derive
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-serde_derive
   │  │  │  └─ lib-serde_derive.json
   │  │  ├─ serde_json-099d8c05912c85a4
   │  │  │  ├─ run-build-script-build-script-build
   │  │  │  └─ run-build-script-build-script-build.json
   │  │  ├─ serde_json-64620c9eb58fc72c
   │  │  │  ├─ dep-lib-serde_json
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-serde_json
   │  │  │  └─ lib-serde_json.json
   │  │  ├─ serde_json-d166b6b75ba93180
   │  │  │  ├─ build-script-build-script-build
   │  │  │  ├─ build-script-build-script-build.json
   │  │  │  ├─ dep-build-script-build-script-build
   │  │  │  └─ invoked.timestamp
   │  │  ├─ serde_path_to_error-4aa619c860e9fdf7
   │  │  │  ├─ dep-lib-serde_path_to_error
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-serde_path_to_error
   │  │  │  └─ lib-serde_path_to_error.json
   │  │  ├─ serde_urlencoded-18988ce4fe9e0e54
   │  │  │  ├─ dep-lib-serde_urlencoded
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-serde_urlencoded
   │  │  │  └─ lib-serde_urlencoded.json
   │  │  ├─ sharded-slab-54e6c9b696dfd18c
   │  │  │  ├─ dep-lib-sharded_slab
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-sharded_slab
   │  │  │  └─ lib-sharded_slab.json
   │  │  ├─ signal-hook-registry-9f2af5a162b77c82
   │  │  │  ├─ dep-lib-signal_hook_registry
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-signal_hook_registry
   │  │  │  └─ lib-signal_hook_registry.json
   │  │  ├─ slab-484496a5e56d87ed
   │  │  │  ├─ dep-lib-slab
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-slab
   │  │  │  └─ lib-slab.json
   │  │  ├─ smallvec-20fb99f0fbe0246f
   │  │  │  ├─ dep-lib-smallvec
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-smallvec
   │  │  │  └─ lib-smallvec.json
   │  │  ├─ socket2-34eab01829be3f52
   │  │  │  ├─ dep-lib-socket2
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-socket2
   │  │  │  └─ lib-socket2.json
   │  │  ├─ syn-60bdbb13700f6005
   │  │  │  ├─ dep-lib-syn
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-syn
   │  │  │  └─ lib-syn.json
   │  │  ├─ sync_wrapper-65042dc5e52f8f1b
   │  │  │  ├─ dep-lib-sync_wrapper
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-sync_wrapper
   │  │  │  └─ lib-sync_wrapper.json
   │  │  ├─ thread_local-8a78a5abf384d9d9
   │  │  │  ├─ dep-lib-thread_local
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-thread_local
   │  │  │  └─ lib-thread_local.json
   │  │  ├─ tokio-972fb711261e38fd
   │  │  │  ├─ dep-lib-tokio
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-tokio
   │  │  │  └─ lib-tokio.json
   │  │  ├─ tokio-macros-9c469441d2fb900c
   │  │  │  ├─ dep-lib-tokio_macros
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-tokio_macros
   │  │  │  └─ lib-tokio_macros.json
   │  │  ├─ tower-10ba45041ade6f2c
   │  │  │  ├─ dep-lib-tower
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-tower
   │  │  │  └─ lib-tower.json
   │  │  ├─ tower-layer-e19fd2325789fc43
   │  │  │  ├─ dep-lib-tower_layer
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-tower_layer
   │  │  │  └─ lib-tower_layer.json
   │  │  ├─ tower-service-4f4d55ac3672fdbb
   │  │  │  ├─ dep-lib-tower_service
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-tower_service
   │  │  │  └─ lib-tower_service.json
   │  │  ├─ tracing-1ca410b79d5d1f69
   │  │  │  ├─ dep-lib-tracing
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-tracing
   │  │  │  └─ lib-tracing.json
   │  │  ├─ tracing-attributes-2e51a3d06b0e5052
   │  │  │  ├─ dep-lib-tracing_attributes
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-tracing_attributes
   │  │  │  └─ lib-tracing_attributes.json
   │  │  ├─ tracing-core-a90a03f61da34714
   │  │  │  ├─ dep-lib-tracing_core
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-tracing_core
   │  │  │  └─ lib-tracing_core.json
   │  │  ├─ tracing-log-e193144326686db6
   │  │  │  ├─ dep-lib-tracing_log
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-tracing_log
   │  │  │  └─ lib-tracing_log.json
   │  │  ├─ tracing-subscriber-7340ce1a7a7ac51f
   │  │  │  ├─ dep-lib-tracing_subscriber
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-tracing_subscriber
   │  │  │  └─ lib-tracing_subscriber.json
   │  │  ├─ unicode-ident-7b5331368ce720fe
   │  │  │  ├─ dep-lib-unicode_ident
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-unicode_ident
   │  │  │  └─ lib-unicode_ident.json
   │  │  ├─ zmij-414b733252835387
   │  │  │  ├─ build-script-build-script-build
   │  │  │  ├─ build-script-build-script-build.json
   │  │  │  ├─ dep-build-script-build-script-build
   │  │  │  └─ invoked.timestamp
   │  │  ├─ zmij-527c9cba665cd1bb
   │  │  │  ├─ dep-lib-zmij
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ lib-zmij
   │  │  │  └─ lib-zmij.json
   │  │  └─ zmij-f61db2e96111f9b0
   │  │     ├─ run-build-script-build-script-build
   │  │     └─ run-build-script-build-script-build.json
   │  ├─ build
   │  │  ├─ httparse-3a974d078fc2e56f
   │  │  │  ├─ build-script-build
   │  │  │  ├─ build_script_build-3a974d078fc2e56f
   │  │  │  └─ build_script_build-3a974d078fc2e56f.d
   │  │  ├─ httparse-7cffa47faac774d4
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ out
   │  │  │  ├─ output
   │  │  │  ├─ root-output
   │  │  │  └─ stderr
   │  │  ├─ libc-5fd41a2d9e97be39
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ out
   │  │  │  ├─ output
   │  │  │  ├─ root-output
   │  │  │  └─ stderr
   │  │  ├─ libc-7064b32ee345a353
   │  │  │  ├─ build-script-build
   │  │  │  ├─ build_script_build-7064b32ee345a353
   │  │  │  └─ build_script_build-7064b32ee345a353.d
   │  │  ├─ parking_lot_core-25eead29c8491529
   │  │  │  ├─ build-script-build
   │  │  │  ├─ build_script_build-25eead29c8491529
   │  │  │  └─ build_script_build-25eead29c8491529.d
   │  │  ├─ parking_lot_core-28a0c970f772509d
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ out
   │  │  │  ├─ output
   │  │  │  ├─ root-output
   │  │  │  └─ stderr
   │  │  ├─ proc-macro2-19449fcd8ddca04a
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ out
   │  │  │  ├─ output
   │  │  │  ├─ root-output
   │  │  │  └─ stderr
   │  │  ├─ proc-macro2-e8b85e895052fb77
   │  │  │  ├─ build-script-build
   │  │  │  ├─ build_script_build-e8b85e895052fb77
   │  │  │  └─ build_script_build-e8b85e895052fb77.d
   │  │  ├─ quote-406fd56f69b1ef5b
   │  │  │  ├─ build-script-build
   │  │  │  ├─ build_script_build-406fd56f69b1ef5b
   │  │  │  └─ build_script_build-406fd56f69b1ef5b.d
   │  │  ├─ quote-f691a7d012986fa8
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ out
   │  │  │  ├─ output
   │  │  │  ├─ root-output
   │  │  │  └─ stderr
   │  │  ├─ rustversion-59c47f2dc99f6db0
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ out
   │  │  │  │  └─ version.expr
   │  │  │  ├─ output
   │  │  │  ├─ root-output
   │  │  │  └─ stderr
   │  │  ├─ rustversion-9345eaa844044ad5
   │  │  │  ├─ build-script-build
   │  │  │  ├─ build_script_build-9345eaa844044ad5
   │  │  │  └─ build_script_build-9345eaa844044ad5.d
   │  │  ├─ serde-830168119362d6eb
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ out
   │  │  │  │  └─ private.rs
   │  │  │  ├─ output
   │  │  │  ├─ root-output
   │  │  │  └─ stderr
   │  │  ├─ serde-c1aeda48f7697429
   │  │  │  ├─ build-script-build
   │  │  │  ├─ build_script_build-c1aeda48f7697429
   │  │  │  └─ build_script_build-c1aeda48f7697429.d
   │  │  ├─ serde_core-65eed7f7203d3051
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ out
   │  │  │  │  └─ private.rs
   │  │  │  ├─ output
   │  │  │  ├─ root-output
   │  │  │  └─ stderr
   │  │  ├─ serde_core-f3a33c7aca6d46aa
   │  │  │  ├─ build-script-build
   │  │  │  ├─ build_script_build-f3a33c7aca6d46aa
   │  │  │  └─ build_script_build-f3a33c7aca6d46aa.d
   │  │  ├─ serde_json-099d8c05912c85a4
   │  │  │  ├─ invoked.timestamp
   │  │  │  ├─ out
   │  │  │  ├─ output
   │  │  │  ├─ root-output
   │  │  │  └─ stderr
   │  │  ├─ serde_json-d166b6b75ba93180
   │  │  │  ├─ build-script-build
   │  │  │  ├─ build_script_build-d166b6b75ba93180
   │  │  │  └─ build_script_build-d166b6b75ba93180.d
   │  │  ├─ zmij-414b733252835387
   │  │  │  ├─ build-script-build
   │  │  │  ├─ build_script_build-414b733252835387
   │  │  │  └─ build_script_build-414b733252835387.d
   │  │  └─ zmij-f61db2e96111f9b0
   │  │     ├─ invoked.timestamp
   │  │     ├─ out
   │  │     ├─ output
   │  │     ├─ root-output
   │  │     └─ stderr
   │  ├─ deps
   │  │  ├─ async_trait-10e1815961dd2ce6.d
   │  │  ├─ atomic_waker-7312adca75df78f3.d
   │  │  ├─ axum-4bea695fa3e55cee.d
   │  │  ├─ axum_core-408be2366e9d29a9.d
   │  │  ├─ bytes-fc4dbe8282c07cd0.d
   │  │  ├─ cfg_if-bc6f99dd165977f0.d
   │  │  ├─ dotenvy-655c250c222611db.d
   │  │  ├─ errno-29020f48b04b5859.d
   │  │  ├─ form_urlencoded-dc785d797d27590b.d
   │  │  ├─ futures_channel-43b8c47964d3ba0b.d
   │  │  ├─ futures_core-d9e332389104b8b0.d
   │  │  ├─ futures_task-8789ce6d8b0288ce.d
   │  │  ├─ futures_util-e4e001c6d80a258e.d
   │  │  ├─ http-4dea63e9bd91bced.d
   │  │  ├─ http_body-491c9b277a8225a1.d
   │  │  ├─ http_body_util-a102616318b9aa73.d
   │  │  ├─ http_server-71ac178811113b1f.d
   │  │  ├─ http_server-ae6d27d1cdf774ef.d
   │  │  ├─ httparse-a2ddefa97b563b92.d
   │  │  ├─ httpdate-f562b7c058b005db.d
   │  │  ├─ hyper-220125fd54d14d0e.d
   │  │  ├─ hyper_util-603ff0029e8cc086.d
   │  │  ├─ itoa-18241bc2233dfc62.d
   │  │  ├─ lazy_static-ec720e21de3dd27f.d
   │  │  ├─ libasync_trait-10e1815961dd2ce6.so
   │  │  ├─ libatomic_waker-7312adca75df78f3.rmeta
   │  │  ├─ libaxum-4bea695fa3e55cee.rmeta
   │  │  ├─ libaxum_core-408be2366e9d29a9.rmeta
   │  │  ├─ libbytes-fc4dbe8282c07cd0.rmeta
   │  │  ├─ libc-029dbd8d4c76489b.d
   │  │  ├─ libcfg_if-bc6f99dd165977f0.rmeta
   │  │  ├─ libdotenvy-655c250c222611db.rmeta
   │  │  ├─ liberrno-29020f48b04b5859.rmeta
   │  │  ├─ libform_urlencoded-dc785d797d27590b.rmeta
   │  │  ├─ libfutures_channel-43b8c47964d3ba0b.rmeta
   │  │  ├─ libfutures_core-d9e332389104b8b0.rmeta
   │  │  ├─ libfutures_task-8789ce6d8b0288ce.rmeta
   │  │  ├─ libfutures_util-e4e001c6d80a258e.rmeta
   │  │  ├─ libhttp-4dea63e9bd91bced.rmeta
   │  │  ├─ libhttp_body-491c9b277a8225a1.rmeta
   │  │  ├─ libhttp_body_util-a102616318b9aa73.rmeta
   │  │  ├─ libhttp_server-71ac178811113b1f.rmeta
   │  │  ├─ libhttp_server-ae6d27d1cdf774ef.rmeta
   │  │  ├─ libhttparse-a2ddefa97b563b92.rmeta
   │  │  ├─ libhttpdate-f562b7c058b005db.rmeta
   │  │  ├─ libhyper-220125fd54d14d0e.rmeta
   │  │  ├─ libhyper_util-603ff0029e8cc086.rmeta
   │  │  ├─ libitoa-18241bc2233dfc62.rmeta
   │  │  ├─ liblazy_static-ec720e21de3dd27f.rmeta
   │  │  ├─ liblibc-029dbd8d4c76489b.rmeta
   │  │  ├─ liblock_api-4337be56952e5e64.rmeta
   │  │  ├─ liblog-c37e0462752f1357.rmeta
   │  │  ├─ libmatchers-9c5a22dab0df1a1c.rmeta
   │  │  ├─ libmatchit-4f4bb755e11b2779.rmeta
   │  │  ├─ libmemchr-c1f717ae461c60e0.rmeta
   │  │  ├─ libmime-707fe96655585e0b.rmeta
   │  │  ├─ libmio-d85a0128fd60a1a7.rmeta
   │  │  ├─ libnu_ansi_term-4194f93a5f3136ca.rmeta
   │  │  ├─ libonce_cell-68ecb02160056e0a.rmeta
   │  │  ├─ libparking_lot-7a103d302e26f843.rmeta
   │  │  ├─ libparking_lot_core-ed0b2cc0b4013cc6.rmeta
   │  │  ├─ libpercent_encoding-51ffba506c82a2c0.rmeta
   │  │  ├─ libpin_project_lite-d6ba8a6bee90f552.rmeta
   │  │  ├─ libproc_macro2-6e361d414403e107.rlib
   │  │  ├─ libproc_macro2-6e361d414403e107.rmeta
   │  │  ├─ libquote-e3338ff9e189f247.rlib
   │  │  ├─ libquote-e3338ff9e189f247.rmeta
   │  │  ├─ libregex_automata-1cd8434d10240732.rmeta
   │  │  ├─ libregex_syntax-311d98b3bd96f231.rmeta
   │  │  ├─ librustversion-a5439b6abf9a0b66.so
   │  │  ├─ libryu-d361f931d71c243a.rmeta
   │  │  ├─ libscopeguard-b5b126953d4d1d57.rmeta
   │  │  ├─ libserde-1f3cb856bb3a5939.rmeta
   │  │  ├─ libserde_core-818d123c0c2cd104.rmeta
   │  │  ├─ libserde_derive-f29c52cf385f4baa.so
   │  │  ├─ libserde_json-64620c9eb58fc72c.rmeta
   │  │  ├─ libserde_path_to_error-4aa619c860e9fdf7.rmeta
   │  │  ├─ libserde_urlencoded-18988ce4fe9e0e54.rmeta
   │  │  ├─ libsharded_slab-54e6c9b696dfd18c.rmeta
   │  │  ├─ libsignal_hook_registry-9f2af5a162b77c82.rmeta
   │  │  ├─ libslab-484496a5e56d87ed.rmeta
   │  │  ├─ libsmallvec-20fb99f0fbe0246f.rmeta
   │  │  ├─ libsocket2-34eab01829be3f52.rmeta
   │  │  ├─ libsyn-60bdbb13700f6005.rlib
   │  │  ├─ libsyn-60bdbb13700f6005.rmeta
   │  │  ├─ libsync_wrapper-65042dc5e52f8f1b.rmeta
   │  │  ├─ libthread_local-8a78a5abf384d9d9.rmeta
   │  │  ├─ libtokio-972fb711261e38fd.rmeta
   │  │  ├─ libtokio_macros-9c469441d2fb900c.so
   │  │  ├─ libtower-10ba45041ade6f2c.rmeta
   │  │  ├─ libtower_layer-e19fd2325789fc43.rmeta
   │  │  ├─ libtower_service-4f4d55ac3672fdbb.rmeta
   │  │  ├─ libtracing-1ca410b79d5d1f69.rmeta
   │  │  ├─ libtracing_attributes-2e51a3d06b0e5052.so
   │  │  ├─ libtracing_core-a90a03f61da34714.rmeta
   │  │  ├─ libtracing_log-e193144326686db6.rmeta
   │  │  ├─ libtracing_subscriber-7340ce1a7a7ac51f.rmeta
   │  │  ├─ libunicode_ident-7b5331368ce720fe.rlib
   │  │  ├─ libunicode_ident-7b5331368ce720fe.rmeta
   │  │  ├─ libzmij-527c9cba665cd1bb.rmeta
   │  │  ├─ lock_api-4337be56952e5e64.d
   │  │  ├─ log-c37e0462752f1357.d
   │  │  ├─ matchers-9c5a22dab0df1a1c.d
   │  │  ├─ matchit-4f4bb755e11b2779.d
   │  │  ├─ memchr-c1f717ae461c60e0.d
   │  │  ├─ mime-707fe96655585e0b.d
   │  │  ├─ mio-d85a0128fd60a1a7.d
   │  │  ├─ nu_ansi_term-4194f93a5f3136ca.d
   │  │  ├─ once_cell-68ecb02160056e0a.d
   │  │  ├─ parking_lot-7a103d302e26f843.d
   │  │  ├─ parking_lot_core-ed0b2cc0b4013cc6.d
   │  │  ├─ percent_encoding-51ffba506c82a2c0.d
   │  │  ├─ pin_project_lite-d6ba8a6bee90f552.d
   │  │  ├─ proc_macro2-6e361d414403e107.d
   │  │  ├─ quote-e3338ff9e189f247.d
   │  │  ├─ regex_automata-1cd8434d10240732.d
   │  │  ├─ regex_syntax-311d98b3bd96f231.d
   │  │  ├─ rustversion-a5439b6abf9a0b66.d
   │  │  ├─ ryu-d361f931d71c243a.d
   │  │  ├─ scopeguard-b5b126953d4d1d57.d
   │  │  ├─ serde-1f3cb856bb3a5939.d
   │  │  ├─ serde_core-818d123c0c2cd104.d
   │  │  ├─ serde_derive-f29c52cf385f4baa.d
   │  │  ├─ serde_json-64620c9eb58fc72c.d
   │  │  ├─ serde_path_to_error-4aa619c860e9fdf7.d
   │  │  ├─ serde_urlencoded-18988ce4fe9e0e54.d
   │  │  ├─ sharded_slab-54e6c9b696dfd18c.d
   │  │  ├─ signal_hook_registry-9f2af5a162b77c82.d
   │  │  ├─ slab-484496a5e56d87ed.d
   │  │  ├─ smallvec-20fb99f0fbe0246f.d
   │  │  ├─ socket2-34eab01829be3f52.d
   │  │  ├─ syn-60bdbb13700f6005.d
   │  │  ├─ sync_wrapper-65042dc5e52f8f1b.d
   │  │  ├─ thread_local-8a78a5abf384d9d9.d
   │  │  ├─ tokio-972fb711261e38fd.d
   │  │  ├─ tokio_macros-9c469441d2fb900c.d
   │  │  ├─ tower-10ba45041ade6f2c.d
   │  │  ├─ tower_layer-e19fd2325789fc43.d
   │  │  ├─ tower_service-4f4d55ac3672fdbb.d
   │  │  ├─ tracing-1ca410b79d5d1f69.d
   │  │  ├─ tracing_attributes-2e51a3d06b0e5052.d
   │  │  ├─ tracing_core-a90a03f61da34714.d
   │  │  ├─ tracing_log-e193144326686db6.d
   │  │  ├─ tracing_subscriber-7340ce1a7a7ac51f.d
   │  │  ├─ unicode_ident-7b5331368ce720fe.d
   │  │  └─ zmij-527c9cba665cd1bb.d
   │  ├─ examples
   │  └─ incremental
   │     ├─ http_server-2d6twdks1c3eh
   │     │  ├─ s-hjuk6s5hwh-1xmntw0-3jmvuz2l9xeb5atrhbn0mwh4b
   │     │  │  ├─ dep-graph.bin
   │     │  │  ├─ query-cache.bin
   │     │  │  └─ work-products.bin
   │     │  └─ s-hjuk6s5hwh-1xmntw0.lock
   │     └─ http_server-3fj0e4v2v05gs
   │        ├─ s-hjul6m9t0y-0x6kzmt-bv4idppaj22tv2njaevwtpnfn
   │        │  ├─ dep-graph.bin
   │        │  ├─ query-cache.bin
   │        │  └─ work-products.bin
   │        └─ s-hjul6m9t0y-0x6kzmt.lock
   └─ flycheck0
      ├─ stderr
      └─ stdout

```