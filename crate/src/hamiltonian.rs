// Reference: https://www.geeksforgeeks.org/hamiltonian-cycle-backtracking-6/


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

fn solve(graph: &mut Vec<Vec<bool>>, path: &mut Vec<i32>, pos: usize) -> bool {
  if pos == graph.len() {
    if graph[path[pos - 1] as usize][path[0] as usize] == true {
      return true;
    } else {
      return false;
    }
  }

  for v in 1..graph.len() {
    if is_safe(v as i32, graph, path, pos) {
      path[pos] = v as i32;

      if solve(graph, path, pos + 1) {
        return true;
      }
      // Else, backtrack
      path[pos] = -1;
    }
  }

  return false;
}

fn ham_cycle(graph: &mut Vec<Vec<bool>>) -> Vec<i32> {
  let mut path = vec![-1; graph.len()];

  path[0] = 0;

  if solve(graph, &mut path, 1) == false {
    println!("Solution does not exist");
    path[0] = -1;
  }

  // print_solution(&mut path, size);

  return path;
}

fn print_solution(path: &mut Vec<i32>) {
  println!("Solution exists");

  for i in 0..path.len() {
    println!("{}", path[i])
  }

  // Print first vertex again to show complete cycle
  println!("{}", path[0])
}

pub fn run(graph: &mut Vec<Vec<bool>>) -> Vec<i32> {
  /* Let us create the following graph
      (0)--(1)--(2)
       |   / \   |
       |  /   \  |
       | /     \ |
      (3)-------(4)
  */
  // let mut graph: Vec<Vec<bool>> = vec![
  //   vec![false, true, false, true, false],
  //   vec![true, false, true, true, true],
  //   vec![false, true, false, false, true],
  //   vec![true, true, false, false, true],
  //   vec![false, true, true, true, false],
  // ];


  let out: Vec<i32> = ham_cycle(graph);
  return out;
}
