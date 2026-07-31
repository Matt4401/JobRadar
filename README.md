<h1 align="center">JobRadar</h1>

<p align="center">
  <font color="red"><strong>Important: the entire organizational aspect of this project (issues and pull requests) is private and therefore invisible to anyone who is not a collaborator.</strong></font>
</p>

<p align="center">
  <strong>Self-hosted job-offer aggregator, written entirely in Rust.</strong><br>
  Targeted scraping of job boards, configurable filtering, MySQL storage and a lightweight web UI.
</p>

<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-edition%202021-000000?logo=rust&logoColor=white">
  <img alt="Axum" src="https://img.shields.io/badge/Axum-0.8-6e40c9">
  <img alt="SQLx" src="https://img.shields.io/badge/SQLx-0.8-4169E1">
  <img alt="MySQL" src="https://img.shields.io/badge/MySQL-8.0-00758F?logo=mysql&logoColor=white">
  <img alt="Docker" src="https://img.shields.io/badge/Docker-Compose-2496ED?logo=docker&logoColor=white">
</p>

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Architecture](#architecture)
- [Tech Stack](#tech-stack)
- [Project Structure](#project-structure)
- [Prerequisites](#prerequisites)
- [Installation & Usage](#installation--usage)
- [Configuration](#configuration)
- [Performance](#performance)
- [Security](#security)
- [Roadmap](#roadmap)

---

## Overview

Job hunting means visiting the same boards every day, manually filtering dozens of listings and keeping only those that actually match your criteria. **JobRadar automates this monitoring**: it fetches offers from job boards, applies customizable filters (keywords, location, contract type, salary...), deduplicates the results and centralizes everything in a single web interface.

The project honors each scraped site's `robots.txt` and only accesses explicitly allowed URLs.

## Features

- **Targeted scraping** — offers are extracted from `JSON-LD` structured data (`schema.org/JobPosting`), which is resilient to layout changes.
- **Configurable filtering** — job keywords, companies, locations, contract types and minimum salary, defined in a TOML file or from the web UI.
- **Deduplication** — the offer URL is the unique key (`INSERT ... ON DUPLICATE KEY UPDATE`): a known offer is updated, never duplicated.
- **Web interface** — list offers, trigger a new scrape and edit filters, served by an asynchronous Axum server.
- **Persistence** — MySQL storage with versioned SQL migrations and privilege separation (application user vs. migrator).

## Architecture

JobRadar is organized as a **multi-crate Cargo workspace**, each crate holding a single responsibility:

```mermaid
flowchart LR
    subgraph Sources
        HW[Job boards<br/>e.g. HelloWork]
    end

    subgraph JobRadar
        SC[scraper<br/>fetch + filter]
        WEB[web<br/>Axum UI]
        DB[(db<br/>MySQL 8)]
        SH[shared<br/>common utilities]
    end

    U[User] -->|browser :3000| WEB
    HW -->|HTTP / JSON-LD| SC
    SC -->|filtered offers| DB
    WEB -->|read / re-run| DB
    WEB -->|triggers| SC
    SC -. env, TOML, HTTP .-> SH
    WEB -. env .-> SH
```

| Crate | Responsibility |
|-------|----------------|
| `scraper` | Page fetching, JSON-LD parsing, filter application, persistence. Exposed as both a library **and** a binary. |
| `web` | Axum web server: offer listing, scrape re-run, filter editing. |
| `db` | Data access layer (SQLx), models, a small internal ORM and migration handling. |
| `shared` | Cross-cutting utilities: environment variable loading, TOML parsing, HTTP client. |
| `api` | Dedicated HTTP entry point (reserved for a future evolution). |

## Tech Stack

| Area | Technologies |
|------|--------------|
| **Language** | Rust (2021 edition) |
| **Async runtime** | Tokio |
| **Web server** | Axum 0.8 |
| **Database** | MySQL 8 via SQLx 0.8 (`runtime-tokio-rustls`) |
| **HTTP client** | reqwest 0.13 (rustls) |
| **Parsing** | serde / serde_json (JSON-LD), regex, toml |
| **Infrastructure** | Docker Compose (MySQL + phpMyAdmin) |
| **Configuration** | TOML files + environment variables (`dotenvy`) |

## Project Structure

```
JobRadar/
├── api/                 # HTTP binary (future evolution)
├── scraper/             # Scraping, parsing, filtering (lib + bin)
│   ├── src/parser/      # Per-site parsing strategies
│   └── restrictions/    # Reference robots.txt per site
├── db/                  # Data access, models, migrations
│   ├── migrations/      # Versioned SQL scripts
│   └── docker/initdb/   # MySQL init (user creation)
├── web/                 # Axum web interface
├── shared/              # Common utilities
├── docker-compose.yml   # MySQL + phpMyAdmin
├── scraper_config.toml  # Sites to scrape
└── scraper_filters.toml # Filtering criteria
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (recent stable toolchain)
- [Docker](https://docs.docker.com/get-docker/) and Docker Compose

## Installation & Usage

### 1. Clone and configure the environment

```bash
git clone <repository-url> JobRadar
cd JobRadar
cp .env.example .env
```

Edit `.env` to set the passwords (replace the bracketed placeholders):

```dotenv
MYSQL_ROOT_PASSWORD=...
JOBRADAR_APP_PASSWORD=...
JOBRADAR_MIGRATOR_PASSWORD=...
DATABASE_URL=mysql://jobradar_app:...@localhost:3306/jobradar
MIGRATION_DATABASE_URL=mysql://jobradar_migrator:...@localhost:3306/jobradar
```

### 2. Start the database

```bash
docker compose up -d
```

MySQL is available on port `3306` and phpMyAdmin at [http://localhost:8080](http://localhost:8080).

### 3. Apply the schema

The migration scripts live in `db/migrations/` and create, among others, the `job_offers` table. Apply them to the freshly started database (e.g. via phpMyAdmin or the `mysql` client).

### 4. Run a scrape

```bash
cargo run -p scraper
```

The scraper reads `scraper_config.toml` and `scraper_filters.toml`, fetches the offers and stores them in the database.

### 5. Start the web interface

```bash
cargo run -p web
```

The UI is available at [http://127.0.0.1:3000](http://127.0.0.1:3000): browse offers, trigger a new search and adjust the filters.

> For optimal performance, build in release mode: `cargo build --release -p web`.

## Configuration

### `scraper_config.toml` — sites to scrape

```toml
sites = ["hello_work"]      # enabled sites
log_system = false          # logging (upcoming)
email_system = false        # email notifications (upcoming)
crash_on_scrapping_errors = false
```

### `scraper_filters.toml` — filtering criteria

All text matches are **case-insensitive** and work on a "contains" basis. An empty list disables the corresponding filter.

| Field | Description |
|-------|-------------|
| `max_offers` | Maximum number of offers kept per site |
| `job_keywords` | Keywords expected in the job title |
| `companies` | Accepted companies |
| `locations` | Accepted cities / regions |
| `contract_types` | Contract types (`schema.org employmentType`: `FULL_TIME`, `INTERN`, `TEMPORARY`...) |
| `min_salary` | Minimum gross salary |
| `exclude_without_salary` | Excludes offers with no salary listed |

These filters can also be edited directly from the web interface (`/filters`).

## Performance

Measured on a development machine (laptop, MySQL in a Docker container), binary compiled in `--release`, using concurrent HTTP load tests.

| Metric | Value |
|--------|-------|
| Throughput — dynamic route (database-backed) | **~1,000 req/s** |
| Throughput — in-memory route (no database) | **~43,000 req/s** |
| Median latency | **8 ms** (with DB) · **0.5 ms** (without DB) |
| Memory footprint (RSS) at idle | **~30 MB** |
| Deployment binary | **~15 MB**, self-contained (no runtime dependency) |

> The gap between the two throughputs shows that the bottleneck lies on the database side rather than the HTTP server, revealing an identified optimization opportunity (shared connection pool via Axum application state).

## Security

- **100% parameterized SQL queries** — all values are sent separately from the SQL text (via SQLx), eliminating SQL injection.
- **Systematic HTML escaping** — every scraped value is escaped before rendering, protecting against XSS attacks.
- **Privilege separation** — an application user (DML only) distinct from the migration user (DDL).
- **`robots.txt` compliance** — only URLs explicitly allowed by each site are scraped.

Matthieu Coraleau @2026.