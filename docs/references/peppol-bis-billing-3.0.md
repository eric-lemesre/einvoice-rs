# PEPPOL BIS Billing 3.0 & the PEPPOL network

## What it is

**PEPPOL BIS Billing 3.0** is the Business Interoperability Specification
maintained by **OpenPeppol** that describes how to exchange invoices and
credit notes across the PEPPOL network while remaining compliant with
**EN 16931**. It restricts and profiles **UBL 2.1** to produce a
deterministic subset that every PEPPOL access point can understand.

**PEPPOL** itself (Pan-European Public Procurement Online) is not a
format: it is the **eDelivery network** that carries the documents, based
on the **4-corner model** and **AS4** transport.

## Customization & Profile identifiers

The billing specification is identified by two URNs that are carried in
every UBL invoice:

| UBL element          | Value                                                                                |
| -------------------- | ------------------------------------------------------------------------------------ |
| `cbc:CustomizationID`| `urn:cen.eu:en16931:2017#compliant#urn:fdc:peppol.eu:2017:poacc:billing:3.0`         |
| `cbc:ProfileID`      | `urn:fdc:peppol.eu:2017:poacc:billing:01:1.0`                                        |

The `ProfileID` refers to the **Billing 01** business process: "the
supplier sends a commercial invoice to the customer".

## The 4-corner model

```
 ┌───────────┐      ┌───────────┐       ┌───────────┐      ┌───────────┐
 │    C1     │──────│    C2     │──AS4──│    C3     │──────│    C4     │
 │  Sender   │ sends│  Sender   │       │ Receiver  │ deliv│ Receiver  │
 │ (business)│ UBL  │   AP      │       │    AP     │ UBL  │ (business)│
 └───────────┘      └───────────┘       └───────────┘      └───────────┘
                         ▲                   ▲
                         │                   │
                         └──── SML / SMP ────┘
                               dynamic discovery
```

| Corner | Role                                                                         |
| ------ | ---------------------------------------------------------------------------- |
| **C1** | End user sending the invoice (ERP, accounting software).                     |
| **C2** | Sender's **Access Point** — a certified Service Provider's AS4 server.       |
| **C3** | Receiver's **Access Point** — likewise certified.                            |
| **C4** | End user receiving the invoice.                                              |

A business never connects to more than its own Access Point. All other
trading partners are reached transparently through the PEPPOL network.

## Discovery: SML + SMP

PEPPOL uses a **two-step lookup**:

1. **SML — Service Metadata Locator**. A single central registry operated
   by OpenPeppol. Given a participant identifier, it uses DNS to return
   the URL of the participant's SMP.
2. **SMP — Service Metadata Publisher**. A distributed directory,
   typically hosted by the Service Provider, that lists which document
   types the receiver can accept, which business processes they
   implement, and which AS4 endpoint (Access Point) serves them.

The sender's Access Point performs SML→SMP lookup in milliseconds before
sending the AS4 message.

## Transport: PEPPOL AS4 Profile 2.0.x

The transport profile is built on the **CEF eDelivery AS4** profile with
PEPPOL-specific restrictions:

- **Exchange pattern**: exclusively **One-Way / Push**. Pull and
  synchronous exchanges are **forbidden**.
- **TLS**: mandatory **TLS 1.2 or higher** on **port 443**.
- **Certificates**: must be issued by the **PEPPOL PKI** and comply with
  the PEPPOL Policy for Transport Security.
- **Payloads**: always carried as **MIME attachments** (required for AS4
  compression).
- **SBDH**: the **Standard Business Document Header** is mandatory on
  every transmission. Standalone SBDH is not supported.
- **Transport profile ID in SMP**: `peppol-transport-as4-v2_0`.

Receiving Access Points must configure one or more **P-Modes** covering
every `(document type, process)` combination registered in their SMP.
Senders must implement the **Dynamic Sender Profile**, building their
P-Mode at runtime from SMP-signed service metadata.

## Message security

- **Digital signatures** on every AS4 message, backed by the PEPPOL PKI.
- **Encryption** at the AS4 level (on top of TLS) for confidentiality and
  non-repudiation.
- **Peppol Service Provider Agreement** binds every access point
  operator contractually.

## Participant identifiers (scheme table)

Every PEPPOL participant has an identifier of the form `scheme:value`.
Common schemes relevant for FR/EU implementations:

| Scheme   | Description                                     |
| -------- | ----------------------------------------------- |
| `0088`   | GLN (Global Location Number, GS1)               |
| `0192`   | Norwegian organisation number                   |
| `0009`   | SIRENE (France, INSEE)                          |
| `0002`   | System Information et Répertoire (SIRET)        |
| `9906`   | IT:VAT — Italian VAT                            |
| `9907`   | IT:CF — Italian fiscal code                     |

Always check the **PEPPOL Policy for use of Identifiers** for the current
catalogue.

## Implementation notes for `einvoice-rs`

- The `einvoice-ubl` crate already knows the PEPPOL BIS 3.0 customization
  and profile IDs — make sure the URNs stay in sync with this document.
- The `einvoice-delivery` crate does **not** currently implement AS4.
  That would require either a full AS4 stack (complex) or delegating
  to a commercial / open-source PEPPOL access point via its own API.
  For now, PEPPOL delivery is **out of scope** — the roadmap covers
  email and Chorus Pro only.
- When AS4 is eventually added: remember the one-way/push, TLS 1.2+,
  SBDH-mandatory and `peppol-transport-as4-v2_0` rules listed above.

## Upstream references

- PEPPOL BIS Billing 3.0 — <https://docs.peppol.eu/poacc/billing/3.0/>
- PEPPOL AS4 Profile — <https://docs.peppol.eu/edelivery/as4/specification/>
- PEPPOL eDelivery portal — <https://docs.peppol.eu/edelivery/>
- OpenPeppol homepage — <https://peppol.eu/>
- 4-corner model overview (E-Rechnung Bund) —
  <https://e-rechnung-bund.de/en/faq/peppols-technical-solution-the-four-corner-model/>
