![asimplevectors](docs/images/logo.svg)

## Overview

[asimplevectors](https://docs.asimplevectors.com/) is a high-performance vector database optimized for retrieval-augmented generation (RAG) vector database. It provides fast and reliable clustering through Rust and the Raft consensus protocol, while leveraging SQLite3 for easy data management. Additionally, the database includes built-in key-value storage for managing original document data within the vector database.

## Key Advantages

- **Optimized for RAG**: Built to handle RAG tasks efficiently.
- **Fast and Reliable Clustering**: Uses Rust and the Raft consensus protocol to deliver a highly performant clustering system.
- **Easy Data Management**: Utilizes SQLite3 for simple and effective data organization.
- **Integrated Key-Value Storage**: Manage original document data alongside vectors with the built-in key-value store.

## Features

- **Version Management**: Includes support for version control, with A/B testing capabilities.
- **Key-Value Storage**: Built-in key-value storage functionality.
- **Snapshot Support**: Offers snapshot capabilities for data backups and state saving.
- **Dense & Sparse Vector Management**: Handles both dense and sparse vector types with *HNSW* & *FAISS*.
- **Filtered Search**: Enables users to filter vectors by metadata attributes in search queries, allowing refined results based on custom conditions.
- **Rerank Capability**: Provides reranking of initial search results using advanced scoring techniques like *BM25*. This feature ensures highly relevant results for document retrieval use cases.

## Docs

For detailed development guidelines and documentation, please refer to the official guide at [https://docs.asimplevectors.com/](https://docs.asimplevectors.com/).

## Architecture

This project is built with Raft consensus to achieve clustering. It leverages components such as [atinyvectors](https://github.com/billionvectors/atinyvectors) (written in C++), faiss, and SQLite3 to provide high-performance vector management and clustering capabilities.

![Overall Architecture](docs/images/overallarchitecture.svg)

## Quick Install from Docker
To download the latest version of the asimplevectors Docker image, use the following
```bash
docker pull billionvectors/asimplevectors:latest
docker run -p 21001:21001 -p 21002:21002 asimplevectors:latest
curl --silent "127.0.0.1:21001/cluster/init" -H "Content-Type: application/json" -d '{}'
```

## How to Run Examples

1. Run `install_dependency.sh` to install necessary dependencies.
2. Copy .env.local file to .env
3. Execute `./run_example.sh search` and select a test file under the `example/` directory.
-> search, security, snapshot, space, vector, version

## How to Run

1. Install dependencies: `./install_dependency.sh`
-> If you get an error that your CMake version is low, we recommend docker build. If you have trouble with *docker build*, get the latest version from [cmake.org](https://cmake.org) and install it. 
2. Build the project: `./build.sh --release`
3. Copy .env.local file to .env
4. Start the vector database node:
```bash
./asimplevectors --id 1 --http-addr 127.0.0.1:21001 --rpc-addr 127.0.0.1:22001 &
```
5. Initialize the cluster with the following curl command:
```bash
curl --silent "127.0.0.1:21001/cluster/init" -H "Content-Type: application/json" -d '{}'
```
## Docker Run

```bash
docker build --build-arg BUILD_TYPE=Release -t asimplevectors .
docker run -p 21001:21001 -p 21002:21002 asimplevectors &
curl --silent "127.0.0.1:21001/cluster/init" -H "Content-Type: application/json" -d '{}'
```

## Clustering
```bash
docker build --build-arg BUILD_TYPE=Release -t asimplevectors .
docker run -p 21001:21001 -p 21002:21002 asimplevectors --id 1 &
curl --silent "127.0.0.1:21001/cluster/init" -H "Content-Type: application/json" -d '{}'

docker run -p 22001:21001 -p 22002:21002 asimplevectors --id 2 &
docker run -p 23001:21001 -p 23002:21002 asimplevectors --id 3 &

# register new cluster
curl --silent "127.0.0.1:21001/cluster/add-learner" -H "Content-Type: application/json" -d '[2, "127.0.0.1:22001", "127.0.0.1:22002"]'
curl --silent "127.0.0.1:21001/cluster/add-learner" -H "Content-Type: application/json" -d '[3, "127.0.0.1:23001", "127.0.0.1:23001"]'
curl --silent "127.0.0.1:21001/cluster/metrics"
```
This adds a clear note about Raft's recommendation for an odd number of nodes but also specifies that two nodes will still work.

## Search Example
### Creating a Space

To create a new Space, you can use the `/api/space` endpoint. Below is an example where a Space called `spacename` is created with a default index configuration. The **dimension** is set to 4, and the **metric** is set to **L2** (Euclidean distance).

```bash
curl "127.0.0.1:21001/api/space" -H "Content-Type: application/json" -d  \
'{
    "name": "spacename",
    "dimension": 4,
    "metric": "L2"
}'
```

### Checking Created Space Information

Once a Space is created, you can check its details by using the /api/space/{spacename} endpoint.

```bash
curl "127.0.0.1:21001/api/space/spacename"
```

### Adding Vectors to the Space

To add vectors to the created Space, you can use the /api/space/{spacename}/vector endpoint. Below is an example where several vectors are added to the spacename Space, each with its own ID, vector data, and metadata.

```bash
curl "127.0.0.1:21001/api/space/spacename/vector" -H "Content-Type: application/json" -d   \
'{
    "vectors": [
        {
            "id": 1,
            "data": [0.1, 0.2, 0.3, 0.4],
            "metadata": {"label": "first"}
        },
        {
            "id": 2,
            "data": [0.5, 0.6, 0.7, 0.8],
            "metadata": {"label": "second"}
        },
        {
            "id": 3,
            "data": [0.9, 0.8, 0.7, 0.6],
            "metadata": {"label": "third"}
        },
        {
            "id": 4,
            "data": [1.0, 0.1, 0.2, 0.3],
            "metadata": {"label": "forth"}
        },
        {
            "id": 5,
            "data": [0.2, 0.3, 0.4, 0.3],
            "metadata": {"label": "fivth"}
        }
    ]
}'
```

### Searching with Dense Vectors

To search using a dense vector, you can use the following JSON format. This will query the database with the given vector and return the most similar vectors.

```bash
curl "127.0.0.1:21001/api/space/spacename/search" -H "Content-Type: application/json" -d  \
'{
    "vector": [0.2, 0.3, 0.4, 0.3]
}'
```

## Support Languages
`asimplevectors` support various programming languages to meet your diverse development needs.
- Python ([guide](https://github.com/billionvectors/client_api/blob/main/python/README.md))
- Javascript
- C#

## Support Metrics
`asimplevectors` supports multiple distance metrics to handle a variety of use cases, offering flexibility in how vectors are compared.
- **L2 Distance (Euclidean Distance)**
- **Cosine Similarity**
- **Inner Product (Dot Product)**

## C API
If you want the C API, please visit [atinyvectors](https://github.com/billionvectors/atinyvectors).

## License

For details, please refer to the `LICENSE` file.

## Support

If you're interested in forming a business partnership with us, please send an email to [support@billionvectors.com](mailto:support@billionvectors.com).
