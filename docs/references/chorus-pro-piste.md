# Chorus Pro & PISTE — French B2G e-invoicing platform

## What it is

**Chorus Pro** is the official French state platform through which every
supplier sends invoices to the **public sector** (central administrations,
local authorities, public institutions). It is operated by the **AIFE**
(Agence pour l'informatique financière de l'État) under the Ministry of
Finance.

**PISTE** (*Plateforme d'Intermédiation des Services pour la
Transformation de l'État*) is the French government **API gateway**. It
is the only entry point for calling Chorus Pro programmatically; direct
endpoints outside PISTE are not supported.

## Access modes

An organisation can submit invoices to Chorus Pro through three channels:

| Channel        | Use case                                                                 |
| -------------- | ------------------------------------------------------------------------ |
| **Portal**     | Manual web UI for small volumes.                                         |
| **EDI**        | Batch / file-based automation (X12, EDIFACT, flat files).                |
| **API (PISTE)**| Programmatic real-time integration — the mode `einvoice-rs` targets.    |

## Accepted invoice formats

Chorus Pro accepts multiple EN 16931-compliant payloads. The relevant
ones for this project:

| Format                        | Description                                            |
| ----------------------------- | ------------------------------------------------------ |
| **Factur-X**                  | Hybrid PDF/A-3 + CII XML (all profiles)                |
| **UBL Invoice**               | OASIS UBL 2.1 `Invoice` document                       |
| **CII**                       | UN/CEFACT Cross Industry Invoice (plain XML)           |

Historical formats (Chorus Pro 1.x) such as *MINIMAL* XML flavours are
still accepted but superseded by Factur-X and UBL for new integrations.

## API authentication: OAuth2 via PISTE

PISTE exposes Chorus Pro (and many other public APIs) behind a standard
**OAuth2** gateway.

| Step                   | Detail                                                                              |
| ---------------------- | ----------------------------------------------------------------------------------- |
| 1. Register            | Create a PISTE account, register an application, get `client_id` / `client_secret` |
| 2. Subscribe to API    | Subscribe the application to the Chorus Pro APIs (sandbox + production)             |
| 3. Obtain token        | `POST {base}/oauth/api/v1/token` with `grant_type=client_credentials` and `scope=openid` |
| 4. Call API            | `Authorization: Bearer <token>` on every Chorus Pro endpoint                         |
| 5. Refresh             | Tokens typically live ~1h — re-issue another `client_credentials` request.          |

Two base URLs exist:

| Environment | Base URL                                  |
| ----------- | ----------------------------------------- |
| Sandbox     | `https://sandbox-api.piste.gouv.fr`       |
| Production  | `https://api.piste.gouv.fr`               |

## Key Chorus Pro endpoints (facturation v1)

The exact paths live under the `cpro` namespace on PISTE. The ones a
supplier needs most often:

| Verb | Path                                  | Purpose                                                       |
| ---- | ------------------------------------- | ------------------------------------------------------------- |
| POST | `/cpro/factures/v1/deposer`           | Deposit an invoice (multipart: file + metadata).              |
| GET  | `/cpro/factures/v1/statuts`           | Retrieve the lifecycle status of a previously deposited file. |
| GET  | `/cpro/factures/v1/rechercher`        | Search deposited invoices by supplier / date / status.        |
| GET  | `/cpro/structures/v1/rechercher`      | Search public-sector recipient structures (SIRET, service).   |

Always check the PISTE console for the current path and payload — the
interface evolves over time.

## Invoice lifecycle statuses

Chorus Pro advances each deposited invoice through a well-known state
machine. The exact names are French, carried verbatim in API responses:

- `deposee` — initial deposit accepted by Chorus Pro
- `mise_a_disposition` — made available to the recipient
- `prise_en_charge` — recipient acknowledged receipt
- `mandatee` — authorised for payment by the recipient
- `mise_en_paiement` — payment instruction issued
- `comptabilisee` — accounted for
- `suspendue` — suspended (missing info)
- `rejetee` — rejected

This lifecycle is what the `invoice_events` table in the initial
migration is designed to store.

## Relationship with the 2026 reform

Today Chorus Pro only handles **B2G** (business → public sector). Under
the French **2026 reform**, it evolves into the **PPF (Portail Public de
Facturation)** which orchestrates **B2B** invoices as well, alongside
certified **PDPs** (Plateformes de Dématérialisation Partenaires). Any
integration code written today against the Chorus Pro APIs will remain
relevant for the PPF, with expanded scope.

See [`french-2026-reform.md`](french-2026-reform.md).

## Implementation notes for `einvoice-rs`

- `crates/delivery/src/chorus_pro.rs` already exposes a
  `ChorusProClient` with an `authenticate()` + `deposit()` API. Both
  methods are stubs returning `Error::Delivery`.
- Implement `authenticate` first against the sandbox — `reqwest` with
  `client_credentials`, store the token + expiry in the client.
- The `deposit` call is a multipart POST; keep the raw response so the
  external ID can be stored in `invoice_events.payload`.
- Never log `client_secret` or `Authorization` headers.

## Upstream references

- Chorus Pro community portal — <https://communaute.chorus-pro.gouv.fr/>
- PISTE portal — <https://piste.gouv.fr/>
- PISTE documentation — <https://developer.aife.economie.gouv.fr/>
- AIFE — <https://www.aife.economie.gouv.fr/>
