# kachi

> ⚠️ **Experimental** — This project is in early development. Many features are incomplete or broken. Use at your own risk.

<!-- ![CI passing](https://github.com/Haberkamp/kachi/actions/workflows/ci.yml/badge.svg?event=push&branch=main) -->

![Created by](https://img.shields.io/badge/created%20by-@n__haberkamp-065afa.svg)
![MIT License](https://img.shields.io/badge/license-MIT-blue)

## Highlights

- Minimal, beautiful keystroke overlay
- Modifier keys displayed with macOS symbols (⌘, ⌃, ⌥, ⇧)
- Auto-resizing floating window
- Runs silently in the background (no dock icon)
- Transparent, blurred glass effect
- Built with Tauri for native performance

## Overview

Kachi displays your keystrokes in a sleek overlay at the bottom of your screen. Perfect for screencasts, tutorials, live coding sessions, and presentations where you want viewers to see what you're typing.

### Author

Hey, I'm Nils. In my spare time [I write about things I learned](https://www.haberkamp.dev/) or I [create open source packages](https://github.com/Haberkamp), that help me (and hopefully you) to build better apps.

## Installation

Right now, you need to build from source:

```bash
# Clone the repo
git clone https://github.com/Haberkamp/kachi.git
cd kachi

# Install dependencies
bun install

# Run in development
bun run tauri dev

# Build for production
bun run tauri build
```

## Usage

1. Launch Kachi
2. Grant accessibility permissions when prompted (required to read keystrokes)
3. Start typing — your keystrokes appear at the bottom of the screen
4. Access the app via the system tray icon

### Features

#### Modifier Keys

Modifier keys are displayed using native macOS symbols and highlighted in a distinct style:

| Key | Symbol |
|-----|--------|
| Command | ⌘ |
| Control | ⌃ |
| Option | ⌥ |
| Shift | ⇧ |
| Caps Lock | ⇪ |

#### Key Combinations

When you press a key with modifiers, kachi shows them together:

```
⌘ + s
⌃ + ⌥ + t
⇧ + ⌘ + p
```

#### Auto-Fade

Keys automatically fade out after 2 seconds, keeping the overlay clean and unobtrusive.

#### Auto-Resize

The overlay window automatically resizes to fit the displayed keys, staying centered at the bottom of your screen.

## Requirements

- macOS (accessibility permissions required)
- ~50MB disk space

## Tech Stack

- **Frontend**: React 19, TypeScript, Vite
- **Backend**: Rust, Tauri 2
- **Keyboard Input**: device_query crate

## Feedback and Contributing

I highly appreciate your feedback! Please create an [issue](https://github.com/Haberkamp/kachi/issues/new) if you've found any bugs or want to request a feature.
