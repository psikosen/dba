#!/usr/bin/env python3
"""
Generate placeholder sprite images for the game.
Each sprite is a colored block matching the type/theme of the entity.
"""

from PIL import Image, ImageDraw
import os

# Define output directory
ASSETS_DIR = "/home/user/dba/assets/sprites"

# Color definitions (RGB)
COLORS = {
    # Player
    "player": (77, 128, 255),  # Blue

    # Tiles (32x32)
    "grass": (51, 204, 51),     # Green
    "forest": (26, 128, 26),    # Dark green
    "mountain": (128, 128, 128), # Gray
    "village": (153, 102, 51),   # Brown
    "corrupted": (128, 26, 128), # Purple

    # Monsters (32x32)
    "forest_spirit": (102, 230, 102),  # Light green
    "chaos_hound": (230, 51, 51),      # Red
    "corrupt_shade": (128, 26, 128),   # Purple
    "shadow_beast": (51, 51, 51),      # Dark gray
    "spirit_wisp": (230, 230, 255),    # Pale blue
    "rock_golem": (153, 128, 102),     # Stone brown
    "flame_wraith": (255, 128, 26),    # Orange
    "void_stalker": (26, 0, 51),       # Dark purple

    # NPCs (32x32)
    "villager": (255, 255, 128),    # Yellow
    "elder": (179, 179, 230),       # Light purple
    "merchant": (230, 179, 77),     # Gold
    "farmer": (128, 153, 77),       # Olive
    "hunter": (128, 102, 77),       # Brown
    "child": (255, 204, 153),       # Peach
    "head_shaman": (153, 77, 230),  # Purple
    "brother": (230, 77, 77),       # Bright red

    # Items (16x16)
    "health_potion": (255, 0, 0),   # Red
    "spirit_orb": (77, 204, 255),   # Cyan
    "drum": (153, 77, 26),          # Dark brown
}

# Sprite definitions: (category, name, size)
SPRITES = [
    # Player
    ("player", "player", 32),

    # Tiles
    ("tiles", "grass", 32),
    ("tiles", "forest", 32),
    ("tiles", "mountain", 32),
    ("tiles", "village", 32),
    ("tiles", "corrupted", 32),

    # Monsters
    ("monsters", "forest_spirit", 32),
    ("monsters", "chaos_hound", 32),
    ("monsters", "corrupt_shade", 32),
    ("monsters", "shadow_beast", 32),
    ("monsters", "spirit_wisp", 32),
    ("monsters", "rock_golem", 32),
    ("monsters", "flame_wraith", 32),
    ("monsters", "void_stalker", 32),

    # NPCs
    ("npcs", "villager", 32),
    ("npcs", "elder", 32),
    ("npcs", "merchant", 32),
    ("npcs", "farmer", 32),
    ("npcs", "hunter", 32),
    ("npcs", "child", 32),
    ("npcs", "head_shaman", 32),
    ("npcs", "brother", 32),

    # Items
    ("items", "health_potion", 16),
    ("items", "spirit_orb", 16),
    ("items", "drum", 16),
]

def create_sprite(color, size, with_border=True):
    """Create a simple colored square sprite with optional border."""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # Draw main colored square
    draw.rectangle([0, 0, size-1, size-1], fill=(*color, 255))

    # Add darker border for definition
    if with_border:
        border_color = tuple(max(0, c - 50) for c in color)
        draw.rectangle([0, 0, size-1, size-1], outline=(*border_color, 255), width=1)

    return img

def main():
    """Generate all placeholder sprites."""
    print("Generating placeholder sprites...")

    for category, name, size in SPRITES:
        # Get color
        color = COLORS.get(name, (128, 128, 128))  # Default gray

        # Create sprite
        sprite = create_sprite(color, size)

        # Save
        output_path = os.path.join(ASSETS_DIR, category, f"{name}.png")
        os.makedirs(os.path.dirname(output_path), exist_ok=True)
        sprite.save(output_path)

        print(f"  Created: {output_path}")

    print(f"\nGenerated {len(SPRITES)} placeholder sprites!")

if __name__ == "__main__":
    main()
