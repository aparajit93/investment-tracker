# Investment Tracker

A local-first desktop application for tracking personal investments and financial accounts.

## Features

- Portfolio management
- Account management
- Asset Management

## Current Status

Investment Tracker is currently under active development.

The following functionality is currently implemented:

- Portfolio creation, editing, and deletion
- Portfolio listing and management
- Local SQLite database with SQLx migrations
- Desktop application shell with sidebar navigation
- Initial Skeleton UI integration

The following functionality is planned:

- Account management
- Asset management
- Investment transactions
- Portfolio dashboard
- Application onboarding
- Persistent current-portfolio context


## Tech Stack

- Tauri v2
- SvelteKit
- TypeScript
- Rust
- SQLite
- SQLx
- Skeleton

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Development


### Prerequisites

The following tools are required to build and run the application:

- Deno (Node.js should work as well)
- npm/deno/pnpm
- Rust: Install from https://rust-lang.org/tools/install/
- Cargo
- Tauri v2: Install Tauri CLI from https://tauri.app/start/create-project/

### Installation

Clone the repository and install the frontend dependencies:

```bash
deno install
```

### Development

Start the application in development mode:

```bash
deno task tauri dev
```

### Production Build

Create a production build:

```bash
deno task tauri build
```


## Project Structure


```text
.  
├── src/ # SvelteKit frontend  
│   ├── lib/ # Shared frontend code and API clients  
│   ├── components/ # Custom Svelte components  
│   └── routes/ # Application routes and pages  
└── src-tauri/ # Rust backend and Tauri application  
    ├── src/ # Rust application code  
    │   ├── commnads/ # Tauri commands  
    │   ├── repositories/ # SQL operations  
    │   ├── models/ # Data models  
    └── migrations/ # SQLx database migrations   
```

## Roadmap

### Core Application

- ~~Portfolio management~~
- Current portfolio context
- Account management
- Asset management
- Investment transaction tracking
- Portfolio dashboard
- Application onboarding

### UI and UX

- ~~Initial application shell~~

- ~~Sidebar navigation~~

- ~~Initial Skeleton UI integration~~

- Evaluate alternative component library approaches

- Responsive UI improvements

### Future

- Data import and export

- Portfolio performance analysis

- Additional investment and asset types

- Application packaging and distribution

## License

...