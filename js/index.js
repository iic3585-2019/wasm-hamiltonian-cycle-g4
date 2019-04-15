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
    const nodes = document.getElementById("nodes").value;
    const edges = document.getElementById("edges").value;
    const result =  module.search_hamiltonian(parseInt(nodes,10), edges);
    let output = document.getElementById("result");
    if(result[0] != -1) {
      let s = result.reduce((a, b) => a+' -> '+b);
      output.innerHTML = s+' -> '+result[0];
    } else {
      output.innerHTML = "No tiene camino hamiltoneano";
    }
  });
});

