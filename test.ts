async function doLoop(): Promise<void> {
  const uuid_cluster = await import("uuid-cluster");
  uuid_cluster.leak_test();
  console.warn("done");
}

(window as any)["doLoop"] = doLoop;
