# tth: Text to Hash Tool

This Rust program, `tth` (text to hash), provides a command-line utility for generating SHA-256 hashes of text or file content.

**Features:**

  - **Versatile Input:** Handles both direct text input and reading from files.
  - **SHA-256 Hashing:** Employs the cryptographically secure SHA-256 algorithm for generating hash values.
  - **Output File:** Writes the generated hash to a designated file named "hash.txt".

**Usage:**
```sh 
tth "This is some text"  # Hash the provided text
tth file.txt             # Hash the content of file.txt
```

**Built with:**

   - Rust programming language

This project offers a simple and effective tool for generating secure hashes for various purposes.
