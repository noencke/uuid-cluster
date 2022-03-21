import("./node_modules/uuid-cluster/uuid_cluster.js").then((js) => {
  const compressor = new js.ClusterCompressor();
  compressor.add(0, 0, 0);
  const compressed = compressor.compress();
  compressor.free();
  alert(compressed);
});
