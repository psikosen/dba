#!/usr/bin/env python3
"""
Placeholder Asset Generator for Shaman's Journey
Generates simple colored rectangles as temporary game assets
"""

from PIL import Image, ImageDraw, ImageFont
import os

# Asset specifications
SPRITES = {
    "player": {
        "player_idle.png": (32, 32, (100, 200, 100)),  # Green
    },
    "monsters": {
        # 9 monsters × 8 states = 72 sprites
        "lion_stable.png": (32, 32, (200, 150, 50)),
        "lion_chaos.png": (32, 32, (255, 50, 50)),
        "lion_corrupt.png": (32, 32, (100, 50, 150)),
        "lion_harmony.png": (32, 32, (100, 200, 255)),
        "lion_spirit.png": (32, 32, (200, 200, 255)),
        "lion_tamed.png": (32, 32, (150, 255, 150)),
        "lion_enraged.png": (32, 32, (255, 100, 0)),
        "lion_purified.png": (32, 32, (255, 255, 200)),

        "hyena_stable.png": (32, 32, (180, 140, 100)),
        "hyena_chaos.png": (32, 32, (255, 50, 50)),
        "hyena_corrupt.png": (32, 32, (100, 50, 150)),
        "hyena_harmony.png": (32, 32, (100, 200, 255)),
        "hyena_spirit.png": (32, 32, (200, 200, 255)),
        "hyena_tamed.png": (32, 32, (150, 255, 150)),
        "hyena_enraged.png": (32, 32, (255, 100, 0)),
        "hyena_purified.png": (32, 32, (255, 255, 200)),

        "serpent_stable.png": (32, 32, (50, 150, 100)),
        "serpent_chaos.png": (32, 32, (255, 50, 50)),
        "serpent_corrupt.png": (32, 32, (100, 50, 150)),
        "serpent_harmony.png": (32, 32, (100, 200, 255)),
        "serpent_spirit.png": (32, 32, (200, 200, 255)),
        "serpent_tamed.png": (32, 32, (150, 255, 150)),
        "serpent_enraged.png": (32, 32, (255, 100, 0)),
        "serpent_purified.png": (32, 32, (255, 255, 200)),

        "vulture_stable.png": (32, 32, (80, 80, 80)),
        "vulture_chaos.png": (32, 32, (255, 50, 50)),
        "vulture_corrupt.png": (32, 32, (100, 50, 150)),
        "vulture_harmony.png": (32, 32, (100, 200, 255)),
        "vulture_spirit.png": (32, 32, (200, 200, 255)),
        "vulture_tamed.png": (32, 32, (150, 255, 150)),
        "vulture_enraged.png": (32, 32, (255, 100, 0)),
        "vulture_purified.png": (32, 32, (255, 255, 200)),

        "crocodile_stable.png": (32, 32, (100, 150, 100)),
        "crocodile_chaos.png": (32, 32, (255, 50, 50)),
        "crocodile_corrupt.png": (32, 32, (100, 50, 150)),
        "crocodile_harmony.png": (32, 32, (100, 200, 255)),
        "crocodile_spirit.png": (32, 32, (200, 200, 255)),
        "crocodile_tamed.png": (32, 32, (150, 255, 150)),
        "crocodile_enraged.png": (32, 32, (255, 100, 0)),
        "crocodile_purified.png": (32, 32, (255, 255, 200)),

        "leopard_stable.png": (32, 32, (200, 180, 100)),
        "leopard_chaos.png": (32, 32, (255, 50, 50)),
        "leopard_corrupt.png": (32, 32, (100, 50, 150)),
        "leopard_harmony.png": (32, 32, (100, 200, 255)),
        "leopard_spirit.png": (32, 32, (200, 200, 255)),
        "leopard_tamed.png": (32, 32, (150, 255, 150)),
        "leopard_enraged.png": (32, 32, (255, 100, 0)),
        "leopard_purified.png": (32, 32, (255, 255, 200)),

        "elephant_stable.png": (32, 32, (150, 150, 150)),
        "elephant_chaos.png": (32, 32, (255, 50, 50)),
        "elephant_corrupt.png": (32, 32, (100, 50, 150)),
        "elephant_harmony.png": (32, 32, (100, 200, 255)),
        "elephant_spirit.png": (32, 32, (200, 200, 255)),
        "elephant_tamed.png": (32, 32, (150, 255, 150)),
        "elephant_enraged.png": (32, 32, (255, 100, 0)),
        "elephant_purified.png": (32, 32, (255, 255, 200)),

        "monkey_stable.png": (32, 32, (160, 120, 80)),
        "monkey_chaos.png": (32, 32, (255, 50, 50)),
        "monkey_corrupt.png": (32, 32, (100, 50, 150)),
        "monkey_harmony.png": (32, 32, (100, 200, 255)),
        "monkey_spirit.png": (32, 32, (200, 200, 255)),
        "monkey_tamed.png": (32, 32, (150, 255, 150)),
        "monkey_enraged.png": (32, 32, (255, 100, 0)),
        "monkey_purified.png": (32, 32, (255, 255, 200)),

        "rhino_stable.png": (32, 32, (140, 140, 140)),
        "rhino_chaos.png": (32, 32, (255, 50, 50)),
        "rhino_corrupt.png": (32, 32, (100, 50, 150)),
        "rhino_harmony.png": (32, 32, (100, 200, 255)),
        "rhino_spirit.png": (32, 32, (200, 200, 255)),
        "rhino_tamed.png": (32, 32, (150, 255, 150)),
        "rhino_enraged.png": (32, 32, (255, 100, 0)),
        "rhino_purified.png": (32, 32, (255, 255, 200)),
    },
    "npcs": {
        "elder.png": (32, 32, (150, 100, 200)),
        "brother.png": (32, 32, (200, 100, 100)),
        "merchant.png": (32, 32, (200, 200, 100)),
        "healer.png": (32, 32, (100, 200, 150)),
        "child.png": (32, 32, (200, 180, 150)),
        "shaman.png": (32, 32, (150, 150, 250)),
    },
    "items": {
        "spirit_orb_small.png": (16, 16, (100, 200, 255)),
        "spirit_orb_medium.png": (16, 16, (100, 220, 255)),
        "spirit_orb_large.png": (16, 16, (100, 240, 255)),
        "health_herb.png": (16, 16, (100, 255, 100)),
        "stamina_root.png": (16, 16, (255, 200, 100)),
        "kora.png": (24, 24, (200, 150, 100)),
        "ngoni.png": (24, 24, (180, 140, 90)),
        "corruption_shard.png": (16, 16, (150, 50, 150)),
        "purification_crystal.png": (16, 16, (255, 255, 200)),
    },
    "ui": {
        "health_bar_fill.png": (100, 10, (255, 100, 100)),
        "health_bar_bg.png": (100, 10, (100, 50, 50)),
        "spirit_bar_fill.png": (100, 10, (100, 200, 255)),
        "spirit_bar_bg.png": (100, 10, (50, 100, 150)),
        "stamina_bar_fill.png": (100, 10, (255, 200, 100)),
        "stamina_bar_bg.png": (100, 10, (150, 100, 50)),
        "button.png": (64, 32, (100, 100, 100)),
        "button_hover.png": (64, 32, (150, 150, 150)),
        "button_pressed.png": (64, 32, (80, 80, 80)),
        "panel.png": (200, 200, (50, 50, 50)),
    }
}

