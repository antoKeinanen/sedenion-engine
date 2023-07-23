# Solvurus Engine
![npm](https://img.shields.io/npm/v/solvurus_engine)
![GitHub](https://img.shields.io/github/license/antoKeinanen/solvurus-engine)

Solvurus Engine is an open-source project that offers a scientific calculator and CAS (Computer Algebra System) solver. This repository contains the Rust source code used by Solvurus to parse and evaluate expressions.

## Building the Project

To build the Solvurus engine to WebAssembly (wasm), follow these steps:

1. Make sure you have `wasm-pack` installed on your system.
2. Execute the following command: `wasm-pack build --target bundler`.
3. The NPM package will be generated and stored in the `pkg` directory.

## License

This project is licensed under GNU GPL-v3. You can find more detailed information in the `LICENSE` file located in the root of this repository.

## Features
Currently solvurus engine only supports numerical evaluation.

## Getting Started

To use the Solvurus Engine in your project, follow the steps below:

1. Install the NPM package generated during the build process.
   ```
   npm install solvurus_engine
   ```

2. Import the Solvurus module in your JavaScript or TypeScript code.
   ```javascript
   import {} from 'solvurus-engine';
   ```

3. Call the functions in your app.
   ```javascript
   const result = evaluate('2 + 2');
   console.log(result); // Output: 4
   ```

## Contributing

We welcome contributions from the community to enhance Solvurus Engine further. If you want to contribute, please follow the guidelines outlined in the `contributing.md` file.

## Support

If you encounter any issues or have questions regarding the Solvurus Engine, please [open an issue](https://github.com/antoKeinanen/solvurus-engine/issues) on our GitHub repository.

## Acknowledgments

We would like to express our gratitude to all the developers and contributors who have made this project possible.