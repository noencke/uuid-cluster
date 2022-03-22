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
