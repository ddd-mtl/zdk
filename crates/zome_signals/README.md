# Zome Signals

This module implements a foundation layer for a signal-based architecture for Holochain applications.
This library is to be included directly in a coordinator zome as a dependency and not as a standalone coordinator zome.

## Usage

This module defines `recv_remote_signal()` which must be initialized in your zome's `init()` callback by calling `create_signal_cap_grant()`.

This module also defines `cast_tip()` and `synchronize_tip()` zome functions intended to be called by UI components for sending signals to remote agents.

Use `emit_zome_signal()` in your zome functions or callbacks to send a signal to app clients.

A call to `attest_post_commit()` must be added in your zome's `post_commit()` function to receive signals during this callback.


## Core Design Principle

 Have data transmitted from zome WASM to UI primarily via *Signals* rather than
 return values of zome functions. This approach provides several benefits:

 - **Simplified UI Processing**: UI components handle data through a consistent signal
   handling mechanism regardless of where the data originates
 - **Consistency**: Since zome callbacks can only send data via signals,
   using signals for all zome-to-UI communication creates a unified approach
 - **Decoupling**: The signal pattern decouples zome function execution from UI updates

 ### Data Flow

 1. UI components register signal handlers to process incoming data
 1. UI components calls zome functions to trigger data processing
 1. During Zome functions or callbacks execution, signals containing relevant data are emitted by using a consistent signaling protocol.
 2. UI components receive the signals and process them accordingly, updating the UI 


### Validation

Each data point received by Signal has a validation marker indication how trustable the data is.

## Terminology

A **Signal** is a carrier for serialized data (`Vec<u8>`) transmitted from the zome wasm to either app clients (e.g. UI components) via `emit_signal()` or remote agents via `remote_signal()`.

When a *Signal* is used to communicate to remote agents it is called a ***Tip***.

When a *Signal* is used to communicate to app clients it is called an ***Attestation***.

A *Signal* is **emitted**, while a *Tip* is **cast** and an *Attestation* is **attested**.

A *Signal* is composed of multiple **Pulses**, where each **Pulse** represents an individual data point.
 - A ***System Pulse*** is a *Pulse* informing about an internal app-agnostic event.
 - An ***Entry Pulse*** is a *Pulse* informing about a Zome Entry.
 - A ***Link Pulse*** is a *Pulse* informing about a Zome Link.
 - A ***Value Pulse*** is a *Pulse* transmitting App specific data in a generic key-value pair as string.
 - A ***Custom Pulse*** is a *Pulse* transmitting App specific data in an app-specific format.

A transmission is either **inbound** if its <ins>from</ins> a remote agent or **outbound** if its <ins>to</ins> a remote agent.

A transmission is either **local** if its between zomes on the same source-chain or **remote** if its between zomes on different source-chains.

A transmission is **synchronous** if it halts execution until a response is received.

A transmission is a **request** if it's expecting a response with data.



## Building

1. [Install rustup](https://rustup.rs/) and the `wasm32` target with: ``rustup target add wasm32-unknown-unknown``
1. Run ``cargo build --release --target wasm32-unknown-unknown``
