# GitHub Activity Rust Project
This is a Rust project that allows you to fetch the latest GitHub repositories that a user has contributed to.

## Installation
To use this project, you'll need to have Rust installed on your machine. You can download Rust from the official website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Once Rust is installed, clone this repository and navigate to the project directory in your terminal:

```bash
git clone https://github.com/mtclinton/github-activity-rust.git
cd github-activity
```

## Usage
To use this project, simply run the following command:

```bash
cargo run USERNAME
```
Replace **USERNAME** with the GitHub username you want to check.

You'll need to provide a valid GitHub access token as an environment variable named **GITHUB_ACCESS_TOKEN**

The program will output a list of the latest repositories that the user has contributed to on GitHub, along with their names and URLs.

## Dependencies
This project uses the following dependencies:

- reqwest (0.9.24) - A Rust HTTP client library.
- serde (1.0) - A Rust library for serializing and deserializing data structures.
- serde_json (1.0) - A Rust library for working with JSON data.
## Author
This project was written by ChatGPT, a large language model trained by OpenAI based on the GPT-3.5 architecture.

##License
This project is licensed under the MIT License - see the LICENSE.md file for details.
