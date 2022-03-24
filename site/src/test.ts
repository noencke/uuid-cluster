async function doLoop(): Promise<void> {
  const uuid_cluster = await import("uuid-cluster");
  for (let i = 1; i <= 1000; i++) {
    uuid_cluster.leak_test();
  }
  console.warn("done");
}

(window as any)["doLoop"] = doLoop;
