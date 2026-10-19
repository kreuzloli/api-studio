# Kafka Studio

A lightweight, native Apache Kafka desktop client built with **Rust and GPUI**.

Kafka Studio aims to provide a fast, clean, and intuitive interface for exploring Kafka clusters, inspecting messages, and monitoring consumer groups.

> 🚧 **Work in Progress** — Kafka Studio is currently in early development. Features and architecture may change.

## Features

### Planned for v1.0

- **Cluster Management** — Connect to and manage multiple Kafka clusters.
- **Topic Explorer** — Browse topics, partitions, configurations, and metadata.
- **Message Viewer** — Inspect Kafka messages with support for JSON and plain text.
- **Message Producer** — Publish messages to selected topics.
- **Consumer Groups** — Monitor consumer groups, offsets, and consumer lag.
- **Message Search** — Filter messages by partition, offset, and content.
- **Local Configuration** — Save cluster connection profiles locally.
- **Native Desktop UI** — GPU-accelerated interface powered by GPUI.

## Tech Stack

| Technology | Purpose |
|---|---|
| Rust | Core programming language |
| GPUI | Native desktop UI framework |
| rust-rdkafka | Apache Kafka client |
| Tokio | Asynchronous runtime |
| Serde | Serialization and deserialization |
| SQLite / SQLx | Local configuration and persistence |
| Tracing | Logging and diagnostics |

## Architecture

Kafka Studio follows a modular architecture that separates the user interface, application logic, Kafka operations, and local storage.

```text
kafka-studio/
├── Cargo.toml
├── crates/
│   ├── kafka-studio-app/       # GPUI desktop application
│   ├── kafka-studio-core/      # Domain models and application logic
│   ├── kafka-studio-kafka/     # Kafka client integration
│   └── kafka-studio-storage/   # Local persistence
├── docs/                       # Documentation
└── README.md
```

This structure represents the planned architecture and will be introduced incrementally during development.

## Getting Started

### Prerequisites

- macOS (initial development target)
- Rust stable toolchain
- Xcode Command Line Tools
- A running Apache Kafka cluster (required for Kafka-related features)

### Build from Source

```bash
git clone https://github.com/kreuzloli/kafka-studio.git
cd kafka-studio

cargo build
cargo run
```

> The repository is in early development. Building and running the application does not yet imply that Kafka functionality is implemented.

## Roadmap

- [ ] Phase 1: GPUI application foundation
- [ ] Phase 2: Kafka cluster connection management
- [ ] Phase 3: Topic and partition explorer
- [ ] Phase 4: Message
