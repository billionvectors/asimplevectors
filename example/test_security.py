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

def extract_token(url):
    try:
        response = requests.get(url)
        if response.status_code == 200:
            try:
                token_data = json.loads(response.text)

                # Ensure token_data is a list and not empty
                if isinstance(token_data, list) and len(token_data) > 0:
                    token = token_data[0].get('token')

                    if token:
                        print(f"Extracted token: {token}")
                        return token
                    else:
                        print("Token key not found in the first element of the response.")
                        return None
                else:
                    print("Token data is not a valid list or the list is empty.")
                    return None
            except json.JSONDecodeError:
                print("Failed to decode JSON response.")
                print(f"Response content (raw): {response.text}")
                return None
        else:
            print(f"Failed to get tokens from {url}. Status code: {response.status_code}")
            return None
    except requests.exceptions.RequestException as e:
        print(f"An error occurred while extracting token from {url}: {e}")
        return None

def run_test(host, singlenode):
    # Step 1: Try to create space without permission
    print("Create space 'spacename' on leader without permission")
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
    print("Space 'spacename' not created")

    # Step 2: Create a new RBAC token
    print("Create a new RBAC token")
    token_data = {
        "user_id": 0,
        "system": 2,
        "space": 2,
        "version": 2,
        "vector": 2,
        "snapshot": 2
    }
    post_request(f"{host}21001/api/security/tokens", token_data)
    time.sleep(1)

    # Step 3: List all RBAC tokens on node 1 and extract token
    print("List all RBAC tokens on node 1")
    token_url = f"{host}21001/api/security/tokens"
    token = extract_token(token_url)
    time.sleep(1)

    if not singlenode:
        # Step 4: List all RBAC tokens on node 2 and node 3 if not single-node
        print("List all RBAC tokens on node 2")
        get_request(f"{host}21002/api/security/tokens")
        time.sleep(1)

        print("List all RBAC tokens on node 3")
        get_request(f"{host}21003/api/security/tokens")
        time.sleep(1)

    # Step 5: Create space with valid token
    print("Create space 'spacename' on leader with valid token")
    post_request(f"{host}21001/api/space", create_space_data, token=token)
    time.sleep(1)
    print("Space 'spacename' created")

    # Step 6: Upsert vectors without specifying version without token
    print("Upsert vectors to 'spacename' without specifying version without token")
    upsert_data = {
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
    }
    post_request(f"{host}21001/api/space/spacename/vector", upsert_data, token=token)
    time.sleep(1)

    # Step 7: Search vectors with specific version id on node 1 without token
    print("Search vectors with specific version id on node 1 without token")
    search_data = {
        "vector": [0.2, 0.3, 0.4, 0.3]
    }
    post_request(f"{host}21001/api/space/spacename/version/1/search", search_data, token=token)
    time.sleep(1)

def main():
    # Argument parser for host and singlenode
    parser = argparse.ArgumentParser(description="Run security-related tests on the API with RBAC.")
    parser.add_argument('--host', type=str, default='127.0.0.1', help="The host to send requests to (default: 127.0.0.1).")
    parser.add_argument('--single', action='store_true', help="Flag to run in single-node mode (default: False).")
    
    args = parser.parse_args()
    
    # Run the test with provided arguments
    run_test(f"http://{args.host}:", args.single)

if __name__ == "__main__":
    main()
