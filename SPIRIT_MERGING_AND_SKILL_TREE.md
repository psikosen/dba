# Spirit Merging/Weapon Enhancement & Skill Tree Systems

## Overview

This implementation adds two major systems to the Shaman's Journey game:
1. **Spirit Merging/Weapon Enhancement System**
2. **Skill Tree System** (Graph-based, African lore-themed)

## Spirit Merging/Weapon Enhancement System

### Features

#### Permanent Enhancement
- **Levels**: 1-10 enhancement levels
- **Cost Structure**: Exponentially increasing costs
  - Spirit orbs (energy requirement)
  - Blood plants (sacrifice requirement)
  - Spirit plants (mystical requirement)
- **Bonuses per Level**:
  - +5% base damage (+ 0.5% per level)
  - +2% spirit efficiency
  - Durability restoration and max increase (+10)
  - Special abilities at levels 3, 6, 9

#### Special Abilities Unlocked
- **Level 3**: +10% critical chance
- **Level 6**: +5% lifesteal
- **Level 9**: Ancestral Strike (ultimate ability)

#### Temporary Enchantments
Four types of plant-based enchantments:

1. **Blood Fury** (from blood plants like Ropa, Ingazi)
   - Temporary damage boost
   - Duration based on plant type

2. **Spirit Infusion** (from spirit plants like Roho, Elima)
   - Reduces spirit cost of abilities
   - Enhanced magical efficiency

3. **Venom Coating** (from poison plants)
   - Adds poison damage to attacks
   - Synergizes with Mambele weapons

4. **Ancestral Blessing** (from legendary plants)
   - +15% damage boost
   - Connection to ancestral spirits

### Controls
- **H**: Toggle enhancement UI
- **M**: Merge spirit orbs into weapon
- **N**: Apply plant enchantments

### File Structure
- `crates/bevy_shaman_combat/src/components.rs`: Added `WeaponEnhancement`, `Enchantment`, `EnchantmentType`
- `crates/bevy_shaman_combat/src/systems/enhancement.rs`: Enhancement logic
- `crates/bevy_shaman_ui/src/enhancement_ui.rs`: UI for weapon enhancement

---

## Skill Tree System

### Design Philosophy
Graph-based skill tree with prerequisites, inspired by African concepts and languages.

### Five Skill Paths

#### 1. Ngoma (Swahili: Drum) - Rhythm & Combos
*Theme: Master the rhythm of battle*

- **Tier 1**: Rhythm Sense - +10% rhythm timing window
- **Tier 2**: Combo Master - +2 max combo length
- **Tier 3**: Perfect Harmony - Perfect hits grant +20% damage
- **Tier 4**: Drum of War - Combos reduce cooldowns by 15%
- **Tier 5**: Ancestral Rhythm - Unlock special combo finishers

#### 2. Ubuntu (Nguni Bantu: Community/Humanity) - Party & Community
*Theme: Together we are stronger*

- **Tier 1**: Shared Strength - +10% damage when near allies
- **Tier 2**: Healing Circle - HP regen affects all nearby allies
- **Tier 3**: Spirit Link - Share 25% of spirit regeneration
- **Tier 4**: War Chant - Party members gain +15% attack speed
- **Tier 5**: Ancestral Bond - Party buffs last 50% longer

#### 3. Ashe (Yoruba: Power/Authority) - Stats & Power
*Theme: Channel your life force*

- **Tier 1**: Inner Power - +15% max health
- **Tier 2**: Spirit Reservoir - +20% max spirit
- **Tier 3**: Power Strike - +20% base damage
- **Tier 4**: Spirit Fortitude - +10% damage resistance
- **Tier 5**: Ancestral Might - +30% damage, +30% spirit regen

#### 4. Ubiqa (Xhosa: To Bloom/Flourish) - Plants & Nature
*Theme: Understand the spirits of plants*

- **Tier 1**: Green Thumb - Plant effects last 20% longer
- **Tier 2**: Blood Knowledge - Blood sacrifice cost reduced by 25%
- **Tier 3**: Spirit Gardener - Spirit plant effects +30% stronger
- **Tier 4**: Harvest Boon - 20% chance for double plant harvest
- **Tier 5**: Nature's Boon - Passive plants regenerate resources 50% faster

#### 5. Tempo - Speed & Cooldowns
*Theme: Move like the wind*

- **Tier 1**: Swiftness - +15% movement speed
- **Tier 2**: Quick Recovery - -10% all cooldowns
- **Tier 3**: Haste - +20% attack speed
- **Tier 4**: Rapid Fire - -25% ability cooldowns
- **Tier 5**: Timeless - +40% attack speed, -30% cooldowns

### Skill Tree Mechanics

#### Prerequisites System
- Each skill can have multiple prerequisites
- Graph structure ensures logical progression
- Prevents jumping to high-tier skills without foundation

#### Skill Points
- Earned through leveling, quests, achievements
- Cost increases with tier (Tier 1 = 1 point, Tier 5 = 5 points)
- Total points tracked for statistics

