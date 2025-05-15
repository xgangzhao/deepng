# deepng

## Introduction
`deepng` is a tool designed to hide data within PNG files. It provides two main subcommands: `encode` and `decode`, allowing users to embed and extract data from PNG files respectively.

## Usage

### Encode
The `encode` subcommand is used to embed data into a PNG file. It supports various options to customize the encoding process.

#### Command Syntax
```bash
deepng encode [OPTIONS] --file <FILE> --type <TYPE> <--message-file <FILE>|--message <TEXT>>
```

#### Options
- `-f, --file <FILE>`: Specifies the PNG file into which the data will be encoded.
- `-t, --type <TYPE>`: Specifies the chunk type to use for encoding.
- `-M, --message-file <FILE>`: Specifies a file containing the content to be encoded into the PNG file.
- `-m, --message <TEXT>`: Specifies the text message to be encoded into the PNG file.
- `-o, --out <FILE>`: Specifies the output file where the encoded PNG will be written.
- `-h, --help`: Displays help information for the `encode` command.

#### Example
To encode a text message "Hello, World!" into a PNG file named `example.png` using the chunk type `iTXt` and save the output as `encoded_example.png`, you can use the following command:
```bash
deepng encode --file example.png --type hiDe --message "Hello, World!" --out encoded_example.png
```

### Decode
The `decode` subcommand is used to extract data from a PNG file that has been previously encoded using the `encode` command.

#### Command Syntax
```bash
deepng decode [OPTIONS] --file <FILE> --type <TYPE>
```

#### Options
- `-f, --file <FILE>`: Specifies the PNG file from which the data will be decoded.
- `-t, --type <TYPE>`: Specifies the chunk type used for decoding.
- `-M, --message-file <FILE>`: Specifies a file to store the decoded content.
- `-o, --out <FILE>`: Specifies the output file where the decoded data will be written.
- `-h, --help`: Displays help information for the `decode` command.

#### Example
To decode data from a PNG file named `encoded_example.png` using the chunk type `iTXt` and save the decoded message to a file named `decoded_message.txt`, you can use the following command:
```bash
deepng decode --file encoded_example.png --type hiDe --message-file decoded_message.txt
```

## Installation
To install `deepng`, you can use the following command:
```bash
cargo build
```

## Contributing
Contributions to `deepng` are welcome! If you find any bugs or have suggestions for improvements, please open an issue or submit a pull request on the [GitHub repository](https://github.com/yourusername/deepng).

## License
`deepng` is licensed under the [MIT License](LICENSE).