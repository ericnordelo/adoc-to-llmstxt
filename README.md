# llms.txt generator

> [!NOTE]
> This tool is specifically design to work with the OpenZeppelin doc-site format.

A command-line tool that generates llms.txt files from Antora documentation sites. The llms.txt format is designed to provide a structured summary of documentation that can be easily consumed by LLMs (Large Language Models).

The tool scans Antora documentation directories and generates a standardized llms.txt file containing:

- Title and description of the documentation
- Links to key documentation pages with short descriptions
- API reference links organized by module
- Structured sections for easy parsing

This helps LLMs better understand and reference your project's documentation when answering questions or providing assistance.

> [!TIP]
>Check [LLMs.txt Explained](https://medium.com/towards-data-science/llms-txt-414d5121bcb3).

## Usage

### Using the files

While this tool main purpose is to generate the files, you can also find the generated results for different versions of the OpenZeppelin Contracts for Cairo library inside the [llmstxts/](llmstxts) directory. This files can then be used for providing context to different AI applications like [Cursor](https://www.cursor.com/) or [ChatGPT](https://chatgpt.com/).



### Generating the files

The `adoc` subcommand expects a directory containing the adoc files (and a [llmstxt.toml configuration file](#config)):

```bash
oz-llmtxt adoc -d examples/cairo/v0.20.0 -v 0.20.0
```

The optional `--library_version` or `-v` param allows to specify which version of the library this represents.

## Config

The tool expects a `llmstxt.toml` file in the adoc files root directory with the following format:

```toml
title = "The title"
description = "The short descrption"
long_description = "The long description"
base_url = "The doc-site base url"
```

### Example

You can find an example of a llmstxt.toml config file [here](examples/cairo/v0.20.0/llmstxt.toml).
