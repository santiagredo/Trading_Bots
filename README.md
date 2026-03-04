# Trading Bots

A modular, real-time, multi-strategy trading engine built in Rust, designed to execute automated trading strategies on Binance with a configurable runtime and desktop control panel.

> Built for performance, extensibility, and full runtime control.

---

## Overview

**Trading Bots** is a desktop-first automated trading system that allows users to define and execute multiple strategies based on price and percentage movements.

The engine evaluates strategies in real time using Binance WebSocket market feeds and executes buy/sell actions when conditions are met.

The system is designed to:

- Support multiple concurrent strategies
- Persist strategies in a database
- Load strategies into memory at runtime
- Execute evaluations per incoming market event
- Remain extensible for future indicator-based trading

---

## Current Exchange Support

- Binance (Spot)
- Multi-exchange support planned

---

## Architecture

The project follows a layered architecture designed for separation of concerns, performance, and maintainability:



Controller
↓
Handler
↓
Core
├── Cache
├── Logic
└── Data



### Layer Responsibilities

- **Controller**  
  Exposes REST endpoints using Actix.

- **Handler**  
  Orchestrates runtime operations and engine state transitions.

- **Core**
  - **Cache** → In-memory evaluation engine for ultra-fast decision making  
  - **Logic** → Strategy validation and execution rules  
  - **Data** → Persistence layer (SeaORM + PostgreSQL)

---

## Tech Stack

| Layer        | Technology |
|-------------|------------|
| Backend     | Rust + Actix (REST API) |
| ORM         | SeaORM |
| Database    | PostgreSQL |
| Frontend    | React |
| Desktop     | Tauri |
| Market Feed | Binance WebSocket API |

---

## Strategy Engine

A strategy is defined as:

> An action (Buy/Sell) executed when a condition is satisfied.

### Current Strategy Model

- Condition based on:
  - Price movement
  - Percentage thresholds
- Action:
  - Buy asset
  - Sell asset

### Execution Model

1. Strategies are persisted in PostgreSQL.
2. At runtime start, strategies are loaded into memory.
3. Each WebSocket price update triggers:
   - Condition evaluation
   - If matched → Action execution

Evaluation happens **in real time per market event**, not on intervals.

### Planned Improvements

- AND / OR condition chaining
- Indicator-based conditions (RSI, EMA, MACD, etc.)
- Advanced risk management
- Trailing stops
- Strategy composition

---

## Runtime Model

The engine operates in real time:

- Connects to Binance WebSocket streams
- Receives live market data
- Triggers evaluation pipeline
- Executes trading actions immediately when matched

Designed for low latency through in-memory cache evaluation.

---

## Desktop Application

The system includes a Tauri-based desktop interface that allows:

- Full engine control (start / stop)
- Strategy creation and editing
- Runtime state visualization
- Execution logs monitoring

Targeted primarily for non-technical users while maintaining full control capabilities.

---

## Installation (Linux - Debian Based)

The application is distributed as a `.deb` package.

```bash
sudo dpkg -i trading-bots_<version>_amd64.deb
sudo apt -f install
````

After installation, the application can be launched from the system menu.

---

## Database Requirements

Currently requires:

* PostgreSQL installed locally
* Required databases created beforehand

Future roadmap includes:

* SQLite support for simplified end-user setup

---

## Binance API Requirements

Users must provide:

* Binance API Key
* Binance Secret Key

The system connects directly to Binance for:

* Market data (WebSocket)
* Order execution (REST API)

---

## Deployment Options

* Desktop usage (primary)
* Can be installed on a server and run in background mode

Docker support is not currently implemented.

---

## Roadmap

* [ ] Multi-exchange support
* [ ] SQLite support
* [ ] Strategy condition chaining (AND / OR)
* [ ] Indicator-based strategies
* [ ] Advanced risk management
* [ ] Trailing stop implementation
* [ ] Backtesting module
* [ ] Cross-platform installers (Windows / macOS)

---

## Project Goals

This project represents a deep exploration of:

* Rust backend architecture
* Real-time event-driven systems
* State-based runtime management
* Trading system design
* Desktop application distribution with Tauri

It is currently optimized for personal use but structured for future scalability.

---

## Disclaimer

This software is for educational and personal use only.

Cryptocurrency trading involves significant risk. Use at your own discretion.

---

