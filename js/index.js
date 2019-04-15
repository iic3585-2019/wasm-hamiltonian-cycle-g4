import("../crate/pkg").then(module => {
  const d3 = require("d3");
  /* Let us create the following graph
      (0)--(1)--(2)
      |   / \   |
      |  /   \  |
      | /     \ |
      (3)-------(4)
  */
  const solveButton = document.getElementById("solve");

  let svg = d3.select("svg"),
  width = +svg.attr("width"),
  height = +svg.attr("height");
  
  solveButton.addEventListener("click", _ => {
    d3.selectAll("svg > *").remove();
    const nodes = parseInt(document.getElementById("nodes").value, 10);
    let nodes_data = [...Array(nodes).keys()].map((x) => { return {id: x.toString()} });

    const edges = document.getElementById("edges").value;
    let edges_data = edges.split("\n").map(x => x.split(" ")).filter(x => x.length > 1).map((a) => { return {source: a[0], target: a[1]}});

    var simulation = d3.forceSimulation().nodes(nodes_data);
    simulation
      .force("charge_force", d3.forceManyBody())
      .force("center_force", d3.forceCenter(width / 2, height / 2));

    // var node = svg.append("g")
    //   .attr("class", "nodes")
    //   .selectAll("circle")
    //   .data(nodes_data)
    //   .enter()
    //   .append("circle")
    //   .attr("r", 7)
    //   .attr("fill", "red");

    var node = svg.append("g")
      .attr("class", "nodes")
    .selectAll("g")
    .data(nodes_data)
    .enter().append("g")

    var circles = node.append("circle")
      .attr("r", 7)
      .attr("fill", "red");
    
    var labels = node.append("text")
      .text(function(d) {
        return d.id;
      })
      .attr('x', 6)
      .attr('y', 6);

    node.append("title")
      .text(function(d) { return d.id; });

    var drag_handler = d3.drag()
      .on("start", drag_start)
      .on("drag", drag_drag)
      .on("end", drag_end);	

    drag_handler(node);

    

    var link_force =  d3.forceLink(edges_data).id(function(d) { return d.id; }).distance(100)
    simulation.force("links",link_force)

    var link = svg.append("g")
      .attr("class", "links")
      .selectAll("line")
      .data(edges_data)
      .enter()
      .append("line")
      .attr("stroke-width", 2);

    
    simulation.on("tick", tickActions );

    function tickActions() {
        //update circle positions each tick of the simulation 
        // node
        //     .attr("cx", function(d) { return d.x; })
        //     .attr("cy", function(d) { return d.y; });
        node
        .attr("transform", function(d) {
          return "translate(" + Math.max(7, Math.min(width - 7, d.x)) + "," + Math.max(7, Math.min(height - 7, d.y)) + ")";
        })
            
        //update link positions 
        //simply tells one end of the line to follow one node around
        //and the other end of the line to follow the other node around
        link
            .attr("x1", function(d) { return d.source.x; })
            .attr("y1", function(d) { return d.source.y; })
            .attr("x2", function(d) { return d.target.x; })
            .attr("y2", function(d) { return d.target.y; });
     
      } 

      function drag_start(d) {
        if (!d3.event.active) simulation.alphaTarget(0.3).restart();
          d.fx = d.x;
          d.fy = d.y;
       }
       
       function drag_drag(d) {
         d.fx = d3.event.x;
         d.fy = d3.event.y;
       }
       
       
       function drag_end(d) {
         if (!d3.event.active) simulation.alphaTarget(0);
         d.fx = null;
         d.fy = null;
       }
    

    const result =  module.search_hamiltonian(nodes, edges);
    let output = document.getElementById("result");
    if(result[0] != -1) {
      let s = result.reduce((a, b) => a+' -> '+b);
      output.innerHTML = s+' -> '+result[0];
    } else {
      output.innerHTML = "No tiene camino hamiltoneano";
    }
  });
});

