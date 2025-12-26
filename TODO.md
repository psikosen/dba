# Shaman's Journey - TODO List

## 🎯 Current Project Status

### ✅ Completed (100%)
- [x] Core gameplay loop (movement, camera, grid system)
- [x] Rhythm-based combat system
- [x] Monster state machine (8 states)
- [x] Minion taming and control
- [x] Procedural dungeon generation
- [x] Boss encounter system
- [x] Item pickup and inventory backend
- [x] Shop system with currency
- [x] Story progression tracking
- [x] NPC sickness state system
- [x] **CrossCode-style dialogue UI** ⭐ NEW
- [x] **Character portrait system** ⭐ NEW
- [x] **African names database (30+ names)** ⭐ NEW
- [x] **Minimap with fog of war** ⭐ NEW
- [x] Main menu and loading screen
- [x] Audio/rhythm beat clock

---

## 🚧 Partially Complete (Needs Work)

### ⚠️ Save/Load System (30% Complete)
**Current Status:**
- ✅ Saving works
- ❌ Loading doesn't restore entities properly
- ❌ NPCs/monsters not respawning on load

**What's Needed:**
1. Fix entity restoration on load
2. Implement proper serialization for all entity types
3. Test save/load cycle for each game state
4. Add save file versioning
5. Implement autosave system

**Priority:** HIGH - Critical for player experience

**Estimated Effort:** 2-3 days

---

### ⚠️ Inventory UI (0% Complete)
**Current Status:**
- ✅ Backend inventory system works
- ✅ Items can be picked up and stored
- ❌ No visual UI for inventory display
- ❌ No drag-and-drop

**What's Needed:**
1. Create inventory grid UI (similar to shop UI style)
2. Add item icons/sprites
3. Implement drag-and-drop for item management
4. Add item tooltips with stats/descriptions
5. Add hotkey support (number keys for quick slots)
6. Show equipped items vs stored items
7. Add item sorting/filtering

**Priority:** MEDIUM - Backend works, just needs UI

**Estimated Effort:** 2-3 days

---

### ⚠️ Sprite Swapping System (10% Complete)
**Current Status:**
- ✅ Monster sprite database exists
- ✅ Sprite swap system code exists
- ❌ System doesn't actually swap sprites
- ❌ Database lookups return None

**What's Needed:**
1. Fix sprite database loading
2. Implement actual sprite swapping on state change
3. Create sprite assets for each monster state
4. Test state transitions trigger visual changes
5. Add smooth transitions/animations between sprites

**Priority:** MEDIUM - Visual feedback important

**Estimated Effort:** 1-2 days (mostly art assets)

---

### ⚠️ Dialogue UI Portraits (80% Complete)
**Current Status:**
- ✅ UI system complete and functional
- ✅ Portrait database implemented
- ✅ African names integrated
- ❌ No actual portrait image assets
- ❌ Placeholder needed

**What's Needed:**
1. Create placeholder portrait (512x512 PNG)
2. Create portrait art for main characters
3. Create portrait art for bosses (9 characters)
4. Create portrait art for NPCs (30+ characters)
5. Create emotional variants (6-8 per character)
6. Load portraits into database at startup

**Priority:** LOW - System works, just needs art

**Estimated Effort:** Art-dependent (weeks if hand-drawn)

---

## ❌ Not Started (0% Complete)

### ❌ LLM Integration (0%)
**Description:**
Replace hardcoded dialogue with gemma3:270m for dynamic NPC conversations

**What's Needed:**
1. Set up gemma3:270m model integration
2. Create prompt templates for NPCs
3. Implement context-aware dialogue generation
4. Add personality traits for different NPC types
5. Integrate with existing dialogue UI
6. Add caching to avoid regenerating same responses
7. Fallback to hardcoded dialogue if LLM unavailable

**Technical Requirements:**
- Gemma3 270M parameter model
- Local inference or API integration
- Context management (NPC personality, game state, player actions)
- Response streaming for long dialogue

**Priority:** LOW - Nice to have, not critical

**Estimated Effort:** 3-5 days

---

### ❌ Typewriter Text Effect (0%)
**Description:**
Add character-by-character text reveal animation to dialogue

