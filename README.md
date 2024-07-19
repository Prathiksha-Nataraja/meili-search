# Meili-Search

Meilisearch is a RESTful search API. It aims to be a ready-to-go solution for everyone who wants a fast and relevant search experience for their end-users

For general information on how to use Meilisearch—such as our API reference, tutorials, guides, and in-depth articles refer to the [main documentation website](https://www.meilisearch.com/docs).

## Installation

To use `meili-search sdk`, add this dependency in the cargo.toml
```
[dependencies]
meilisearch-sdk = "0.27.0"
```

## Running a Meilisearch Instance

This crate requires a Meilisearch server to run.

```
# Install Meilisearch
curl -L https://install.meilisearch.com | sh

# Launch Meilisearch
./meilisearch --master-key=masterKey
```