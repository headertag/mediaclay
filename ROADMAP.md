# MediaClay Roadmap

**Mission:** To build a hyper-scalable, distributed structural engine ("Clay") that organizes information for the symbiotic era of human and AI interaction. MediaClay is not just a CMS or application backend; it is a universal taxonomy protocol designed to be a foundational data-application layer for intelligent systems.

**History:** Originally forged in the transitional era of the early year 2000 internet, MediaClay was a visionary system designed to liberate creative expression from the technical constraints of raw code. It operated as a proprietary engine that empowered non-technical users to build dynamic websites long before modern standards made such digital freedom commonplace.

## I. Architectural Philosophy

1.  **Agent-Native Design:** The system is built primarily to be read, understood, and manipulated by AI Agents (LLMs) via standard protocols (MCP), with human interfaces serving as a view layer on top.
2.  **Performance as a Feature:** We reject "database bottlenecks." The architecture decouples read/write paths, utilizes async streaming, and leverages Rust’s zero-cost abstractions to handle massive concurrency and distributed and highly scalable redis caches for read oprations.
3.  **Universal Structure:** We collapse complex abstractions into a singular, flexible concept: **Clay Structures**. This allows for arbitrary depth, polymorphism, and relationship modeling without rigid schema constraints.
4.  **Protocol First:** Interaction happens via strictly typed, high-performance standards (gRPC, HTTP/3, GraphQL) rather than custom application logic for maximum leverage.

---

## II. The Technology Stack (Rust)

We are utilizing a modern, distributed Rust stack designed for high availability and low latency.

* **Edge / Ingress:** **Pingora** (Cloudflare)
    * *Role:* Programmable proxy for HTTP/3 (QUIC) termination, load balancing, and edge security.
* **API Gateway:** **Axum** (Tokio)
    * *Role:* The public face for REST and GraphQL interactions.
* **Service Mesh:** **Tonic** (gRPC)
    * *Role:* High-speed, strongly-typed binary communication between internal microservices.
* **Runtime:** **Tokio**
    * *Role:* Asynchronous work-stealing runtime for maximum CPU saturation.
* **Data Layer:**
    * **Primary:** **PostgreSQL** (managed via **SQLx** for compile-time safety) utilizing `ltree` for high-performance hierarchy lookups.
    * **Cache:** **Redis** (via `deadpool-redis`) for hot-path structure caching.
    * **Search:** **Meilisearch** or **Tantivy** for full-text and vector retrieval.

---

## III. Phase 1: The Foundation (Bedrock)
*Goal: Establish the distributed runtime environment and the atomic unit of data.*

- [ ] **Repository Initialization:** Setup Cargo workspace with strict linting (`clippy::pedantic`) and dependency isolation.
- [ ] **Schema Design (The Clay Node):**
    -   Define the atomic `Node` structure (the fundamental unit of MediaClay).
    -   Implement high-speed tree storage logic (e.g., Materialized Path or Nested Set) optimized for read-heavy agent workloads.
    -   Implement polymorphic properties using binary JSON (JSONB) to support any content type.
- [ ] **Identity & Access (IAM):**
    -   Implement a stateless authentication system (OIDC/PASETO) capable of verifying both Human Users and Agent Identities.

## IV. Phase 2: The Core Services
*Goal: Build the engines that manipulate the Clay structure.*

- [ ] **Structure Service (gRPC):**
    -   Atomic operations: `Create`, `Move`, `Clone`, `Prune`.
    -   Transactional integrity for subtree operations (moving huge branches instantly).
- [ ] **Query Engine (Read Path):**
    -   Implement "Cursor-based Streaming" to deliver massive datasets to clients byte-by-byte without RAM spikes.
    -   Implement aggressive "Cache-Aside" strategies to protect the database.
- [ ] **Event Bus:**
    -   Implement a pub/sub system so changes in the Taxonomy propagate instantly to connected Agents and Clients.

## V. Phase 3: The Agent & Human Interfaces
*Goal: Expose the system to the world.*

- [ ] **Model Context Protocol (MCP) Server:**
    -   Expose MediaClay as a native "Tool" for LLMs (Claude, ChatGPT, Local Models).
    -   Enable Agents to query structure: `Clay.get_context("project_alpha")`.
    -   Enable Agents to modify structure: `Clay.propose_reorganization(source="...", logic="...")`.
- [ ] **GraphQL API:**
    -   The primary interface for human-facing frontends (React/SolidJS/Mobile).
    -   Offers flexible data shaping to reduce over-fetching.
- [ ] **Real-Time Streams:**
    -   Server-Sent Events (SSE) / WebSockets for live collaborative updates.

## VI. Phase 4: Infrastructure & Observability
*Goal: Production readiness for distributed scale.*

- [ ] **Containerization:** Distroless Docker images (targeting <50MB footprint).
- [ ] **Orchestration:** Kubernetes manifests for auto-scaling Node Services based on CPU load.
- [ ] **Telemetry:**
    -   **OpenTelemetry (OTEL)** integration for distributed tracing (following a request from Edge -> Gateway -> DB).
    -   Metrics for "Tree Traversal Time" and "Agent Response Latency."

---

## Future Horizons

* **Vector Embedding Integration:** Automatically generating embeddings for every Node in the Clay structure to allow semantic search ("Find me concepts related to 'happiness' within this hierarchy").
* **Federated Clay:** Allowing distinct MediaClay instances to "mount" each other's structures remotely, creating a global, decentralized taxonomy.
