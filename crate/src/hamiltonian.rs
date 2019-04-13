fn is_safe(v: i32, graph: &[[bool; 5]; 5], path: &mut [i32], pos: usize) -> bool {
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

fn solve(graph: &[[bool; 5]; 5], path: &mut [i32; 5], pos: usize) -> bool {
  if pos == 5 {
    if graph[path[pos - 1] as usize][path[0] as usize] == true {
      return true;
    } else {
      return false;
    }
  }

  for v in 1..5 {
    if is_safe(v, graph, path, pos) {
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

fn ham_cycle(graph: &[[bool; 5]; 5]) -> bool {
  let mut path: [i32; 5] = [-1, -1, -1, -1, -1];

  path[0] = 0;

  if solve(graph, &mut path, 1) == false {
    println!("Solution does not exist");
    return false;
  }

  print_solution(&mut path);

  return true;
}

fn print_solution(path: &mut [i32]) {
  println!("Solution exists");

  for i in 0..5 {
    println!("{}", path[i])
  }

  // Print first vertex again to show complete cycle
  println!("{}", path[0])
}

pub fn run() {
  /* Let us create the following graph
      (0)--(1)--(2)
       |   / \   |
       |  /   \  |
       | /     \ |
      (3)-------(4)
  */
  let graph: [[bool; 5]; 5] = [
    [false, true, false, true, false],
    [true, false, true, true, true],
    [false, true, false, false, true],
    [true, true, false, false, true],
    [false, true, true, true, false],
  ];

  ham_cycle(&graph);
}
