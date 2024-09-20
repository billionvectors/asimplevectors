import requests
import json
import time
import argparse

# Helper function to send POST requests
def post_request(url, json_data, token=None):
    headers = {'Content-Type': 'application/json'}
    if token:
        headers['Authorization'] = f'Bearer {token}'
    
    try:
        response = requests.post(url, headers=headers, data=json.dumps(json_data))
        if response.status_code == 200:
            print(f"Request to {url} successful.")
            print(f"Response: {response.text}")
        elif response.status_code == 201:
            print(f"Request to {url} created successfully (201).")
            print(f"Response: {response.text}")
        elif response.status_code == 403:
            print(f"Request to {url} forbidden (403). You don't have the necessary permissions.")
        else:
            print(f"Request to {url} failed with status code: {response.status_code}")
            print(f"Error message: {response.text}")
    except requests.exceptions.RequestException as e:
        print(f"An error occurred while sending a POST request to {url}: {e}")

# Helper function to send GET requests
def get_request(url, token=None):
    headers = {}
    if token:
        headers['Authorization'] = f'Bearer {token}'
    
    try:
        response = requests.get(url, headers=headers)
        if response.status_code == 200:
            print(f"Request to {url} successful.")
            print(f"Response: {response.text}")
        elif response.status_code == 403:
            print(f"Request to {url} forbidden (403). You don't have the necessary permissions.")
        else:
            print(f"Request to {url} failed with status code: {response.status_code}")
            print(f"Error message: {response.text}")
    except requests.exceptions.RequestException as e:
        print(f"An error occurred while sending a GET request to {url}: {e}")

def run_test(host, singlenode):
    # Step 1: Create space 'spacename' on leader
    print("Create space 'spacename' on leader")
    time.sleep(1)
    
    create_space_data = {
        "name": "spacename",
        "dimension": 4,
        "metric": "L2",
        "hnsw_config": {
            "M": 16,
            "ef_construct": 100
        }
    }
    
    post_request(f"{host}21001/api/space", create_space_data)
    time.sleep(1)

    print("Space 'spacename' created")
    time.sleep(1)

    # Step 2: Upsert vectors without specifying version
    print("Upsert vectors to 'spacename' without specifying version")
    time.sleep(1)
    
    upsert_data_1 = {
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
            }
        ]
    }
    
    post_request(f"{host}21001/api/space/spacename/vector", upsert_data_1)
    time.sleep(1)

    print("Vectors upserted to 'spacename'")
    time.sleep(1)

    # Step 3: Get vectors by version ID on node 1 (default version)
    print("Get vectors by version ID on node 1")
    get_request(f"{host}21001/api/space/spacename/version/default/vectors")
    time.sleep(1)

    # Step 4: Upsert vectors to 'spacename' with specific version ID
    print("Upsert vectors to 'spacename' with specific version ID")
    time.sleep(1)
    
    upsert_data_2 = {
        "vectors": [
            {
                "id": 3,
                "data": [0.9, 0.8, 0.7, 0.6],
                "metadata": {"label": "third"}
            }
        ]
    }
    
    post_request(f"{host}21001/api/space/spacename/version/1/vector", upsert_data_2)
    time.sleep(1)

    print("Vectors upserted to 'spacename' with specific version ID")
    time.sleep(1)

    # Step 5: Get vectors by version ID 1 on node 1
    print("Get vectors by version ID 1 on node 1")
    get_request(f"{host}21001/api/space/spacename/version/1/vectors")
    time.sleep(1)

    # Step 6: Get vectors by version ID 1 on other nodes if not a single node
    if not singlenode:
        print("Get vectors by version ID 1 on node 2")
        get_request(f"{host}21002/api/space/spacename/version/1/vectors")
        time.sleep(1)

        print("Get vectors by version ID 1 on node 3")
        get_request(f"{host}21003/api/space/spacename/version/1/vectors")
        time.sleep(1)

def main():
    # Argument parser for host and singlenode
    parser = argparse.ArgumentParser(description="Run vector operations on the specified host and node settings.")
    parser.add_argument('--host', type=str, default='127.0.0.1', help="The host for API requests (default: 127.0.0.1).")
    parser.add_argument('--single', action='store_true', help="Flag to run in single-node mode.")
    
    args = parser.parse_args()
    
    # Run the test with provided arguments
    run_test(f"http://{args.host}:", args.single)

if __name__ == "__main__":
    main()
