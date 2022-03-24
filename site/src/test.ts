import * as uuid_cluster from "uuid-cluster";
// const uuid_cluster = await import("uuid-cluster");
export {};

// import("uuid-cluster").then((uuid_cluster) => {
const compressor = new uuid_cluster.ClusterCompressor();
compressor.add(0, 0, 0);
const compressed = compressor.compress(6);
compressor.free();
console.warn(compressed);
// });

async function doLoop(): Promise<void> {
  const uuid_cluster = await import("uuid-cluster");
  const compressor = new uuid_cluster.ClusterCompressor();
  compressor.add(0, 0, 0);
  for (let i = 1; i <= 1000; i++) {
    const compressed = compressor.compress(6);
  }
  compressor.free();
  console.warn("done");
}

(window as any)["doLoop"] = doLoop;
