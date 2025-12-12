# Model Simplification Plan

## Problem: Duplication Between Rust and Flutter

Currently there's duplication of data models:

### Rust FFI Structs (backend/src/ffi.rs)
```rust
pub struct Game {
    pub id: String,
    pub title: String,
    pub icon_url: String,
    // ... flat structure
}
```

### Flutter Models (lib/src/models/game.dart)
```dart
class Game {
    final String id;
    final Display display;  // Nested!
}

class Display {
    final String title;
    final Image icon;  // More nesting!
}
```

### Current Conversion Code
```dart
Game _gameFromRust(dynamic game) {
    return Game(
        id: game.id,
        display: Display(
            title: game.title,
            icon: Image(url: game.iconUrl),
            // Manual mapping...
        )
    );
}
```

## Solution: Single Source of Truth

### Option A: Use Auto-Generated Dart Classes (RECOMMENDED)

**Advantages:**
- ✅ Single source of truth (Rust structs)
- ✅ Zero duplication
- ✅ No manual conversion code
- ✅ Type-safe by design
- ✅ Less maintenance

**Changes Required:**
1. Remove custom Flutter models (`lib/src/models/game.dart`)
2. Use auto-generated classes from `package:elysia/api.dart`
3. Update UI code to use flat structure:
   - `game.display.title` → `game.title`
   - `game.display.icon.url` → `game.iconUrl`

**Example:**
```dart
// Before
import '../models/models.dart';
final title = game.display.title;

// After
import 'package:elysia/api.dart';
final title = game.title;
```

### Option B: Nested Rust Structs (NOT RECOMMENDED)

Make Rust structs match Flutter's nested structure. This is worse because:
- ❌ More complex Rust code
- ❌ Unnecessary nesting in FFI layer
- ❌ Still have duplication (Rust defines structure Flutter consumes)

## Migration Path

### Phase 1: Gradual Migration
1. Keep both models temporarily
2. Add `// ignore: unused_import` to old models
3. Update pages one by one to use auto-generated classes
4. Remove old models when all pages migrated

### Phase 2: Remove Conversion Layer
Once all UI code uses auto-generated classes:
1. Delete `lib/src/models/game.dart`
2. Delete `_gameFromRust()` conversion functions
3. Return auto-generated classes directly from provider

### Phase 3: Simplify Provider
```dart
// Before
List<Game> _games = [];

Future<void> _getGames() async {
    final games = await rust_api.getAllGames();
    _games = games.map((g) => _gameFromRust(g)).toList();
}

// After
List<Game> _games = [];  // Use generated class directly!

Future<void> _getGames() async {
    _games = await rust_api.getAllGames();  // No conversion needed!
}
```

## Benefits Summary

After migration:
- **~500 lines less code** (remove models and conversions)
- **Zero duplication** of data structures
- **Compile-time safety** if Rust API changes
- **Simpler mental model** for developers
- **Follows FRB best practices**

## When to Keep Custom Models

Keep custom models if:
- UI needs computed properties not in Rust data
- Complex view-specific transformations needed
- Multiple Rust structs combine into one view model

For Elysia, the Rust structs map 1:1 to UI needs, so custom models aren't necessary.

## Current Status

The conversion layer exists in:
- `app_provider.dart`: `_gameFromRust()`, `_contentFromRust()`
- `components_page.dart`: Manual mapping of runners/components

These can be removed once the migration is complete, further simplifying the codebase.
