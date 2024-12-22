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

# Test function for rerank operation
def test_rerank(host, single_node):
    # Step 1: Create space 'rerank_space'
    print("Create space 'rerank_space'")
    create_space_data = {
        "name": "rerank_space",
        "dimension": 4,
        "metric": "L2",
        "hnsw_config": {
            "M": 16,
            "ef_construct": 100
        }
    }
    post_request(f"{host}21001/api/space", create_space_data)
    time.sleep(1)
    
    print("Space 'rerank_space' created")
    time.sleep(1)

    # Step 2: Upsert vectors with associated documents and tokens
    print("Upsert vectors to 'rerank_space'")
    upsert_vectors_data = {
        "vectors": [
            {
                "id": 1,
                "data": [0.25, 0.45, 0.75, 0.85],
                "metadata": {"category": "A"},
                "doc": "This is a test document about vectors.",
                "doc_tokens": ["test", "document", "vectors"]
            },
            {
                "id": 2,
                "data": [0.20, 0.62, 0.77, 0.75],
                "metadata": {"category": "B"},
                "doc": "Another document with different content.",
                "doc_tokens": ["another", "document", "different", "content"]
            }
        ]
    }
    post_request(f"{host}21001/api/space/rerank_space/vector", upsert_vectors_data)
    time.sleep(1)

    # Step 3: Perform rerank operation
    print("Perform rerank operation on 'rerank_space'")
    rerank_data = {
        "vector": [0.25, 0.45, 0.75, 0.85],
        "tokens": ["test", "vectors"]
    }
    post_request(f"{host}21001/api/space/rerank_space/rerank", rerank_data)
    time.sleep(1)

    if not single_node:
        # Rerank on node 2
        print("Perform rerank operation on node 2")
        post_request(f"{host}21002/api/space/rerank_space/rerank", rerank_data)
        time.sleep(1)

        # Rerank on node 3
        print("Perform rerank operation on node 3")
        post_request(f"{host}21003/api/space/rerank_space/rerank", rerank_data)
        time.sleep(1)

def main():
    # Argument parser for host and single-node flag
    parser = argparse.ArgumentParser(description="Test rerank operation in a distributed system.")
    parser.add_argument('--host', type=str, default='http://127.0.0.1:', help="Base URL of the host.")
    parser.add_argument('--single-node', action='store_true', help="Flag to run in single-node mode.")
    
    args = parser.parse_args()
    
    # Run the rerank test
    test_rerank(args.host, args.single_node)

if __name__ == "__main__":
    main()
