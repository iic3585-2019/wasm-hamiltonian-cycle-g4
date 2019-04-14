import("../crate/pkg").then(module => {
  module.run();

  const exampleGraph = "0 1 0 1 0 1 0 1 1 1 0 1 0 0 1 1 1 0 0 1 0 1 1 1 0";
  console.log(module.search_hamiltonian(exampleGraph, 5));

  // console.log(module.parse_matrix("0 1 1 0", 2));
});
