# Rust Text File Reader

This project is a simple Rust cli application designed to read and process text files. It demonstrates basic file handling and string manipulation in Rust.

## Features

- Read content from a text file.
- Display the content in the terminal.
- Handle errors gracefully if the file does not exist or cannot be read.

## Prerequisites

- Rust installed on your system. You can download it from [rust-lang.org](https://www.rust-lang.org/).

## Usage

1. Clone the repository:
    ```bash
    git clone https://github.com/your-username/rust_text_file_reader.git
    cd rust_text_file_reader
    ```

2. Build the project:
    ```bash
    cargo build
    ```

3. Run the application:
    ```bash
    cargo run <file_path>
    ```
    Replace `<file_path>` with the path to the text file you want to read.

## Example

```bash
cargo run example.txt
```

Output:
```
File content:
Hello, world!
This is an example text file.
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contribution

Contributions are welcome! Feel free to open issues or submit pull requests.
