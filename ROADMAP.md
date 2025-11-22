# MediaClay Roadmap

**Mission:** To build a hyper-scalable, distributed structural engine ("Clay") that organizes information for the symbiotic era of human and AI interaction. MediaClay is not just a CMS or application backend; it is a universal taxonomy protocol designed to be a foundational data-application layer for intelligent systems.

**History:** Originally forged in the transitional era of the early year 2000 internet, MediaClay was a visionary system designed to liberate creative expression from the technical constraints of raw code. It operated as a proprietary engine that empowered non-technical users to build dynamic websites long before modern standards made such digital freedom commonplace.

## I. Architectural Philosophy

1.  **Agent-Native Design:** The system is built primarily to be read, understood, and manipulated by AI Agents (LLMs) via standard protocols (MCP), with human interfaces serving as a view layer on top.
2.  **Performance as a Feature:** We reject "database bottlenecks." The architecture decouples read/write paths, utilizes async streaming, and leverages Rust’s zero-cost abstractions to handle massive concurrency.
3.  **Universal Structure:** We collapse complex abstractions into a singular, flexible concept: **Clay Structures**. This allows for arbitrary depth, polymorphism, and relationship modeling without rigid schema constraints.
4.  **Protocol First:** Interaction happens via strictly typed, high-performance standards (gRPC, HTTP/3, GraphQL) rather than custom application logic.

---

## II. The Technology Stack (Rust)

We are utilizing a modern, distributed Rust stack designed for high availability and low latency.

* **Edge / Ingress:** **Pingora** (Cloudflare) - *Programmable proxy for HTTP/3 (QUIC) termination and edge security.*
* **API Gateway:** **Axum** (Tokio) - *The public face for REST and GraphQL interactions.*
* **Service Mesh:** **Tonic** (gRPC) - *High-speed binary communication between internal microservices.*
* **Runtime:** **Tokio** - *Asynchronous work-stealing runtime for maximum CPU saturation.*
* **Data Layer:**
    * **Primary:** **PostgreSQL** (via **SQLx**) utilizing `ltree` for hierarchy and `jsonb` for polymorphic data.
    * **Cache:** **Redis** (via `deadpool-redis`) for hot-path structure caching.
    * **Search:** **Meilisearch** or **Tantivy** for vector and full-text retrieval.

---

## III. Phase 1: The Foundation (Bedrock)
*Goal: Establish the distributed runtime environment, multi-tenancy, and the atomic unit of data.*

- [ ] **Repository Initialization:** Setup Cargo workspace with strict linting (`clippy::pedantic`) and dependency isolation.
- [ ] **Multi-Tenant Schema Design:**
    -   Implement the **Tenant Isolation** pattern. Every query requires a `tenant_id` (formerly "Solution ID").
    -   Define the `Tenants` table to manage distinct environments (e.g., "Corporate Site", "Intranet").
- [ ] **The "Clay Node" Schema:**
    -   Define the atomic `nodes` table using PostgreSQL `ltree` for high-speed hierarchy lookups.
    -   Implement the `properties` column using binary JSON (`jsonb`) to support unlimited, schema-less field definitions per node.
- [ ] **Identity & Access (IAM):**
    -   Implement a stateless authentication system (OIDC/PASETO) capable of verifying both Human Users and Agent Identities.

## IV. Phase 2: The Structural Engine (Core Services)
*Goal: Build the internal logic that manipulates the Clay structure. This is the "Brain" of the system.*

- [ ] **Tree Engine (gRPC):**
    -   `CreateNode`: Atomic insertion logic.
    -   `MoveNode`: Transactional subtree relocation (re-parenting a branch updates all descendants instantly).
    -   `LinkNode`: Implementation of "Shortcuts" (Symbolic Links) allowing a Node to exist in multiple locations simultaneously.
- [ ] **Node Lifecycle:**
    -   Implement **Versioning** (Optimistic Concurrency) to prevent overwrite conflicts.
    -   Implement **Status States** (`draft`, `review`, `published`) directly on the Node object.
- [ ] **Type System (Validation):**
    -   Implement a "Schema Definition" engine. While the DB is schema-less (`jsonb`), the Application Layer enforces rules (e.g., "An 'Article' node *must* have a 'Title'").

## V. Phase 3: The Management Layer (Control Plane)
*Goal: Unlock the first API features. These APIs allow administrators to configure the system, creating the playground for Agents.*

- [ ] **Tenant Administration API:**
    -   Endpoints to Create/Suspend Tenants ("Solutions") and assign Users to them.
- [ ] **Schema Management API:**
    -   Endpoints to define Node Types (e.g., "Define 'Product' type with fields 'Price' and 'SKU'").
    -   This replaces legacy "Element Field" management GUIs with a Headless API.
- [ ] **Content Management API (CRUD):**
    -   The standard REST/gRPC endpoints for Humans to manually edit Nodes and Properties.

## VI. Phase 4: The Consumption Layer (Agents & Humans)
*Goal: Expose the system to the world for high-speed consumption.*

- [ ] **Model Context Protocol (MCP) Server:**
    -   Expose MediaClay as a native "Tool" for LLMs.
    -   Enable Agents to query structure: `Clay.get_context("marketing_assets")`.
    -   Enable Agents to modify structure: `Clay.propose_reorganization(source="...", logic="...")`.
- [ ] **GraphQL Gateway:**
    -   The primary read-interface for human-facing frontends (React/SolidJS/Mobile).
    -   Offers flexible data shaping to reduce over-fetching.
- [ ] **Real-Time Event Bus:**
    -   Server-Sent Events (SSE) / WebSockets to push tree updates to connected clients instantly.

## VII. Infrastructure & Observability
*Goal: Production readiness for distributed scale.*

- [ ] **Containerization:** Distroless Docker images (targeting <50MB footprint).
- [ ] **Telemetry:** **OpenTelemetry (OTEL)** integration for distributed tracing (following a request from Edge -> Gateway -> DB).

---

## Future Horizons

* **Vector Embedding Integration:** Automatically generating embeddings for every Node in the Clay structure to allow semantic search.
* **Federated Clay:** Allowing distinct MediaClay instances to "mount" each other's structures remotely.
