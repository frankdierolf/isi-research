#!/usr/bin/env python3
"""Generate macOS template tray icons.

For macOS template icons (iconAsTemplate: true):
- Black pixels on transparent background
- macOS automatically inverts colors based on menubar appearance
"""

from PIL import Image, ImageDraw, ImageFont
import os

ICON_SIZE = 22
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))


def create_idle_icon():
    """Create tray-idle.png - black 'I' on transparent background."""
    img = Image.new('RGBA', (ICON_SIZE, ICON_SIZE), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # Draw a bold "I" letter
    # Using basic shapes since system fonts may vary
    center_x = ICON_SIZE // 2

    # Top horizontal bar
    draw.rectangle([center_x - 5, 4, center_x + 5, 6], fill=(0, 0, 0, 255))

    # Vertical bar
    draw.rectangle([center_x - 2, 6, center_x + 2, 16], fill=(0, 0, 0, 255))

    # Bottom horizontal bar
    draw.rectangle([center_x - 5, 16, center_x + 5, 18], fill=(0, 0, 0, 255))

    return img


def create_recording_icon():
    """Create tray-recording.png - black filled circle on transparent background."""
    img = Image.new('RGBA', (ICON_SIZE, ICON_SIZE), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # Draw a filled circle (recording indicator)
    center = ICON_SIZE // 2
    radius = 7
    draw.ellipse(
        [center - radius, center - radius, center + radius, center + radius],
        fill=(0, 0, 0, 255)
    )

    return img


def create_processing_icon():
    """Create tray-processing.png - black gear/spinner on transparent background."""
    img = Image.new('RGBA', (ICON_SIZE, ICON_SIZE), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    center = ICON_SIZE // 2

    # Draw a simple gear shape using circles and rectangles
    # Outer ring
    outer_radius = 8
    inner_radius = 4

    # Draw outer circle
    draw.ellipse(
        [center - outer_radius, center - outer_radius,
         center + outer_radius, center + outer_radius],
        fill=(0, 0, 0, 255)
    )

    # Cut out inner circle (transparent)
    draw.ellipse(
        [center - inner_radius, center - inner_radius,
         center + inner_radius, center + inner_radius],
        fill=(0, 0, 0, 0)
    )

    # Add gear teeth (small rectangles around the circle)
    tooth_length = 3
    tooth_width = 3

    # Top tooth
    draw.rectangle([center - tooth_width//2, 2, center + tooth_width//2, 2 + tooth_length], fill=(0, 0, 0, 255))
    # Bottom tooth
    draw.rectangle([center - tooth_width//2, ICON_SIZE - 2 - tooth_length, center + tooth_width//2, ICON_SIZE - 2], fill=(0, 0, 0, 255))
    # Left tooth
    draw.rectangle([2, center - tooth_width//2, 2 + tooth_length, center + tooth_width//2], fill=(0, 0, 0, 255))
    # Right tooth
    draw.rectangle([ICON_SIZE - 2 - tooth_length, center - tooth_width//2, ICON_SIZE - 2, center + tooth_width//2], fill=(0, 0, 0, 255))

    return img


def main():
    icons = [
        ('tray-idle.png', create_idle_icon),
        ('tray-recording.png', create_recording_icon),
        ('tray-processing.png', create_processing_icon),
    ]

    for filename, create_func in icons:
        img = create_func()
        path = os.path.join(SCRIPT_DIR, filename)
        img.save(path)
        print(f"Created: {path}")


if __name__ == '__main__':
    main()
