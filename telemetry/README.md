# Telemetry

This module provides a developer-friendly yet performance-conscious interface for observability. It is designed to serve as a monitoring layer for both development and production environments, *with a deliberate emphasis on signal quality over logging volume*.

## Key Features

**Scope Recorders** are the central abstraction of this module. They follow a RAII pattern, automatically managing lifecycle and enabling structured context propagation through parent-child relationships between recorders.

**Lightweight Logs** reflect a core design philosophy: minimizing the observability overhead on hot paths. This module deliberately trades flexibility for focus — fewer logs, but logs that carry meaningful, actionable information.

## Motivation

> ***"Why build a telemetry framework when `tracing` and `opentelemetry` already exist?"***

Fair question. The short answer is: this is built for a high-frequency trading system, and general-purpose solutions come with trade-offs that are unacceptable in that context.

**No compromise on performance.** Hot paths cannot be burdened by logging overhead, yet they cannot run entirely dark. This module is designed to make monitoring viable where it would otherwise be too costly.

**Total control over optimization.** While the current implementation is not yet fully optimized, owning the framework means we can tune it precisely to our needs — no waiting on upstream priorities or working around abstractions we don't control.

**Opinionated by design.** Classical logging frameworks offer broad flexibility, which often leads to noisy, low-quality logs. By constraining what developers can do, this module encourages deliberate thinking about what is actually worth observing — and that constraint tends to produce significantly better signal.

## Design Overview

<!-- TODO: Describe the architecture of the module.
     Suggested points to cover:
     - How scope recorders are structured internally
     - How parent-child relationships are established and propagated
     - The lifecycle of a recorder (creation, logging, drop)
     - How log entries are collected and flushed (e.g. sync vs async, buffering strategy)
     - Any relevant data structures (ring buffers, arenas, etc.)
     A simple diagram of the recorder hierarchy would be a valuable addition here. -->

## Usage

### Creating a Scope Recorder

<!-- TODO: Show how to instantiate a root-level recorder. -->
```rust
// TODO
```

### Parent-Child Recorders

<!-- TODO: Demonstrate how a child recorder inherits or propagates context from a parent. -->
```rust
// TODO
```

### Emitting Logs

<!-- TODO: Show the logging API — what can be logged, with what severity levels or metadata. -->
```rust
// TODO
```

## Configuration

<!-- TODO: Document available configuration options.
     Suggested points to cover:
     - Log levels or verbosity controls
     - Output targets (stdout, file, network sink, etc.)
     - Compile-time feature flags (e.g. stripping logs from release builds)
     - Any environment variables or config file support -->

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| TODO   | TODO | TODO    | TODO        |

## Performance

<!-- TODO: Characterize the performance profile of this module.
     Suggested points to cover:
     - Measured overhead per log event (ns/op or cycles)
     - Memory footprint (allocation strategy, buffer sizes)
     - Key implementation choices that enable low overhead (e.g. lock-free structures, pre-allocation, compile-time elision)
     - Benchmark setup and how to reproduce results -->

| Benchmark | Result |
|-----------|--------|
| TODO      | TODO   |

## Limitations & Trade-offs

<!-- TODO: Be explicit about what this module does not do, and where the deliberate constraints lie.
     Suggested points to cover:
     - No support for X (structured fields, distributed tracing, etc.)
     - Reduced flexibility compared to tracing/opentelemetry and why that is intentional
     - Any thread-safety caveats
     - Known edge cases or failure modes -->

## Potential Improvements

<!-- TODO: Track ideas and planned work here, or link to your issue tracker.
     Suggested points to cover:
     - Optimisations identified but not yet implemented
     - Features that could be added without compromising the design philosophy
     - Open design questions -->

- [ ] TODO