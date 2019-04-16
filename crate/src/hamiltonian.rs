// Reference: https://www.geeksforgeeks.org/hamiltonian-cycle-backtracking-6/

// Check if a given node has connection with last node added in the path and it is not already in that path
fn is_safe(v: i32, graph: &mut Vec<Vec<bool>>, path: &mut Vec<i32>, pos: usize) -> bool {
  if graph[path[pos - 1] as usize][v as usize] == false {
    return false;
  }

  for i in 0..pos {
    if path[i] == v {
      return false;
    }
  }

  return true;
}

// Main backtracking algorithm
fn solve(graph: &mut Vec<Vec<bool>>, path: &mut Vec<i32>, pos: usize) -> bool {
  // Base case: we reached all nodes
  if pos == graph.len() {
    // If there is connection between last node of path and first node
    if graph[path[pos - 1] as usize][path[0] as usize] == true {
      return true;
    } else {
      return false;
    }
  }

  // Traverse all nodes, except the start node
  for v in 1..graph.len() {
    // If there is connection with last node added and this node is not already in the path
    if is_safe(v as i32, graph, path, pos) {
      path[pos] = v as i32;

      // Explore further in this branch
      if solve(graph, path, pos + 1) {
        return true;
      }
      // Else, backtrack
      path[pos] = -1;
    }
  }

  // No solution found
  return false;
}

fn ham_cycle(graph: &mut Vec<Vec<bool>>) -> Vec<i32> {
  let mut path = vec![-1; graph.len()];

  path[0] = 0;

  if solve(graph, &mut path, 1) == false {
    path[0] = -1;
  }

  return path;
}

pub fn run(graph: &mut Vec<Vec<bool>>) -> Vec<i32> {
  let out: Vec<i32> = ham_cycle(graph);
  return out;
}
