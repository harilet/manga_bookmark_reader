# Manga Bookmark Reader

A desktop application built with **Tauri**, **SvelteKit**, and **TypeScript** that helps you track and read manga from various online sources.

## Features

- 📚 Track reading progress across multiple manga sites
- 🎯 Automatic chapter detection and navigation
- 🔧 Configurable site-specific rules
- 🖥️ Native desktop application with minimal resource usage
- 🌐 Support for multiple manga aggregator sites

## Tech Stack

- **Frontend**: SvelteKit + TypeScript + Vite
- **Backend**: Tauri (Rust)
- **Package Manager**: npm

## Prerequisites

- npm v25.0.0

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd manga_bookmark_reader_old
```

2. Install dependencies:
```bash
npm install
```

## Development

Run the development server:
```bash
npm run tauri dev
```

## Building

Create a production build:
```bash
npm run tauri build
```

Built binaries will be in `src-tauri/target/release/bundle/`

## Configuration: rules.json

The `config/rules.json` file contains site-specific rules that tell the application how to navigate chapters and verify pages on different manga websites.

### File Location
```
config/rules.json
```

### Rule Structure

Each rule in the JSON array follows this structure:

```json
{
  "site_match": ["site1.com", "site2.com"],
  "page_check": "HTML selector or text to verify page loaded correctly",
  "main_check": "HTML selector or text to find next chapter link",
  "type": "detection_type"
}
```

### Fields Explained

| Field | Type | Description |
|-------|------|-------------|
| `site_match` | `string[]` | Array of domain names/keywords this rule applies to. The app matches these against the URL. |
| `page_check` | `string` | **Page validation check** - HTML selector, class, or text that confirms the correct page is loaded. If this element/text is not found, the page may be a error page, or invalid chapter. |
| `main_check` | `string` | **Next chapter locator** - HTML selector, class, or text used to find the link to the next chapter. |
| `type` | `string` | Detection method (see below) |
| `value` | `string` | Optional static value for `static` type rules |

### Detection Types

| Type | Description | Use Case |
|------|-------------|----------|
| `auto` | Automatic detection using page_check for validation and main_check for navigation | Standard manga sites with prev/next buttons |
| `toggle` | Toggle-based navigation with page validation | Sites with chapter toggle buttons |
| `inverse_toggle` | Inverted toggle logic with page validation | Sites with non-standard navigation structure |
| `static` | Fixed value, no DOM detection needed | Sites with consistent chapter structure or API-based navigation |

### How page_check Works

The `page_check` field is **not** for finding the previous chapter link. Instead, it validates that the correct chapter page has loaded successfully:

- ✅ If `page_check` element/text is found → Page is valid, continue
- ❌ If `page_check` element/text is NOT found → Page may be error, or invalid

This is important because:
- A 404 error page won't have the expected chapter content
- Some sites redirect to different pages when chapters don't exist
- Network errors may load incomplete pages

### Example Rules

```json
[
  {
    "site_match": ["natomanga", "manganato"],
    "page_check": "PREV CHAPTER",
    "main_check": "class=\"back\"",
    "type": "auto"
  },
  {
    "site_match": ["mangadex"],
    "type": "static",
    "value": "1"
  },
  {
    "site_match": ["asuracomic", "asurascans"],
    "page_check": "rel=\"prev\"",
    "main_check": "rel=\"next\"",
    "type": "toggle"
  }
]
```

### Adding a New Site Rule

1. **Inspect the website** using browser DevTools (F12)
2. **Find a unique element** that only appears on valid chapter pages (for `page_check`)
3. **Find the next chapter link** selector or text (for `main_check`)
4. Open `config/rules.json`
5. Add a new rule object to the array:
```json
{
  "site_match": ["yourmangasite.com"],
  "page_check": "class=\"chapter-content\"",
  "main_check": "Next Chapter",
  "type": "toggle"
}
```
6. Save the file
7. Restart the application

### Tips for Creating Rules

- **For page_check**: Choose elements that won't appear on 404/error pages (chapter title, reader container, navigation buttons)
- **For main_check**: Use specific selectors like `class="btn next"` or unique text like "Next Chapter"
- **Test on multiple chapters** including first and last chapters
- **Check for dynamic content** that might load after page render
- **Use site domain keywords** in `site_match` that appear in the URL
- **Escape special characters** in selectors (quotes, etc.)

## Supported Sites

The application currently includes rules for:
- MangaNato / Natomanga
- ManhwaClan
- Flame Comics
- Asura Scans / AsuraComic
- MangaDex
- LikeManga / MangaYY
- hto.to / xbato.com
- Webtoons
- MangaRead
- Demonics Scans
- Greed Scans
- Mgeko
- Manga Blaze
- MGRead

## Project Structure

```
manga_bookmark_reader/
├── config/
│   └── rules.json          # Site navigation and validation rules
├── src/                    # SvelteKit frontend source
├── src-tauri/              # Tauri backend (Rust)
│   ├── src/                # Rust source code
│   └── tauri.conf.json     # Tauri configuration
├── build/                  # Production build output
├── static/                 # Static assets
└── README.md               # This file
```

## Scripts

| Command | Description |
|---------|-------------|
| `npm run tauri dev` | Run Tauri app in development mode |
| `npm run tauri build` | Build Tauri app for production |

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests, especially for adding new site rules to `config/rules.json`.
