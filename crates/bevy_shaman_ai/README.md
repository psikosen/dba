# Shaman's Journey - LLM AI System

This crate provides dynamic, LLM-driven AI for bosses and NPCs using the Gemma3:270M model.

## Features

- **Boss Combat AI**: Dynamic combat decisions based on personality, health, and battle state
- **Boss Dialogue**: Context-aware taunts, phase transitions, and battle cries
- **Brother NPC Dialogue**: Natural conversations with player's brothers
- **Query Queue System**: Pre-generates responses for smooth gameplay
- **Conversation History**: Tracks context for coherent multi-turn dialogue
- **Personality System**: Each AI has unique traits affecting behavior

## Architecture

### Components

- `LlmAi`: Core component marking entities with LLM-driven behavior
- `LlmQueryQueue`: Pre-generated responses and combat decisions
- `ConversationHistory`: Tracks dialogue for contextual responses
- `LlmGenerationRequest`: Requests new AI generation

### Resources

- `LlmModel`: Manages the GGUF model file and configuration
- `PromptTemplates`: Templates for different scenarios (boss, brother, etc.)
- `ResponseCache`: Caches common responses for performance

### Systems

**Query Queue** (`systems/query_queue.rs`):
- `monitor_query_queues`: Detects when queues need refilling
- `process_generation_requests`: Generates new responses (currently using placeholders)

**Boss AI** (`systems/boss_ai.rs`):
- `boss_combat_ai`: Executes LLM-driven combat decisions
- `boss_combat_dialogue`: Generates battle dialogue
- `boss_phase_transitions`: Handles boss phase changes

**NPC Dialogue** (`systems/npc_dialogue.rs`):
- `brother_dialogue_system`: Handles player-NPC conversations
- `brother_advice_system`: Provides contextual gameplay advice
- `dynamic_greeting_system`: Generates initial greetings

## Adding the GGUF Model (TODO)

**Steps to integrate the actual LLM:**

1. **Download Gemma3:270M GGUF model**:
   ```bash
   # Download from Hugging Face or Google AI
   # Place in: assets/models/gemma3_270m.gguf
   ```

2. **Add llama.cpp Rust bindings**:
   ```toml
   # Add to Cargo.toml:
   [dependencies]
   llama-cpp-2 = "0.1"  # Or appropriate version
   ```

3. **Update `resources.rs`**:
   - Implement actual model loading in `LlmModel::default()`
   - Add model instance field using llama-cpp types

4. **Update `systems/query_queue.rs`**:
   - Replace `generate_placeholder_dialogue()` with actual LLM inference
   - Replace `generate_placeholder_combat_decision()` with LLM JSON parsing

5. **Inference pseudocode**:
   ```rust
   // In process_generation_requests:
   let model = llama_cpp::Model::from_file(&model.model_path)?;
   let session = model.create_session(model.config.context_size)?;

   let prompt = templates.fill_boss_dialogue_prompt(&data);
   let response = session.generate(
       &prompt,
       model.config.max_tokens,
       model.config.temperature,
       model.config.top_p,
   )?;

   // Parse response and add to queue
   ```

## Usage Examples

### Spawn a Boss with LLM AI

```rust
use bevy_shaman_ai::*;

fn spawn_boss_example(mut commands: Commands) {
    let boss = spawn_boss_with_ai(
        &mut commands,
        "Corrupted Chieftain".to_string(),
        components::PersonalityTraits {
            aggression: 0.9,
            wisdom: 0.3,
            chattiness: 0.6,
            honor: 0.2,
            spirituality: 0.1,
        },
    );

    // Add boss-specific components
    commands.entity(boss).insert((
        Health::new(500.0),
        MonsterState::default(),
        // ... other components
    ));
}
```

### Spawn a Brother NPC

```rust
fn spawn_brother_example(mut commands: Commands) {
    let brother = spawn_brother_with_ai(
        &mut commands,
        "Kofi".to_string(),
        components::PersonalityTraits {
            aggression: 0.2,
            wisdom: 0.8,
            chattiness: 0.7,
            honor: 0.9,
            spirituality: 0.9,
        },
    );
}
```

### Request Dialogue

```rust
fn player_talk_to_npc(
    mut dialogue_events: EventWriter<PlayerDialogueRequest>,
    player_query: Query<Entity, With<Player>>,
    brother_query: Query<Entity, With<LlmAi>>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Some(brother) = brother_query.iter().next() {
            dialogue_events.send(PlayerDialogueRequest {
                player,
                npc: brother,
                player_message: Some("How do I defeat the corrupted spirits?".to_string()),
            });
        }
    }
}
```

## Personality Traits

Each AI has 5 core traits (0.0 to 1.0):

- **Aggression**: Affects combat style and dialogue tone
- **Wisdom**: Influences strategic decisions and advice quality
- **Chattiness**: Controls dialogue frequency and length
- **Honor**: Affects trustworthiness and fighting style
- **Spirituality**: Connection to spirit world themes

## Combat Stances

Bosses use different combat stances that affect their behavior:

- `Aggressive`: High damage, rushdown tactics
- `Defensive`: Block and counter
- `Tactical`: Use environment and special moves
- `Evasive`: Dodge-focused
- `Summoner`: Calls minions
- `Corrupting`: Spreads corruption

## Emotional States

AI emotional state changes based on combat situation:

- `Calm`: Default, balanced behavior
- `Confident`: High health, aggressive
- `Angry`: Taking damage, more aggressive
- `Desperate`: Low health, unpredictable
- `Mocking`: Player is weak
- `Fearful`: Player is dominant
- `Corrupted`: High corruption meter

## Prompt Templates

The system includes several prompt templates:

1. **Boss Combat Template**: Generates combat decisions with JSON output
2. **Boss Dialogue Template**: Generates contextual battle dialogue
3. **Brother Dialogue Template**: Generates supportive, wise dialogue
4. **Spirit Guide Template**: Generates mystical, cryptic guidance

Each template fills in personality traits, current state, and context for dynamic generation.

## Performance Notes

- Query queues pre-generate 5-10 responses to minimize runtime generation
- Response cache stores common responses (max 100 entries)
- Generation limited to 2 concurrent requests
- Boss AIs have critical priority, brothers have normal priority

## Future Enhancements

- [ ] Integrate actual GGUF model inference
- [ ] Add more prompt templates (merchants, spirit guides)
- [ ] Implement dialogue branching based on player choices
- [ ] Add voice line generation hooks
- [ ] Create boss-specific combat patterns
- [ ] Add memory system for long-term relationship tracking
- [ ] Implement faction-based dialogue variations
