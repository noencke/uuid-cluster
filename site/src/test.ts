async function doLoop(): Promise<void> {
  const uuid_cluster = await import("uuid-cluster");
  const compressor = new uuid_cluster.ClusterCompressor();
  compressor.add(0, 0, 0);
  for (let i = 1; i <= 5000; i++) {
    const thingy = compressor.compress(6);

    if (i % 1000 === 0) {
      console.warn(i);
    }
  }
  compressor.free();
}

(window as any)["doLoop"] = doLoop;