**What's Needed:**
1. Create typewriter component with timer
2. Reveal text character by character
3. Add configurable speed
4. Add skip functionality (click to complete)
5. Add sound effect per character (optional)
6. Support for pause markers in text

**Priority:** LOW - Polish feature

**Estimated Effort:** 1 day

---

### ❌ Multiple Choice Dialogue (0%)
**Description:**
Allow players to make choices in conversations

**What's Needed:**
1. Create dialogue choice component
2. Add button UI for choices (2-4 options)
3. Implement choice branching logic
4. Store dialogue history/choices made
5. Link choices to quest progression
6. Add visual indication of choice consequences

**Priority:** MEDIUM - Adds depth to story

**Estimated Effort:** 2-3 days

---

### ❌ Quest UI (0%)
**Description:**
Visual display of active and completed quests

**What's Needed:**
1. Quest log UI (toggle with hotkey)
2. Active quest tracking (objectives)
3. Quest markers on minimap
4. Quest completion notifications
5. Quest reward display
6. Quest history/journal

**Priority:** MEDIUM - Helps player track progress

**Estimated Effort:** 2-3 days

---

### ❌ Enhanced Minimap Features (0%)
**Description:**
Add icons, markers, and navigation features

**What's Needed:**
1. Quest markers on minimap
2. NPC location markers
3. Point of interest icons (shops, bosses, dungeons)
4. Waypoint system (set custom markers)
5. Minimap zoom levels
6. Compass direction indicator
7. Minimap panning/dragging

**Priority:** LOW - Current minimap functional

**Estimated Effort:** 2 days

---

### ❌ Spirit World Visual Effects (0%)
**Description:**
Distinct visual style for spirit realms

**What's Needed:**
1. Color grading/filters per spirit world
2. Particle effects for corruption
3. Portal visual effects
4. Transition animations between worlds
5. Ambient lighting changes
6. Background parallax layers

**Priority:** MEDIUM - Helps distinguish areas

**Estimated Effort:** 3-4 days

---

### ❌ Monster Taming Improvements (0%)
**Description:**
Better feedback and UI for taming system

**What's Needed:**
1. Taming progress bar
2. Visual indicator when monster is tameable
3. Taming mini-game or rhythm challenge
4. Success/failure animations
5. Monster loyalty system
6. Rename tamed monsters
7. Release monster functionality

**Priority:** MEDIUM - Core feature needs polish

**Estimated Effort:** 2-3 days

---

### ❌ Combat Visual Feedback (0%)
**Description:**
Better visual/audio feedback for combat

**What's Needed:**
1. Damage numbers floating text
2. Hit effects (screen shake, flash)
3. Critical hit animations
4. Dodge/miss indicators
5. Combo counter display
6. Perfect timing visual effects
7. Health bar animations

**Priority:** MEDIUM - Combat feels better with feedback

**Estimated Effort:** 2-3 days

---

### ❌ Audio System Expansion (0%)
**Description:**
Add more audio features beyond rhythm system

**What's Needed:**
1. Background music per area
2. Combat music with dynamic intensity
3. Sound effects for actions (footsteps, attacks, UI)
4. NPC voice clips (optional)
5. Ambient environmental sounds
6. Audio settings (volume controls)
7. Music cross-fading between areas

**Priority:** LOW - Game functional without

**Estimated Effort:** 3-4 days (+ audio asset creation)

---

### ❌ Performance Optimization (0%)
**Description:**
Optimize for smooth gameplay

**What's Needed:**
1. Profile systems for bottlenecks
2. Spatial partitioning for entity queries
3. Sprite batching
4. Level of detail (LOD) for distant entities
5. Lazy loading for dungeon chunks
6. Memory pooling for frequent spawns
7. FPS counter and debug overlay

**Priority:** LOW - Optimize when needed

**Estimated Effort:** 2-3 days

---

### ❌ Game Balance & Tuning (0%)
**Description:**
Balance gameplay values

**What's Needed:**
1. Monster health/damage tuning
2. Rhythm timing window adjustments
3. Item price balancing
4. Experience/progression curve
5. Boss difficulty scaling
6. Stamina consumption rates
7. Corruption spread rates

