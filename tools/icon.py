#!/usr/bin/env python3
"""Draw icon.png: pixel-art particles rising over a toggle switch (32x32, scaled to 512)."""
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
GRID, SCALE = 32, 16

# Each art row is drawn at (x, y); "." is transparent.
PALETTE = {
    "Y": (255, 236, 120), "O": (255, 160, 40), "R": (230, 70, 30),  # flame
    "H": (235, 45, 70), "h": (255, 130, 150),  # heart
    "W": (250, 250, 255), "C": (150, 220, 255),  # crit sparkle
    "G": (150, 150, 165), "g": (195, 195, 205),  # smoke
    "V": (90, 230, 110), "B": (110, 140, 255), "P": (200, 120, 255),  # sparks
    "T": (124, 92, 255), "t": (96, 66, 220),  # switch track
    "K": (246, 244, 255), "k": (205, 198, 235),  # switch knob
}
SPRITES = [
    (5, 7, ["..R..", ".RR..", ".ROR.", "ROOOR", "ROYOR", "OYYYO", ".OYO."]),  # flame
    (21, 5, [".hh.hh.", "hHHhHHH", "HHHHHHH", ".HHHHH.", "..HHH..", "...H..."]),  # heart
    (13, 3, ["..C..", "..W..", "CWWWC", "..W..", "..C.."]),  # crit
    (10, 13, ["gg", "gG"]),  # smoke
    (18, 14, [".g.", "gGg", ".g."]),  # smoke
    (16, 10, ["V"]), (26, 12, ["B"]), (4, 15, ["P"]), (24, 16, ["Y"]), (8, 3, ["B"]), (28, 8, ["V"]),
    (5, 20, [  # the switch's track
        "..ttttttttttttttttt..",
        ".tTTTTTTTTTTTTTTTTTt.",
        "tTTTTTTTTTTTTTTTTTTTt",
        "tTTTTTTTTTTTTTTTTTTTt",
        "tTTTTTTTTTTTTTTTTTTTt",
        "tTTTTTTTTTTTTTTTTTTTt",
        ".tTTTTTTTTTTTTTTTTTt.",
        "..ttttttttttttttttt..",
    ]),
    (17, 20, [  # its knob, on
        "..KKKK..",
        ".KKKKKK.",
        "KKKKKKKK",
        "KKKKKKKK",
        "KKKKKKKk",
        "KKKKKKKk",
        ".kKKKKk.",
        "..kkkk..",
    ]),
]


def background(x, y):
    # A rounded square with Horizon's dark violet, lighter toward the bottom.
    radius = 4
    cx = min(max(x, radius), GRID - 1 - radius)
    cy = min(max(y, radius), GRID - 1 - radius)
    if (x - cx) ** 2 + (y - cy) ** 2 > radius ** 2 + 1:
        return (0, 0, 0, 0)
    t = y / (GRID - 1)
    return (int(26 + 16 * t), int(20 + 12 * t), int(48 + 30 * t), 255)


def main():
    image = Image.new("RGBA", (GRID, GRID))
    for y in range(GRID):
        for x in range(GRID):
            image.putpixel((x, y), background(x, y))
    for ox, oy, rows in SPRITES:
        for dy, row in enumerate(rows):
            for dx, c in enumerate(row):
                if c != ".":
                    image.putpixel((ox + dx, oy + dy), (*PALETTE[c], 255))
    out = ROOT / "icon.png"
    image.resize((GRID * SCALE, GRID * SCALE), Image.NEAREST).save(out, optimize=True)
    print(out)


if __name__ == "__main__":
    main()
