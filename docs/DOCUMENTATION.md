General Project Directory Structure for GEDStats:

gedstats/
├── flutter_ui/               # The Flutter project (Frontend)
│   ├── lib/
│   │   ├── main.dart
│   │   ├── slides/           # UI code for "Wrapped" slides
│   │   └── rust/             # Auto-generated bridge code
│
├── rust_backend/             # The Rust project (Backend)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs            # The Public API exposed to Flutter
│       ├── api.rs            # The bridge entry point
│       ├── domain/           # Pure logic (No external deps ideally)
│       │   ├── person.rs     # Structs for Person, Family
│       │   └── dna.rs        # Structs for DNA markers (SNPs)
│       ├── parsers/          # Handling dirty input
│       │   ├── gedcom.rs     # Reading .ged files
│       │   └── dna_raw.rs    # Reading 23andMe .txt files
│       └── analytics/        # The "Wrapped" Logic
│           ├── mod.rs
│           ├── longevity.rs  # E.g., Average lifespan calc
│           ├── migration.rs  # E.g., Location mapping
│           └── heritage.rs   # E.g., DNA trait matching