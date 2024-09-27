import requests
import time
import argparse

# Helper function to send GET requests
def get_request(url):
    try:
        response = requests.get(url)
        print(f"GET {url} -> Status: {response.status_code}")
        if response.status_code in [200, 201]:
            print(f"Response: {response.text}")
        else:
            print(f"Error: {response.text}")
    except requests.exceptions.RequestException as e:
        print(f"An error occurred during GET request to {url}: {e}")

# Helper function to send POST requests
def post_request(url, json_data):
    try:
        headers = {'Content-Type': 'application/json'}
        response = requests.post(url, headers=headers, json=json_data)
        print(f"POST {url} -> Status: {response.status_code}")
        if response.status_code in [200, 201]:
            print(f"Response: {response.text}")
        else:
            print(f"Error: {response.text}")
    except requests.exceptions.RequestException as e:
        print(f"An error occurred during POST request to {url}: {e}")

# Helper function to send DELETE requests
def delete_request(url):
    try:
        response = requests.delete(url)
        print(f"DELETE {url} -> Status: {response.status_code}")
        if response.status_code in [200, 201]:
            print(f"Response: {response.text}")
        else:
            print(f"Error: {response.text}")
    except requests.exceptions.RequestException as e:
        print(f"An error occurred during DELETE request to {url}: {e}")


# Test function to create a space, upsert vectors, and search vectors
def test_storage(host, single_node):
    # Step 1: Create space 'spacename' on leader
    print("Create key test")
    create_key_data = {
        "hihi": "hellohello"
    }

    post_request(f"{host}21001/api/space/spacename/key/test", create_key_data)
    time.sleep(1)

    print("Read test on node 1")
    get_request(f"{host}21001/api/space/spacename/key/test")

    if not single_node:
        print("Read test on node 2")
        get_request(f"{host}21002/api/space/spacename/key/test")
        time.sleep(1)

        print("Read test on node 3")
        get_request(f"{host}21003/api/space/spacename/key/test")
        time.sleep(1)

    print("Create another key test2")
    create_key_data2 = {
        "test2": "test2"
    }

    post_request(f"{host}21001/api/space/spacename/key/test2", create_key_data2)
    time.sleep(1)

    print("Read test2 on node 1")
    get_request(f"{host}21001/api/space/spacename/key/test2")

    if not single_node:
        print("Read test2 on node 2")
        get_request(f"{host}21002/api/space/spacename/key/test2")
        time.sleep(1)

        print("Read test2 on node 3")
        get_request(f"{host}21003/api/space/spacename/key/test2")
        time.sleep(1)

    print("key lists on node1")
    get_request(f"{host}21001/api/space/spacename/keys")

    if not single_node:
        print("key lists on node 2")
        get_request(f"{host}21002/api/space/spacename/keys")
        time.sleep(1)

        print("key lists on node 3")
        get_request(f"{host}21003/api/space/spacename/keys")
        time.sleep(1)

    print("Delete key test2")
    delete_request(f"{host}21001/api/space/spacename/key/test2")
    time.sleep(1)

    print("key lists on node1")
    get_request(f"{host}21001/api/space/spacename/keys")

    if not single_node:
        print("key lists on node 2")
        get_request(f"{host}21002/api/space/spacename/keys")
        time.sleep(1)

        print("key lists on node 3")
        get_request(f"{host}21003/api/space/spacename/keys")
        time.sleep(1)
        
    print("Read test2 on node 1")
    get_request(f"{host}21001/api/space/spacename/key/test2")

    if not single_node:
        print("Read test2 on node 2")
        get_request(f"{host}21002/api/space/spacename/key/test2")
        time.sleep(1)

        print("Read test2 on node 3")
        get_request(f"{host}21003/api/space/spacename/key/test2")
        time.sleep(1)

def main():
    # Argument parser for host and single-node flag
    parser = argparse.ArgumentParser(description="Test kv storage in a distributed system.")
    parser.add_argument('--host', type=str, default='http://127.0.0.1:', help="Base URL of the host.")
    parser.add_argument('--single-node', action='store_true', help="Flag to run in single-node mode.")
    
    args = parser.parse_args()
    
    # Run the search test
    test_storage(args.host, args.single_node)

if __name__ == "__main__":
    main()
