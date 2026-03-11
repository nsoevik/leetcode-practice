pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut parents: Vec<usize> = (0..=edges.len()).collect();
    fn find_parent(parents: &mut Vec<usize>, i: usize) -> usize {
        if parents[i] != i {
            parents[i] = find_parent(parents, parents[i]);
        }
        parents[i]
    }

    for edge in edges.iter() {
        let parent_0 = find_parent(&mut parents, edge[0] as usize);
        let parent_1 = find_parent(&mut parents, edge[1] as usize);
        if parent_0 != parent_1 {
            parents[parent_0] = parent_1;
            continue;
        }

        return edge.clone(); 
    }

    unreachable!()
}
