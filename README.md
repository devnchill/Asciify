# ASCII Video Generator

> **DISCLAMER: Work In Progress**
>
> This project is still under active development. The implementation is currently unoptimized and may contain bugs, inefficiencies, and incomplete features. Performance improvements and additional functionality are planned for future releases.

Convert images and videos into ASCII-art representations using Rust and FFmpeg.

---

## Demo

### Image Conversion

<img width="910" height="1440" alt="Image" src="https://github.com/user-attachments/assets/ab416eab-8dfe-49b8-bb88-614383d743fb" />

### Video Conversion

https://github.com/user-attachments/assets/e1844666-1e6c-46ca-96a3-a1e346062d2c

---

## Overview

This project converts images and videos into ASCII art by mapping pixel brightness values to text characters of varying visual density.

For videos, the workflow consists of:

1. Extracting video frames.
2. Converting each frame into ASCII art.
3. Rendering the ASCII representation back into image frames.
4. Combining the generated frames into a final video.

---

## Installation

### Nix Users

**Run directly (no clone needed):**

```console
nix run github:devnchill/Asciify
```

**Build locally:**

```console
git clone https://github.com/devnchill/Asciify
cd Asciify
nix build
```

### Non-Nix Users

Requires [Rust](https://rustup.rs) and [ffmpeg](https://ffmpeg.org/download.html) on your system.

```console
git clone https://github.com/devnchill/Asciify
cd Asciify
cargo run --release
```

## Contributing

Check the [open issues](https://github.com/devnchill/Asciify/issues) if you'd like to help out.

**Nix users:** simply run `nix develop` to get a shell with all dependencies (Rust toolchain + ffmpeg).

---

## Future Improvements

- [ ] Faster frame processing
- [ ] Parallel frame generation
- [ ] Colored ASCII output
- [ ] Custom character ramps
- [ ] Adjustable output resolution
- [ ] Audio preservation
- [ ] Better font rendering
- [ ] Reduced memory usage

---

## How It Works

### 1. Frame Extraction

The input video is decomposed into individual frames using FFmpeg.

```text
Input Video
     │
     ▼
Frame 000001.png
Frame 000002.png
Frame 000003.png
...
```

Each frame is stored as a PNG image for processing.

---

### 2. Image Processing

For every frame:

- The image is loaded into memory.
- The image is resized to the target ASCII dimensions.
- Pixel brightness values are calculated.

Conceptually:

```text
Dark Pixel  ─────► Dense Character
Bright Pixel ───► Sparse Character
```

---

### 3. Character Mapping

Each brightness value is mapped to a character from a predefined density ramp.

Example:

```text
@%#*+=-:.
```

Darker pixels become:

```text
@
#
%
```

Brighter pixels become:

```text
.
:
-
```

This creates the illusion of shading using only characters.

---

### 4. ASCII Frame Generation

After mapping, each frame becomes a text-based representation.

Example:

```text
@@@@@@@@@@@@@@
@@@@@@##***++=
@@@##*++==--::
###*++==-::...
```

The ASCII output is then rendered back into an image frame.

This step allows standard video encoders to process the output.

---

### 5. Video Reconstruction

Once all ASCII frames have been generated, FFmpeg combines them into a video.

```text
ASCII Frames
      │
      ▼
FFmpeg Encoding
      │
      ▼
ASCII Video (.mp4)
```

The final result preserves the original motion while displaying every frame as ASCII art.

---

## Pipeline

```text
Input Video
     │
     ▼
Frame Extraction
     │
     ▼
PNG Frames
     │
     ▼
ASCII Conversion
     │
     ▼
ASCII Frames
     │
     ▼
Video Encoding
     │
     ▼
ASCII Video
```

---

## Tech Stack

- Rust
- FFmpeg