#### UI Features
- **Path tabs**: Quick navigation between skill paths
- **Color-coded nodes**:
  - Gold: Unlocked
  - Green: Available to unlock
  - Grey: Locked (missing prerequisites or points)
- **Detailed tooltips**: Name, description, cost, prerequisites

### Controls
- **K**: Toggle skill tree UI
- **Click on skills**: Unlock with skill points

### File Structure
- `crates/bevy_shaman_combat/src/systems/skill_tree.rs`: Skill tree logic and database
- `crates/bevy_shaman_ui/src/skill_tree_ui.rs`: UI for skill tree

---

## UI Design - African Aesthetic

Both UIs follow the "Ancestral Legacy" theme established in the game:

### Visual Elements
- **Carved wooden panels**: Mahogany and ebony backgrounds
- **Bronze frames**: Hammered bronze borders with patina
- **Gold accents**: For titles and important information
- **Earth tones**: Ochre, terracotta, charcoal
- **Natural dyes**: Indigo, forest green, turmeric yellow
- **Bone/ivory**: For text and highlights

### Color Meanings
- **Gold/Bronze**: Power, prestige, unlocked abilities
- **Red Ochre**: Blood, damage, sacrifice
- **Indigo**: Spirit, mystical power
- **Forest Green**: Nature, plants, availability
- **Terracotta**: Earth, grounding, passives

---

## Integration with Existing Systems

### Plants
- Blood plants (Mogodu, Ropa, Ingazi) for weapon enhancement
- Spirit plants (Roho, Elima, Nommo) for spiritual infusions
- Poison plants (VenomVine, ShadowMushroom) for weapon coatings

### Combat
- Enhancement bonuses apply to all attacks
- Skill tree bonuses modify core stats
- Combo system enhanced through Ngoma path
- Party mechanics enhanced through Ubuntu path

### Resources
- Spirit orbs can be infused into weapons
- Blood and spirit costs for enhancement
- Skill points as progression currency

---

## Technical Implementation

### Components
- `WeaponEnhancement`: Tracks enhancement level and bonuses
- `SkillTree`: Tracks unlocked skills and available points

### Resources
- `SkillDatabase`: Centralized skill data with 25 total skills
- `EnhancementUIState`: UI visibility state
- `SkillTreeUIState`: UI visibility and selected path

### Systems
- `spirit_merging_system`: Handles permanent upgrades
- `plant_enhancement_system`: Applies temporary enchantments
- `enchantment_decay_system`: Updates active enchantments
- `apply_enhancement_bonuses`: Applies bonuses to attacks
- `skill_unlock_system`: Handles skill unlocking
- `apply_skill_bonuses`: Applies passive skill effects

---

## Future Enhancements

### Possible Additions
1. **Weapon Transmutation**: Change weapon types while preserving enhancements
2. **Ancestral Spirits**: Summon ancestors at max enhancement
3. **Combo Trees**: Unlock custom combo patterns through skill tree
4. **Prestige System**: Reset skills for permanent bonuses
5. **Cross-Path Synergies**: Bonuses for unlocking skills across multiple paths
6. **Plant Fusion**: Combine multiple plants for unique enchantments

### UI Improvements
1. **Visual skill tree graph**: Show connections between skills
2. **Animation effects**: Pulse effects for available skills
3. **Sound effects**: African drums for unlocks, traditional instruments
4. **Tooltip enhancements**: Preview skill effects before unlock
5. **Build templates**: Save and load skill builds
6. **Respec system**: Allow skill point reallocation

---

## African Cultural Elements

### Language Integration
- **Ngoma** (Swahili): Drum - Represents rhythm and heartbeat
- **Ubuntu** (Nguni Bantu): Humanity - "I am because we are"
- **Ashe** (Yoruba): Power to make things happen
- **Ubiqa** (Xhosa): To bloom/flourish - Growth and nature
- **Tempo**: Universal concept integrated into African rhythm tradition

### Cultural Concepts
- **Community over individual**: Ubuntu path emphasizes party play
- **Ancestral connection**: Multiple references to ancestral power
- **Nature harmony**: Ubiqa path reflects relationship with land
- **Rhythm as life**: Ngoma path honors drum/dance culture
- **Power with responsibility**: Ashe path reflects balanced power

### Visual Themes
- Fractalarithmetic patterns inspired by African architecture
- Earthtones and natural materials
- Handcrafted, organic aesthetic (not digital/synthetic)
- Symbolic use of bronze, gold, and ivory
- References to traditional crafts (carving, weaving, metalwork)

---

## Testing Notes

**Build Status**: Code implementation complete. Build fails due to missing system libraries (libudev, alsa) in environment - not related to the new code.

**Manual Testing Required**:
1. Test weapon enhancement UI (Press H)
2. Test skill tree UI (Press K)
3. Verify skill unlocking with prerequisites
4. Test temporary enchantments decay
5. Verify passive skill bonuses apply correctly
6. Test party bonuses (Ubuntu path)
7. Test plant-based enhancement materials

---

## Credits

Design inspired by:
- African linguistic traditions and philosophy
- Traditional African crafts and aesthetics
- Bevy ECS architecture patterns
- Existing game systems (plants, combat, spirits)
