
#!/bin/bash

# Check if a name is provided as an argument
if [ -z "$1" ]; then
  echo "Usage: $0 <folder-name>"
  exit 1
fi

# Get the name from the first argument
FOLDER_NAME="$1"

# Create a folder with the given name
mkdir -p "$FOLDER_NAME"

# Create a .gitignore file with 'target' as the content
echo "target" > "$FOLDER_NAME/.gitignore"

# Initialize a new cargo binary project
cd "$FOLDER_NAME"
cargo init --bin

# Create input folder and file
mkdir -p src/input
touch src/input/input.txt

# Replace the content of main.rs with the provided code
cat > src/main.rs << EOF
use std::time::{SystemTime, UNIX_EPOCH};

fn solve_hard(input: &str) {}

fn solve_easy(input: &str) {}

fn main() {
    let input = include_str!("input/input.txt");

    // start easy
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    solve_easy(&input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("easy took {:?}", end-start);
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    solve_hard(&input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("hard took {:?}", end-start); 
}
EOF

# Navigate back to the original directory
cd ..

if [ -f cargo.toml ]; then
  # Modify the members array to include the new folder, handling trailing commas
  sed -i.bak -E "s/(members = \\[.*)(\\])/\\1, \"$FOLDER_NAME\"\\2/" cargo.toml
  echo "Added '$FOLDER_NAME' to Cargo.toml members array."
else
  echo "Cargo.toml not found in the current directory. Please ensure it exists and try again."
fi

# Notify the user
echo "Cargo project initialized in '$FOLDER_NAME' with updated main.rs and input folder."
