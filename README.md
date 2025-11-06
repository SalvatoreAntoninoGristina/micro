# Microservizio di Raccolta Metriche in Rust

[![CI/CD Pipeline](https://github.com/SalvatoreAntoninoGristina/micro/actions/workflows/ci.yml/badge.svg)](https://github.com/TUO_USERNAME/TUO_REPO/actions/workflows/ci.yml)
[![Build & Push Docker](https://github.com/SalvatoreAntoninoGristina/micro/actions/workflows/cd.yml/badge.svg)](https://github.com/TUO_USERNAME/TUO_REPO/actions/workflows/cd.yml)

Un microservizio backend ad alte prestazioni, scritto in Rust, progettato per ricevere, archiviare e servire dati di metriche.

Questo progetto è un'applicazione "pronta per la produzione" che dimostra competenze nell'ecosistema Rust moderno, nella containerizzazione con Docker e nelle pipeline CI/CD con GitHub Actions.


## 🎯 Obiettivo del Progetto

L'obiettivo principale è costruire un servizio robusto ed efficiente che simuli un caso d'uso aziendale reale. Il focus è sulla **qualità del codice**, sulla **performance** (per cui Rust è ideale) e sulle **best practice DevOps**.

Questo progetto dimostra competenza in:
* **Backend Development:** Creazione di un'API RESTful sicura e veloce con **Rust** (usando `axum`).
* **Interazione con Database:** Utilizzo di `sqlx` per query asincrone e sicure (controllate a tempo di compilazione) verso **PostgreSQL**.
* **Containerizzazione:** Creazione di immagini Docker ottimizzate (multi-stage) per uno small footprint e sicurezza.
* **Infrastruttura come Codice (IaC):** Utilizzo di `docker-compose` per definire e avviare l'intero ambiente di sviluppo (API + Database) con un solo comando.
* **DevOps e CI/CD:** Automazione di test, linting, build e pubblicazione delle immagini tramite **GitHub Actions**.

---

## 🛠️ Stack Tecnologico

* **Linguaggio:** Rust (Stable)
* **Web Framework:** `axum`
* **Runtime Asincrono:** `tokio`
* **Database:** PostgreSQL
* **Driver Database:** `sqlx` (Async, Type-safe)
* **Serializzazione:** `serde` (per JSON)
* **Container:** Docker & Docker Compose
* **CI/CD:** GitHub Actions

---

## 🚀 Come Avviare il Progetto (Running Locally)

L'intero ambiente è gestito tramite Docker Compose. È il modo più semplice e raccomandato per eseguire il progetto.

### Prerequisiti
* [Docker](https://www.docker.com/get-started)
* [Docker Compose](https://docs.docker.com/compose/install/) (spesso incluso con Docker)
* (Opzionale) [Rust Toolchain](https://rustup.rs/) (solo se si desidera eseguire senza Docker)

### Metodo 1: Docker Compose (Consigliato)

Questo metodo avvierà sia l'API Rust che il database PostgreSQL.

1.  **Clonare il repository:**
    ```bash
    git clone [https://github.com/TUO_USERNAME/TUO_REPO.git](https://github.com/TUO_USERNAME/TUO_REPO.git)
    cd TUO_REPO
    ```

2.  **Creare il file `.env`:**
    Copia il file di esempio `.env.example` in un nuovo file `.env`. Questo file contiene le variabili d'ambiente (come la password del DB) che `docker-compose` userà.
    ```bash
    cp .env.example .env
    ```
    *(Opzionale: puoi modificare i valori in `.env` se lo desideri, ma quelli di default funzioneranno localmente).*

3.  **Avviare i servizi:**
    ```bash
    docker-compose up --build
    ```
    * `--build` forza la ri-costruzione dell'immagine Docker della tua app Rust.
    * L'API sarà disponibile all'indirizzo `http://localhost:8080` (o qualsiasi porta tu abbia configurato).

### Metodo 2: Esecuzione Manuale (Sviluppo)

1.  **Avviare il Database:**
    Puoi usare Docker per avviare solo Postgres:
    ```bash
    docker run --name metric-db -p 5432:5432 \
    -e POSTGRES_PASSWORD=password \
    -e POSTGRES_USER=user \
    -e POSTGRES_DB=metrics \
    -d postgres
    ```

2.  **Configurare l'Ambiente:**
    Assicurati di avere un file `.env` leggibile dall'applicazione (o esporta le variabili). La variabile `DATABASE_URL` deve puntare al DB.
    ```
    DATABASE_URL=postgres://user:password@localhost:5432/metrics
    ```

3.  **Eseguire l'applicazione Rust:**
    ```bash
    cargo run
    ```

---

## 🤖 API Endpoints

| Metodo | Endpoint | Corpo (Esempio) | Descrizione |
| :--- | :--- | :--- | :--- |
| `POST` | `/metrics` | `{"name": "page_load_ms", "value": 120.5, "tags": {"page": "/home"}}` | Invia una nuova metrica al servizio. |
| `GET` | `/metrics` | *N/A* | Recupera un elenco delle ultime metriche registrate. |
| `GET` | `/health` | *N/A* | Endpoint di health check per il servizio. Restituisce `200 OK`. |

### Esempio di utilizzo (con `curl`)

**Inviare una metrica:**
```bash
curl -X POST 'http://localhost:8080/metrics' \
-H 'Content-Type: application/json' \
-d '{
    "name": "user_login",
    "value": 1,
    "tags": {
        "status": "success"
    }
}'
