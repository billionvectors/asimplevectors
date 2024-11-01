import requests
import time
import argparse

# Helper function to send POST requests
def post_request(url, json_data):
    headers = {'Content-Type': 'application/json'}
    try:
        response = requests.post(url, headers=headers, json=json_data)
        print(f"POST {url} -> Status: {response.status_code}")
        if response.status_code in [200, 201]:
            print(f"Response: {response.json()}")
        else:
            print(f"Error: {response.text}")
    except requests.exceptions.RequestException as e:
        print(f"An error occurred during POST request to {url}: {e}")

# Test function to create a space, upsert vectors, and search vectors
def test_search(host, single_node):
    # Step 1: Create space 'spacename' on leader
    print("Create space 'spacename' on leader")
    create_space_data = {
        "name": "spacename",
        "dimension": 4,
        "metric": "L2",
        "hnsw_config": {
            "M": 64,
            "ef_construct": 500
        }
    }
    post_request(f"{host}21001/api/space", create_space_data)
    time.sleep(1)
    
    print("Space 'spacename' created")
    time.sleep(1)

    # Step 2: Upsert vectors to 'spacename' without specifying version
    print("Upsert vectors to 'spacename' without specifying version")
    upsert_vectors_data = {
        "vectors": [
            {
                "id": 1,
                "data": [0.1, 0.2, 0.3, 0.4],
                "metadata": {"meta": "first"}
            },
            {
                "id": 2,
                "data": [0.5, 0.6, 0.7, 0.8],
                "metadata": {"meta": "second"}
            },
            {
                "id": 3,
                "data": [0.9, 0.8, 0.7, 0.6],
                "metadata": {"meta": "third"}
            },
            {
                "id": 4,
                "data": [1.0, 0.1, 0.2, 0.3],
                "metadata": {"meta": "forth"}
            },
            {
                "id": 5,
                "data": [0.2, 0.3, 0.4, 0.3],
                "metadata": {"meta": "fivth"}
            }
        ]
    }
    post_request(f"{host}21001/api/space/spacename/vector", upsert_vectors_data)
    time.sleep(1)

    # Step 3: Search vectors with specific version id on node 1
    print("Search vectors with specific version id on node 1")
    search_data = {
        "vector": [0.2, 0.3, 0.4, 0.3]
    }
    post_request(f"{host}21001/api/space/spacename/version/1/search", search_data)
    time.sleep(1)

    if not single_node:
        # Search on node 2
        print("Search vectors with specific version id on node 2")
        post_request(f"{host}21002/api/space/spacename/version/1/search", search_data)
        time.sleep(1)

        # Search on node 3
        print("Search vectors with specific version id on node 3")
        post_request(f"{host}21003/api/space/spacename/version/1/search", search_data)
        time.sleep(1)

    # Step 4: Search vectors with default version on node 1
    print("Search vectors with default version on node 1")
    search_data_default = {
        "vector": [1.0, 0.1, 0.2, 0.3]
    }
    post_request(f"{host}21001/api/space/spacename/search", search_data_default)
    time.sleep(1)

    if not single_node:
        # Search with default version on node 2
        print("Search vectors with default version on node 2")
        post_request(f"{host}21002/api/space/spacename/search", search_data_default)
        time.sleep(1)

        # Search with default version on node 3
        print("Search vectors with default version on node 3")
        post_request(f"{host}21003/api/space/spacename/search", search_data_default)
        time.sleep(1)

    # Step 5: Filter search with metadata
    print("Filter search with metadata on node 1")
    search_data_with_filter = {
        "vector": [0.2, 0.3, 0.4, 0.3],
        "filter": "meta == 'first' OR meta == 'second'"
    }
    post_request(f"{host}21001/api/space/spacename/search", search_data_with_filter)
    time.sleep(1)

    if not single_node:
        # Filter search with metadata on node 2
        print("Filter search with metadata on node 2")
        post_request(f"{host}21002/api/space/spacename/search", search_data_with_filter)
        time.sleep(1)

        # Filter search with metadata on node 3
        print("Filter search with metadata on node 3")
        post_request(f"{host}21003/api/space/spacename/search", search_data_with_filter)
        time.sleep(1)

def main():
    # Argument parser for host and single-node flag
    parser = argparse.ArgumentParser(description="Test vector search in a distributed system.")
    parser.add_argument('--host', type=str, default='http://127.0.0.1:', help="Base URL of the host.")
    parser.add_argument('--single-node', action='store_true', help="Flag to run in single-node mode.")
    
    args = parser.parse_args()
    
    # Run the search test
    test_search(args.host, args.single_node)

if __name__ == "__main__":
    main()
