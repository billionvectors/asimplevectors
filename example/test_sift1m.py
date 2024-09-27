import os
import shutil
import requests
import json
import time
import argparse
import h5py

cache_file = 'benchmark/sift1m/cache/snapshot-20240101.zip'
dataset_file = 'benchmark/dataset/sift-128-euclidean.hdf5'

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

def upsert_vectors(host, vectors_batch, start_id):
    upsert_vectors_data = {
        "vectors": [
            {"id": start_id + i, "data": [round(float(val), 5) for val in vector]}
            for i, vector in enumerate(vectors_batch)
        ]
    }
    response = requests.post(f"{host}21001/api/space/spacename/vector", json=upsert_vectors_data)
    if response.status_code != 200:
        print(f"Error upserting vectors: {response.text}")

# Helper function to download a file
def download_file(url, dest_path):
    print(f"Downloading {url} to {dest_path}")
    response = requests.get(url, stream=True)
    if response.status_code == 200:
        os.makedirs(os.path.dirname(dest_path), exist_ok=True)
        with open(dest_path, 'wb') as f:
            for chunk in response.iter_content(chunk_size=1024):
                if chunk:
                    f.write(chunk)
        print(f"Downloaded {url} successfully.")
    else:
        print(f"Failed to download {url}. Status code: {response.status_code}")

# Helper function to copy a file
def copy_file(src, dest):
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    shutil.copy(src, dest)
    print(f"Copied {src} to {dest}")

# Function to generate cache if not already present
def generate_cache(host):
    print("Cache is not present. Generating cache... it takes around 1 hour")

    create_space_data = {
        "name": "spacename",
        "dimension": 128,
        "metric": "L2",
        "hnsw_config": {
            "M": 96,
            "ef_construct": 500
        }
    }
    post_request(f"{host}21001/api/space", create_space_data)

    # Check if the file exists; if not, download it
    if not os.path.exists(dataset_file):
        download_file('http://ann-benchmarks.com/sift-128-euclidean.hdf5', dataset_file)

    # Open the HDF5 file
    with h5py.File(dataset_file, 'r') as hdf_file:
        distances = hdf_file['distances'][:]
        neighbors = hdf_file['neighbors'][:]
        test = hdf_file['test'][:]
        train = hdf_file['train'][:]

        print(f'Distances shape: {distances.shape}')
        print(f'Neighbors shape: {neighbors.shape}')
        print(f'Test shape: {test.shape}')
        print(f'Train shape: {train.shape}')

        batch_size = 500
        start_time = time.time()
        start_lap_time = time.time()

        for i in range(0, len(train), batch_size):
            batch = train[i:i + batch_size]
            start_id = i + 1
            upsert_vectors(host, batch, start_id)

            if start_id % 10000 == 1:
                elapsed_time = time.time() - start_lap_time
                print(f"Upserted {start_id - 1} vectors so far, took {elapsed_time:.2f} seconds for the last 10,000 vectors.")
                start_lap_time = time.time()

        elapsed_time = time.time() - start_time
        print(f"Upserting process completed in {elapsed_time:.2f} seconds")

    print("Request snapshots... it takes around 30 minutes")
    create_snapshot_data = {
        "spacename": 1
    }
    post_request(f"{host}21001/api/snapshot", create_snapshot_data)

    # Look for the latest snapshot zip file
    snapshot_dir = 'data/data1/snapshot/'
    snapshot_files = [f for f in os.listdir(snapshot_dir) if f.endswith('.zip')]
    if snapshot_files:
        latest_snapshot = max(snapshot_files, key=lambda f: os.path.getctime(os.path.join(snapshot_dir, f)))
        src_snapshot = os.path.join(snapshot_dir, latest_snapshot)
        dest_snapshot = cache_file
        
        # Copy the snapshot to the cache location
        copy_file(src_snapshot, dest_snapshot)
        print(f"Snapshot copied to cache: {dest_snapshot}")
    else:
        print("No snapshot files found.")
    print("done")

def restore_cache(host):
    snapshot_dirs = [
        'data/data1/snapshot/',
        'data/data2/snapshot/',
        'data/data3/snapshot/'
    ]
    
    for snapshot_dir in snapshot_dirs:
        os.makedirs(snapshot_dir, exist_ok=True)
        snapshot_destination = os.path.join(snapshot_dir, os.path.basename(cache_file))
        copy_file(cache_file, snapshot_destination)
        print(f"Copied {cache_file} to {snapshot_destination}")
    
    print("Restore snapshots... it takes around 10 minutes")
    restore_snapshot_data = {}
    post_request(f"{host}21001/api/snapshot/20240101/restore", restore_snapshot_data)
    print("Restore done!")

def benchmark(host):
    # Check if the file exists; if not, download it
    if not os.path.exists(dataset_file):
        download_file('http://ann-benchmarks.com/sift-128-euclidean.hdf5', dataset_file)

    print("run test")
    
    # Open the HDF5 file
    with h5py.File(dataset_file, 'r') as hdf_file:
        neighbors = hdf_file['neighbors'][:]
        test = hdf_file['test'][:]

        # To track the correct first label (p@0)
        correct_predictions = 0
        total_queries = len(test)

        # Variable to track total query execution time
        total_query_time = 0

        # Iterate over the test vectors and send them for searching
        for i, test_vector in enumerate(test):
            search_data = {
                "vector": test_vector.tolist()  # Convert to a list for JSON serialization
            }

            # Measure time before and after the request
            start_query_time = time.time()
            response = requests.post(f"{host}21001/api/space/spacename/version/1/search", json=search_data)
            end_query_time = time.time()

            # Add the duration of this query to the total query time
            query_duration = end_query_time - start_query_time
            total_query_time += query_duration

            if response.status_code == 200:
                try:
                    # First parse the outer JSON string
                    search_results = json.loads(response.text)

                    # Ensure search_results[0] is a dictionary and contains 'label'
                    if isinstance(search_results[0], dict) and 'label' in search_results[0]:
                        expected_label = int(neighbors[i][0])
                        actual_label = int(search_results[0]['label']) - 1
                        
                        if actual_label == expected_label:
                            correct_predictions += 1
                    else:
                        print(f"Unexpected response format: {search_results[0]}")
                except ValueError as e:
                    print(f"Failed to decode JSON response for query {i + 1}: {e}")
            else:
                print(f"Query {i + 1} failed with status code: {response.status_code}")

        # Calculate the p@0 accuracy
        accuracy = correct_predictions / total_queries

        # Calculate queries per second (QPS)
        qps = total_queries / total_query_time

        # Print the results
        print(f"Accuracy (p@0): {accuracy * 100:.2f}%")
        print(f"Total query time for {total_queries} queries: {total_query_time:.2f} seconds")
        print(f"Queries per second: {qps:.2f}")


# Function to run the test and handle cache
def run_test(host, singlenode):
    # Check if the cache file exists
    if not os.path.exists(cache_file):
        # Cache doesn't exist, so generate cache
        generate_cache(host)
    else:
        restore_cache(host)

    benchmark(host)

# Main function to handle argument parsing and invoke the test
def main():
    parser = argparse.ArgumentParser(description="Run API requests with a specified host and singlenode setting.")
    parser.add_argument('--host', type=str, default='127.0.0.1', help="The host to send requests to (default: 127.0.0.1).")
    parser.add_argument('--single', action='store_true', help="Flag to run in single-node mode (default: False).")

    args = parser.parse_args()
    
    run_test(f"http://{args.host}:", args.single)

if __name__ == "__main__":
    main()
