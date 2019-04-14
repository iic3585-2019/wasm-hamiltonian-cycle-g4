import("../crate/pkg").then(module => {
  /* Let us create the following graph
      (0)--(1)--(2)
      |   / \   |
      |  /   \  |
      | /     \ |
      (3)-------(4)
  */
  const exampleGraph = "0 1 0 1 0 1 0 1 1 1 0 1 0 0 1 1 1 0 0 1 0 1 1 1 0";

  console.log(module.search_hamiltonian(exampleGraph, 5));
});