**Priority:** MEDIUM - Needs playtesting data

**Estimated Effort:** Ongoing

---

### ❌ Tutorial System (0%)
**Description:**
Teach players the game mechanics

**What's Needed:**
1. Tutorial quest/dialogue
2. On-screen tooltips for first-time actions
3. Rhythm combat tutorial
4. Taming tutorial
5. UI element highlights
6. Tutorial skip option
7. Tips during loading screens

**Priority:** HIGH - Critical for new players

**Estimated Effort:** 3-4 days

---

## 🎨 Art Asset Needs

### Required Assets
- [ ] Character portraits (40+ characters × 6-8 emotions each)
- [ ] Monster state sprites (sprite sheet for each state)
- [ ] Environment tilesets (villages, forests, mountains, spirit realms)
- [ ] Item icons (consumables, equipment, quest items)
- [ ] UI elements (buttons, frames, borders)
- [ ] Effect sprites (attacks, magic, corruption)
- [ ] Boss sprites (9 bosses × multiple states)

### Style Guide
- Pixel art aesthetic
- 32x32 for game sprites
- 512x512 for portraits
- Consistent color palette
- Cutout style for portraits (transparent background)

---

## 🎵 Audio Asset Needs

### Required Audio
- [ ] Background music tracks (overworld, villages, dungeons, boss fights)
- [ ] Rhythm tracks for combat
- [ ] Sound effects (attacks, UI, pickups, footsteps)
- [ ] NPC voice clips (optional)
- [ ] Ambient sounds (wind, water, corruption)

---

## 📋 Priority Roadmap

### Phase 1: Critical Features (Make game fully playable)
1. **Fix Save/Load System** - Players need to save progress
2. **Build Inventory UI** - Players need to see/manage items
3. **Add Tutorial System** - Players need to learn mechanics
4. **Create placeholder portrait** - Stop placeholder asset errors

**Estimated Time:** 1-2 weeks

### Phase 2: Polish & Feedback (Make game feel good)
1. **Fix Sprite Swapping** - Visual state feedback
2. **Add Combat Visual Feedback** - Damage numbers, effects
3. **Implement Monster Taming Improvements** - Better UX
4. **Add Multiple Choice Dialogue** - Story depth

**Estimated Time:** 1-2 weeks

### Phase 3: Content & Depth (Make game interesting)
1. **Create Quest UI** - Track player objectives
2. **Add Spirit World Visual Effects** - Area distinction
3. **Implement Typewriter Text** - Professional dialogue feel
4. **Balance & Tuning** - Playtest and adjust

**Estimated Time:** 2-3 weeks

### Phase 4: Advanced Features (Make game unique)
1. **LLM Integration** - Dynamic dialogue
2. **Enhanced Minimap Features** - Navigation polish
3. **Audio System Expansion** - Immersive soundscape
4. **Performance Optimization** - Smooth experience

**Estimated Time:** 2-4 weeks

---

## 🚀 Quick Wins (Easy, High Impact)

These can be done quickly and improve the experience significantly:

1. **Placeholder portrait** (1 hour) - Stop missing asset warnings
2. **Typewriter text effect** (1 day) - Professional dialogue feel
3. **Damage numbers** (1 day) - Combat feedback
4. **Quest markers on minimap** (1 day) - Easy navigation
5. **Taming progress bar** (1 day) - Clear feedback
6. **FPS counter debug** (1 hour) - Performance monitoring
7. **Audio volume sliders** (1 day) - User control

---

## 📝 Notes

### Documentation Status
- ✅ Dialogue System - Fully documented
- ✅ Minimap System - Fully documented
- ✅ African Names Database - Fully documented
- ⚠️ Other systems - Need documentation

### Testing Status
- ❌ No automated tests yet
- ⚠️ Manual testing only
- 🎯 Need unit tests for core systems

### Known Issues
1. Save/load doesn't restore entities
2. Sprite swapping not functional
3. No portrait assets exist
4. Some systems have unused variable warnings

---

**Last Updated:** December 2024
**Total Features Remaining:** 18
**Estimated Total Time:** 8-12 weeks (with art assets)
**Estimated Total Time (Code Only):** 4-6 weeks
