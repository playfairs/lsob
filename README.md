# l_SOB

`lsob (l_SOB) is a simple CLI tool to lovingly destroy the clarity of emojis and images.`

## Usage

```text
lsob input.png output.png --blur 16 --pixelate 8
```

The Tauri desktop editor uses a TypeScript frontend with CSS and a Rust image-processing backend. Drop an image onto the canvas, adjust the effect stack in the inspector, and export the full-resolution result.

## Development

```text
cargo xtask check
cargo xtask test
cargo xtask fmt
npm install
npm run tauri dev
nix develop
nix build
```

The interactive canvas uses a 512 px proxy preview so changes stay responsive. Export uses the same proxy and effect pipeline as the live preview, ensuring the exported appearance matches what is shown.
