# Compilation Fixes Needed

The AI crate has a few borrowing issues that need to be fixed:

## boss_ai.rs

### Fix 1: boss_combat_dialogue() - Line ~146
Change from:
```rust
if let Some(response) = queue.dialogue_responses.iter().find(...)
{
    history.add_npc_message(response.text.clone(), ...);
    queue.dialogue_responses.retain(|r| !std::ptr::eq(r, response));
}
```

To:
```rust
if let Some(idx) = queue.dialogue_responses.iter().position(...)
{
    let response = queue.dialogue_responses.remove(idx);
    history.add_npc_message(response.text.clone(), ...);
}
```

### Fix 2: boss_phase_transitions() - Line ~176
Same pattern - use `position()` to get index, then `remove(idx)`

## npc_dialogue.rs

### Fix 1: Line 166
Change:
```rust
for (entity, ai, mut queue) in brother_query.iter() {
```

To:
```rust
for (_entity, ai, mut queue) in brother_query.iter_mut() {
```

### Fix 2: brother_dialogue_system() - Line ~52
Same pattern as boss_ai.rs - use `position()` and `remove()`
