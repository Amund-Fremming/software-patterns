# State Machines in Rust

Two patterns, different guarantees.

---

## Struct typestate (`struct_variant.rs`)

State is encoded in the type parameter. Transitions are methods that consume `self` and return a new type.

```rust
struct Order<S> { id: u128, state: S }

impl Order<Pending> {
    fn confirmed(self) -> Order<Confirmed> { ... }
}
```

Calling `.ship()` on `Order<Pending>` is a **compile error** — the method doesn't exist on that type.

**Strengths:**
- Wrong-state calls are impossible, not just handled
- No runtime overhead — zero-cost abstraction
- Memory-efficient in Rust: the old value is moved, not copied; no GC pressure (unlike Java/Go where each transition allocates)
- Self-documenting function signatures: `fn start(app: App<Configured>)` tells you everything

**When to use:**
- Linear, owned pipelines: app startup, builders, protocol handshakes you drive yourself
- When a wrong-state call is a **programming error**, not a business case
- Short-lived values with one owner from start to finish

**Hard limit:** once you need to store the value in a collection or transition it from an external event, you lose the static type — you need a wrapper enum, at which point you've recreated the enum pattern with extra steps.

---

## Enum state machine (`enum_variant.rs`)

State is a value, not a type. One `handle` method matches on current state + event and returns the next state or an error.

```rust
fn handle(&mut self, event: OrderEvent) -> Result<(), String> {
    let next = match (&self.state, event) {
        (Pending, Confirm) => Confirmed,
        (Confirmed, Ship)  => Shipped,
        (state, event)     => return Err(...),
    };
    self.state = next;
    Ok(())
}
```

**Strengths:**
- Heterogeneous collections: `HashMap<u128, Order>` just works
- Event-driven: external events arrive and transition arbitrary stored items
- Invalid transitions are runtime errors you explicitly handle — correct for business cases that can legitimately occur
- Exhaustive `match` forces you to handle every state/event combination

**When to use:**
- State lives in a cache, DB, or any shared store
- Transitions are driven by external events (queues, HTTP, websockets)
- Multiple owners or threads can interact with the same item
- Wrong-state is a **runtime condition**, not a bug

---

## The core tradeoff

| | Struct typestate | Enum |
|---|---|---|
| Error catching | Compile time | Runtime |
| Storage | Single owner only | Any collection |
| External events | Awkward | Natural |
| Rust-specific advantage | Move semantics = zero cost | Exhaustive match = forced handling |

The struct pattern's value is that there **is no wrapper** — the type in the signature is the guarantee. The moment you add a wrapper enum for storage, you've rebuilt the enum pattern. Don't fight it; use the enum.

See `real_world_example.rs` for what the hybrid looks like and why it doesn't pay off over a plain enum.
