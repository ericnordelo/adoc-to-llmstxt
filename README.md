# llms.txt generator

Is active in deadline

> [!NOTE]
> This tool is specifically designed to work with the OpenZeppelin doc-site format.

A command-line tool that generates llms.txt files from Antora documentation sites. The llms.txt format is designed to provide a structured summary of documentation that can be easily consumed by LLMs (Large Language Models).

The tool scans Antora documentation directories and generates a standardized llms.txt file containing:

- Title and description of the documentation
- Links to key documentation pages with short descriptions
- API reference links organized by module
- Structured sections for easy parsing

This helps LLMs better understand and reference your project's documentation when answering questions or providing assistance.

> [!TIP]
>Check [LLMs.txt Explained](https://medium.com/towards-data-science/llms-txt-414d5121bcb3).

## Features

| name                            | status |
|---------------------------------|--------|
| standard generation (llms.txt)  | ✅      |
| full generation (llms-full.txt) | ✅      |

## Usage

### Using the files

While this tool main purpose is to generate the files, you can also find the generated results for different versions of the OpenZeppelin Contracts for Cairo library inside the [llmstxts/](llmstxts) directory. This files can then be used for providing context to different AI applications like [Cursor](https://www.cursor.com/) or [ChatGPT](https://chatgpt.com/).

Check [cairo-v1.0.0](llmstxts/cairo-v1.0.0.txt) as an example.

### Generating the files

The `adoc` subcommand expects a directory containing the adoc files (and a [llmstxt.toml configuration file](#config)):

```bash
oz-llmtxt adoc -d examples/cairo/v1.0.0 -v 1.0.0
```

The optional `--library_version` or `-v` argument allows to specify which version of the library this represents.

To generate the full format containing all the information in a single file, use the optional `--full` or `-f` argument:

```bash
oz-llmtxt adoc -d examples/cairo/v1.0.0 -f
```

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
