# chronicle

**Event-centric narrative knowledge graphs with temporal verification.**

```rust
let graph = Chronicle::from_directory("world/")?;
let report = graph.validate();

// Can this event have happened?
let proposed = Event {
    id: "new_battle".into(),
    name: "New Battle".into(),
    event_type: EventType::Battle,
    time_span: TimeSpan { start: 25, end: 25 },
    location: None,
    caused_by: vec![],
    participants: vec![
        Participant { actor: "brother_halix".into(), role: Role::Attacker, .. },
    ],
    state_changes: vec![],
    description: String::new(),
};
graph.can_add_event(&proposed)?;
// Err: actor 'brother_halix' has status Captured
//      as of 'battle_of_broken_glass' (year 16),
//      cannot participate in proposed event 'new_battle' (year 25)
```

## The problem

World-building rots. You write 45 lore documents for a game — histories, faction profiles, character backstories, location guides — and they start contradicting each other. A character dies in one document and shows up alive in another. A city is destroyed in year 20 but hosts a festival in year 25. An AI agent generates a plausible-sounding lore entry that quietly breaks the timeline.

At small scale you hold the world in your head. At 40+ documents and growing, you can't. The inconsistencies compound faster than you catch them.

## The approach

Replace disconnected prose with a typed knowledge graph where every relationship passes through an event. This is borrowed from how historians actually model real history — specifically the event-centric principle from CIDOC CRM (ISO 21127), the international standard for cultural heritage documentation.

The insight: don't link a character directly to a location. Link them through an event that happened at that location, at a specific time, with specific participants who had specific roles and feelings about it. The event is the connective tissue.

```
[Commander Kaine] --participated_in(Attacker, Dutiful)--> [Siege of Silica]
[Siege of Silica] --occurred_at--> [Silica]
[Siege of Silica] --caused_by--> [Purist Uprising]
[Siege of Silica] --state_change--> [Silica: Destroyed]
```

Every entity has a stable ID. Narrative prose uses `{entity_id}` references instead of raw names, so every mention is a resolvable, queryable graph reference. An account written by a biased narrator is tagged with its fidelity — the same event looks different depending on who's telling it.

## The interesting part: verification

The graph isn't just storage — it's a constraint system. When you add a new record, chronicle checks whether it's temporally and logically possible given everything else in the graph.

Temporal consistency uses Allen's Interval Algebra — 13 possible relationships between time intervals (before, after, during, overlaps, meets, etc.). If an actor dies in year 16, they can't participate in an event in year 25. If Event A caused Event B, A must precede B. If a city was destroyed, future events there need to acknowledge that.

This is the inversion that makes it interesting: CIDOC CRM was designed for historians analyzing what *did* happen. Chronicle uses the same formal model to check what *can* happen. Same math, different direction — descriptive becomes prescriptive.

## Objective vs. subjective graphs

Not all sources are reliable. Chronicle models this explicitly:

- **Canonical** sources (archive-drones with intact records) match the objective graph exactly
- **Partial** sources (eyewitnesses) have omissions but no fabrication
- **Biased** sources (faction propagandists) have deliberate spin
- **Corrupted** sources (storm-damaged records) have degraded data

Two NPCs tell conflicting stories about the same war. That's not a bug — it's two subjective fragments with different fidelity ratings, both queryable against the objective ground truth. The player learns to cross-reference. Finding a high-fidelity source (an archive-drone) becomes mechanically meaningful.

## Typed queries, not magic

```rust
graph.actor("kaine_durgan").interactions().people()
graph.event("siege_of_silica").causal_chain()
graph.place("silica").events_during(15..25)
graph.actor("brother_halix").status_at(20)  // Captured
```

The query API is deliberately simple because the event metadata does the heavy lifting. Rich, well-typed event records make traversal straightforward — no natural language parsing, no fuzzy matching, just method chains over structured data.

## What it's built on

- **petgraph** — graph structure and traversal algorithms (StableGraph with enum node/edge types)
- **allen-intervals** — all 13 Allen temporal relations, discrete integer time domain
- **serde + ron** — hand-authorable structured content format
- **thiserror** — error types

No async, no database, no network, no GPU. ~1,900 lines of Rust.

## Where it's going

Chronicle is being built for [saltglass-steppe](/content/saltglass-steppe), a deterministic TUI roguelike with ~45 lore documents that are already showing consistency drift. The near-term goal is validation and querying. The longer-term idea: procedurally generate the *skeleton* of a narrative graph (events, participants, causal chains, temporal ordering, sentiment tags) and use chronicle's constraint checker to guarantee consistency by construction. The graph becomes the outline; prose generation becomes a constrained fill-in task.

**Links:** [GitHub](https://github.com/EliasVahlberg/chronicle) · [crates.io](https://crates.io/crates/chronicle-graph) · [docs.rs](https://docs.rs/chronicle-graph)
