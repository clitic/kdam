import tqdm

pb = tqdm.tqdm(total=100000000);

for i in range(100000000):
    if i % 10000000 == 0:
        pb.write(f"reached at {pb.format_sizeof(i)}")
    pb.update(1)
