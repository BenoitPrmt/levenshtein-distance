
# 🧮 Levenshtein Distance in Rust

A simple Rust implementation of the [Levenshtein distance algorithm](https://en.wikipedia.org/wiki/Levenshtein_distance), used to calculate the minimum number of single-character edits required to change one string into another.

This project is a learning exercise to explore Rust basics, algorithms, and project organization.

## 📁 Project Structure

```
src/
├── main.rs 
├── levenshtein.rs 
└── spellchecker.rs 
```

## 🚀 Getting Started

Make sure you have [Rust installed](https://rustup.rs/).

```bash
# Clone the project
git clone https://github.com/BenoitPrmt/levenshtein-distance.git
cd levenshtein-distance

# Run the main program
cargo run

# Run tests
cargo test
```

## 📦 Features

- Calculates Levenshtein distance between two strings
- Spell checker using Levenshtein distance
  -> Suggests the closest word from a dictionary for a given input
- Includes unit and integration tests
- Modular project structure

## 🔧 Example Output

```
Distance de Levenshtein entre "chat" et "chats" : 1

Mot recherché: chzt
Suggestions:
- chat
```

## 🧠 Notes

- For simplicity, this implementation assumes ASCII strings. Unicode support would require working with `.chars()` instead of `.as_bytes()`.

## ✅ TODO
- [ ] Import dictionary from a file (spellchecker)
- [ ] Build CLI to check spelling
- [ ] Build little API to use tools easily through other projects

## 📚 License

MIT License. Free to use and modify.
