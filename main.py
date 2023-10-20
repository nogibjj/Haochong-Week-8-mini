import pandas as pd
import matplotlib.pyplot as plt
import time
from mylib.lib import get_median

# Record the start time
start_time = time.time()

if __name__ == "__main__":
    example_csv = "25ktopomapseriesindex.csv"
    df = pd.read_csv(example_csv)
    print(get_median(df))
    end_time = time.time()

# Calculate the elapsed time
elapsed_time = end_time - start_time

print(f"Elapsed time: {elapsed_time:.4f} seconds")