TILES = {
    "grass.png": (32, 32, (80, 150, 80)),
    "dirt.png": (32, 32, (140, 100, 60)),
    "water.png": (32, 32, (50, 100, 200)),
    "corruption_light.png": (32, 32, (150, 100, 150)),
    "corruption_medium.png": (32, 32, (120, 80, 140)),
    "corruption_heavy.png": (32, 32, (100, 50, 120)),
    "corruption_extreme.png": (32, 32, (80, 30, 100)),
    "purified.png": (32, 32, (255, 255, 200)),
    "spirit_world.png": (32, 32, (200, 200, 255)),
}

def create_placeholder_sprite(width, height, color, label=None):
    """Create a simple colored rectangle with optional label"""
    img = Image.new('RGBA', (width, height), color=(*color, 255))

    # Add a border
    draw = ImageDraw.Draw(img)
    border_color = tuple(max(0, c - 50) for c in color)
    draw.rectangle([0, 0, width-1, height-1], outline=(*border_color, 255), width=1)

    # Add label if small enough
    if label and width >= 16 and height >= 16:
        try:
            # Use default font
            text_color = (255, 255, 255) if sum(color) < 400 else (0, 0, 0)
            text_bbox = draw.textbbox((0, 0), label[0])
            text_width = text_bbox[2] - text_bbox[0]
            text_height = text_bbox[3] - text_bbox[1]
            x = (width - text_width) // 2
            y = (height - text_height) // 2
            draw.text((x, y), label[0], fill=(*text_color, 255))
        except:
            pass  # Skip if font issues

    return img

def generate_all_placeholders():
    """Generate all placeholder assets"""
    base_path = "assets/sprites"

    # Generate sprites
    for category, sprites in SPRITES.items():
        category_path = os.path.join(base_path, category)
        os.makedirs(category_path, exist_ok=True)

        for filename, (w, h, color) in sprites.items():
            label = filename.split('_')[0]  # First part of filename
            img = create_placeholder_sprite(w, h, color, label)
            filepath = os.path.join(category_path, filename)
            img.save(filepath)
            print(f"✓ Created {filepath}")

    # Generate tiles
    tiles_path = os.path.join(base_path, "tiles")
    os.makedirs(tiles_path, exist_ok=True)

    for filename, (w, h, color) in TILES.items():
        label = filename.split('.')[0][:1]  # First letter
        img = create_placeholder_sprite(w, h, color, label)
        filepath = os.path.join(tiles_path, filename)
        img.save(filepath)
        print(f"✓ Created {filepath}")

    print(f"\n✅ Generated {len([s for sprites in SPRITES.values() for s in sprites]) + len(TILES)} placeholder assets!")
    print("📁 Location: assets/sprites/")

if __name__ == "__main__":
    generate_all_placeholders()
