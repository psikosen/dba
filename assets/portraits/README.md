# Character Portraits

This directory contains portrait images for the dialogue UI system.

## Directory Structure

- **characters/** - Player and main character portraits (512x512 px)
- **bosses/** - Boss character portraits (512x512 px)
- **npcs/** - Village NPCs and other characters (512x512 px)

## Portrait Format

- **Resolution**: 512x512 pixels (high-res for dialogue busts)
- **Style**: Pixel art, waist-up cutout style
- **Background**: Transparent PNG
- **Naming**: `{character_name}_{emotion}.png` (e.g., `kofi_happy.png`, `kofi_sad.png`)

## Emotions/States

Each character should have multiple emotional states:
- `neutral` - Default expression
- `happy` - Joyful/pleased
- `sad` - Sorrowful/depressed
- `angry` - Upset/frustrated
- `surprised` - Shocked/amazed
- `sick` - Ill/weakened (for NPCs with sickness states)
- `waking` - Recovering/partially aware
- `determined` - Focused/resolute

## CrossCode-Style Dialogue

Portraits will be displayed as:
- Large floating busts anchored to bottom corners of screen
- Cutout style (no background box)
- Paired with floating dialogue box with sci-fi aesthetic
- Semi-transparent text container with speaker handle/bracket
