import requests
import json
import time
import argparse
import os
import signal

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
    # Step 1: Write data on leader
    print("Write data on leader")
    time.sleep(1)

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

    print("Data written")
    time.sleep(1)

    # Step 2: Read on every node, including the leader
    print("Read from node 1")
    get_request(f"{host}21001/api/space/spacename")
    time.sleep(1)

    if not singlenode:
        print("Read from node 2")
        get_request(f"{host}21002/api/space/spacename")
        time.sleep(1)

        print("Read from node 3")
        get_request(f"{host}21003/api/space/spacename")
        time.sleep(1)

    # Step 3: list api
    print("Call List api")
    time.sleep(1)

    print("Read from node 1")
    get_request(f"{host}21001/api/spaces")
    time.sleep(1)

    if not singlenode:
        print("Read from node 2")
        get_request(f"{host}21002/api/spaces")
        time.sleep(1)

        print("Read from node 3")
        get_request(f"{host}21003/api/spaces")
        time.sleep(1)

    # Step 4: Update space
    update_space_data = {
        "dense": {
            "dimension": 1234,
            "metric": "l2",
            "hnsw_config": {
                "m": 64,
                "ef_construct": 55
            }
        }
    }

    post_request(f"{host}21001/api/space/spacename", update_space_data)
    print("Read from node 1")
    get_request(f"{host}21001/api/space/spacename")
    time.sleep(1)

def main():
    # Argument parser for host, singlenode, and PID1
    parser = argparse.ArgumentParser(description="Test Space API interactions.")
    parser.add_argument('--host', type=str, default='127.0.0.1', help="Host for the API requests (default: 127.0.0.1).")
    parser.add_argument('--single', action='store_true', help="Flag to indicate single-node mode.")

    args = parser.parse_args()

    # Run the test with provided arguments
    run_test(f"http://{args.host}:", args.single)

if __name__ == "__main__":
    main()
