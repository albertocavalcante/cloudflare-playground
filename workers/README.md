# Cloudflare Workers Samples

This repository contains sample code demonstrating different implementations of Cloudflare Workers.

## Projects

### Weather Client (Rust)

A Rust-based Cloudflare Worker that acts as a weather client, fetching weather data from an external API.
- Located in: [weather-client-rust](weather-client-rust)
- Features:
  - Built with Rust and WebAssembly
  - Uses the Worker runtime
  - Implements a REST API endpoint
  - Fetches weather data from goweather.herokuapp.com

## Getting Started

Each project contains its own README and setup instructions. To run any of these samples, you'll need:

1. A Cloudflare account
2. Wrangler CLI (accessed via `npx wrangler`)
3. Project-specific dependencies (see individual project READMEs)

## Note

These are example implementations intended for learning and reference purposes. Feel free to use them as starting points for your own Cloudflare Workers projects.
