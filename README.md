# Wallity

> **⚠️ Work In Progress**  
> This project is currently under active development. Features may be incomplete or subject to change.

A desktop wallpaper manager built with Tauri, SvelteKit, and TypeScript, designed for **Wayland/Hyprland** environments.

## Platform Support

This application is specifically designed for:
- **Wayland** compositors
- **Hyprland** window manager

## Features

- Browse and manage wallpapers
- Virtual scrolling for performance
- Keyboard navigation support
- Search functionality
- Thumbnail generation and caching
- Config file support

## Tech Stack

- **Frontend**: SvelteKit 5, TypeScript, Tailwind CSS
- **Backend**: Tauri 2, Rust
- **Package Manager**: Bun

## Development

### Prerequisites

- [Bun](https://bun.sh/)
- [Rust](https://www.rust-lang.org/) (with `cargo`)
- System dependencies for Tauri development:
  ```bash
  # Arch Linux / Manjaro
  sudo pacman -S webkit2gtk base-devel curl wget file openssl appmenu-gtk-module gtk3 libappindicator-gtk3 librsvg libvips
  
  # Ubuntu / Debian
  sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
  ```

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/wallity.git
   cd wallity
   ```

2. Install dependencies:
   ```bash
   bun install
   ```

### Running the Application

#### Development Mode
```bash
# Run with hot-reload (recommended for development)
bun run tauri dev
```

This will start both the SvelteKit dev server and the Tauri application with hot-reload enabled.

#### Build for Production
```bash
# Build the application
bun run tauri build
```

The built application will be available in `src-tauri/target/release/`.

#### Other Commands
```bash
# Run SvelteKit dev server only (for frontend development)
bun run dev

# Build frontend only
bun run build

# Type checking
bun run check

# Type checking with watch mode
bun run check:watch
```

## Configuration

The application reads configuration from `~/.config/wallity/wallity.toml`. If the file doesn't exist, default values will be used.

### Config File Location
```
~/.config/wallity/wallity.toml
```

### Configuration Options

Create or edit the config file with the following options:

```toml
# Path to the directory containing your wallpapers
# Default: ~/Pictures/wallpapers
wallpaper_path = "~/Pictures/wallpapers"

# Path where the current wallpaper symlink will be created
# This symlink points to the currently selected wallpaper
# Default: ~/.config/wallity/.current_wallpaper
current_wallpaper = "~/.config/wallity/.current_wallpaper"

# Script to execute after setting a wallpaper
# For Hyprland, you might use: hyprctl hyprpaper wallpaper "eDP-1,~/.config/wallity/.current_wallpaper"
# Default: "" (empty)
post_script = ""

# Directory where thumbnail cache is stored
# Default: ~/.cache/wallity/thumbnails
cache_path = "~/.cache/wallity/thumbnails"
```

### Example Configuration

#### For Hyprland with hyprpaper:
```toml
wallpaper_path = "~/Pictures/wallpapers"
current_wallpaper = "~/.config/wallity/.current_wallpaper"
post_script = "hyprctl hyprpaper wallpaper 'eDP-1,~/.config/wallity/.current_wallpaper'"
cache_path = "~/.cache/wallity/thumbnails"
```

#### For Hyprland with swww:
```toml
wallpaper_path = "~/Pictures/wallpapers"
current_wallpaper = "~/.config/wallity/.current_wallpaper"
post_script = "swww img ~/.config/wallity/.current_wallpaper"
cache_path = "~/.cache/wallity/thumbnails"
```

### Notes
- All paths support tilde (`~`) expansion for home directory
- The `post_script` is executed after the wallpaper symlink is created
- If no config file exists, the application will use default values
- The config directory and cache directory will be created automatically if they don't exist

## TODO

### High Priority
- [ ] **Cache cleanup** - Implement cache management and cleanup functionality
- [ ] **More config options** - Add configuration for select-on-close behavior
- [ ] **Virtualization improvements** - Optimize virtual scrolling for better performance with large collections
- [ ] **Light/Dark mode** - Theme switching based on config or system preference

### Features
- [ ] Multi-monitor support - Set different wallpapers per monitor
- [ ] Wallpaper rotation/slideshow - Automatic wallpaper changing on timer
- [ ] Favorites/Collections - Organize wallpapers into custom collections
- [ ] Batch operations - Delete, move, or set multiple wallpapers at once
- [ ] Sort options - Sort by date, name, size, resolution
- [ ] Filter options - Filter by resolution, aspect ratio, file type
- [ ] Import/Export settings - Backup and restore configuration
- [ ] Preview mode - Full-screen wallpaper preview

### Improvements
- [ ] Custom thumbnail cache location via config
- [ ] Keyboard shortcuts customization
- [ ] Better error handling and user feedback
- [ ] Add loading indicators for thumbnail generation
- [ ] Memory usage optimizations
- [ ] Support for animated wallpapers (GIF, video)
- [ ] Wallpaper metadata display (resolution, file size, date)
- [ ] Drag-and-drop support for adding wallpapers

### UI/UX
- [ ] Add tooltips for keyboard shortcuts
- [ ] Improve search with filters and tags
- [ ] Add grid size adjustment controls
- [ ] Context menu for wallpaper actions
- [ ] Better handling of slow/large directory scanning

## License

This project is open source and available under the [MIT License](LICENSE).
