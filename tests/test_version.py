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

# Helper function to send GET requests
def get_request(url):
    try:
        response = requests.get(url)
        print(f"GET {url} -> Status: {response.status_code}")
        if response.status_code == 200:
            print(f"Response: {response.json()}")
        else:
            print(f"Error: {response.text}")
    except requests.exceptions.RequestException as e:
        print(f"An error occurred during GET request to {url}: {e}")

def test_version(host, single_node):
    # Step 1: Create space 'spacename' on leader
    print("Create space 'spacename' on leader")
    create_space_data = {
        "name": "spacename",
        "dimension": 128,
        "metric": "cosine",
        "hnsw_config": {
            "ef_construct": 123
        },
        "quantization_config": {
            "scalar": {
                "type": "int8",
                "quantile": 0.99,
                "always_ram": True
            }
        },
        "dense": {
            "dimension": 1536,
            "metric": "Cosine",
            "hnsw_config": {
                "m": 32,
                "ef_construct": 123
            },
            "quantization_config": {
                "scalar": {
                    "type": "int8",
                    "quantile": 0.8
                }
            }
        },
        "sparse": {
            "metric": "Cosine"
        },
        "indexes": {
            "index1": {
                "dimension": 4,
                "metric": "Cosine",
                "hnsw_config": {
                    "m": 20
                },
                "quantization_config": {
                    "scalar": {
                        "type": "int8",
                        "quantile": 0.6
                    }
                }
            },
            "index2": {
                "dimension": 4,
                "metric": "Cosine",
                "hnsw_config": {
                    "m": 20
                },
                "quantization_config": {
                    "scalar": {
                        "type": "int8",
                        "quantile": 0.6
                    }
                }
            }
        }
    }
    post_request(f"{host}21001/api/space", create_space_data)
    time.sleep(1)

    print("Space 'spacename' created")
    time.sleep(1)

    # Step 2: Create version on leader
    print("Create version on leader")
    create_version_data = {
        "name": "version1",
        "description": "Initial version",
        "tag": "v1.0",
        "is_default": True
    }
    post_request(f"{host}21001/api/space/spacename/version", create_version_data)
    time.sleep(1)

    print("Version created")
    time.sleep(1)

    # Step 3: Get version1 on node 1
    print("Get version1 node 1")
    get_request(f"{host}21001/api/space/spacename/version/version1/by-name")
    time.sleep(1)

    if not single_node:
        # Get version1 on node 2
        print("Get version1 node 2")
        get_request(f"{host}21002/api/space/spacename/version/version1/by-name")
        time.sleep(1)

        # Get version1 on node 3
        print("Get version1 node 3")
        get_request(f"{host}21003/api/space/spacename/version/version1/by-name")
        time.sleep(1)

    # Step 4: Get default version on node 1
    print("Get default version node 1")
    get_request(f"{host}21001/api/space/spacename/version/default")
    time.sleep(1)

    if not single_node:
        # Get default version on node 2
        print("Get default version node 2")
        get_request(f"{host}21002/api/space/spacename/version/default")
        time.sleep(1)

        # Get default version on node 3
        print("Get default version node 3")
        get_request(f"{host}21003/api/space/spacename/version/default")
        time.sleep(1)

    # Step 5: Get version list on node 1
    print("Get version list node 1")
    get_request(f"{host}21001/api/space/spacename/version/list")
    time.sleep(1)

    if not single_node:
        # Get version list on node 2
        print("Get version list node 2")
        get_request(f"{host}21002/api/space/spacename/version/list")
        time.sleep(1)

        # Get version list on node 3
        print("Get version list node 3")
        get_request(f"{host}21003/api/space/spacename/version/list")
        time.sleep(1)

def main():
    # Argument parser for host and single-node flag
    parser = argparse.ArgumentParser(description="Test version creation and retrieval in a distributed system.")
    parser.add_argument('--host', type=str, default='http://127.0.0.1:', help="Base URL of the host.")
    parser.add_argument('--single-node', action='store_true', help="Flag to run in single-node mode.")
    
    args = parser.parse_args()
    
    # Run the version test
    test_version(args.host, args.single_node)

if __name__ == "__main__":
    main()
