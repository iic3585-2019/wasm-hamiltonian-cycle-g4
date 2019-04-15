import("../crate/pkg").then(module => {
  /* Let us create the following graph
      (0)--(1)--(2)
      |   / \   |
      |  /   \  |
      | /     \ |
      (3)-------(4)
  */
  const solveButton = document.getElementById("solve");
  
  solveButton.addEventListener("click", _ => {
    let nodes = document.getElementById("nodes").value;
    let edges = document.getElementById("edges").value;
    console.log(module.search_hamiltonian(parseInt(nodes,10), edges))
  });
  // const exampleGraph = "0 1 0 1 0 1 0 1 1 1 0 1 0 0 1 1 1 0 0 1 0 1 1 1 0";

  // console.log(module.search_hamiltonian(exampleGraph, 5));
});

