use crate::util::{Vector3f};
use cgmath::{MetricSpace};

//TODO: Implement the OCTREE! :p
// this is currently a quadtree

const MIN_POINTS: usize = 5;

pub trait HasPosition {
    fn position(&self) -> Vector3f;
}

// The node only stores the index of the item
struct Node {
    items: Vec<usize>,
    children: Vec<Node>,
}

pub struct Octree<'a, T> {
    root: Node,
    width: f32,
    height: f32,
    items: &'a [T],
}

impl<'a, T> Octree<'a, T> where T: HasPosition + Clone {
    pub fn new(width: f32, height: f32, items: &'a [T]) -> Self {
        let tl = Vector3f::new(0.0, 0.0, 0.0);
        let br = Vector3f::new(width, height, 0.0);

        let mut indices: Vec<usize> = Vec::new();
        for i in 0..items.len() {
            indices.push(i);
        }

        let root = Octree::<T>::construct_tree(tl, br, items, &indices);
        Octree{
            root: root,
            width: width,
            height: height,
            items: items,
        }
    }

    pub fn nearest_neighbours_indices(&self, i: usize, r: f32) -> Vec<usize> {
        let tl = Vector3f::new(0.0, 0.0, 0.0);
        let br = Vector3f::new(self.width, self.height, 0.0);

        self.tree_search(&self.root, i, r, tl, br)
    }

    fn tree_search(&self, node: &Node, i: usize, r: f32, tl: Vector3f, br: Vector3f) -> Vec<usize> {
        let pos = self.items[i].position();
        let mut v: Vec<usize> = Vec::new();

        if node.items.len() < MIN_POINTS {
            for j in node.items.iter() {
                if i == *j {
                    continue;
                }

                let pj = &self.items[*j];
                let d = pos.distance(pj.position());
                if d < r {
                    v.push(*j);
                }
            }
        }

        for child in node.children.iter() {
            //WRONG
            v.extend(self.tree_search(&child, i, r, tl, br));
        }

        v
    }

    fn construct_tree(tl: Vector3f, br: Vector3f, items: &'a [T], indices: &[usize]) -> Node {
        if indices.len() < MIN_POINTS {
            return Node{
                items: indices.to_vec(),
                children: Vec::new(),
            };
        }

        let mid = (tl + br) / 2.0;

        let mut nw: Vec<usize> = Vec::new();
        let mut ne: Vec<usize> = Vec::new();
        let mut sw: Vec<usize> = Vec::new();
        let mut se: Vec<usize> = Vec::new();

        for i in indices.iter() {
            let pi = &items[*i];
            let pos = pi.position();
            
            if pos.x < mid.x && pos.y < mid.y {
                nw.push(*i);
            } else if pos.x > mid.x && pos.y < mid.y {
                ne.push(*i);
            } else if pos.x > mid.x && pos.y > mid.y {
                se.push(*i);
            } else if pos.x < mid.x && pos.y > mid.y {
                sw.push(*i);
            }
        }

        let mut children: Vec<Node> = Vec::new();

        children.push(Octree::<T>::construct_tree(
            tl, mid, items, &nw
        ));
        children.push(Octree::<T>::construct_tree(
            Vector3f::new(mid.x, tl.y, 0.0), Vector3f::new(br.x, mid.y, 0.0), items, &ne
        ));
        children.push(Octree::<T>::construct_tree(
            mid, br, items, &se
        ));
        children.push(Octree::<T>::construct_tree(
            Vector3f::new(tl.x, mid.y, 0.0), Vector3f::new(mid.x, br.y, 0.0), items, &sw
        ));


        Node{
            items: Vec::new(),
            children: children,
        }
    }

    pub fn debug_get_splits(&self) -> Vec<(Vector3f, Vector3f)> {
        let tl = Vector3f::new(0.0, 0.0, 0.0);
        let br = Vector3f::new(self.width, self.height, 0.0);

        self.debug_node_splits(&self.root, tl, br)
    }

    fn debug_node_splits(&self, node: &Node, tl: Vector3f, br: Vector3f) -> Vec<(Vector3f, Vector3f)> {
        let mut v: Vec<(Vector3f, Vector3f)> = Vec::new();

        if node.children.len() == 0 {
            return v;
        }

        let mid = (tl + br) / 2.0;

        // Vertical line
        v.push((Vector3f::new(mid.x, tl.y, 0.0), Vector3f::new(mid.x, br.y, 0.0)));
        // Horizontal line
        v.push((Vector3f::new(tl.x, mid.y, 0.0), Vector3f::new(br.x, mid.y, 0.0)));

        v.extend(self.debug_node_splits(&node.children[0], tl, mid));
        v.extend(self.debug_node_splits(&node.children[1], Vector3f::new(mid.x, tl.y, 0.0), Vector3f::new(br.x, mid.y, 0.0)));
        v.extend(self.debug_node_splits(&node.children[2], mid, br));
        v.extend(self.debug_node_splits(&node.children[3], Vector3f::new(tl.x, mid.y, 0.0), Vector3f::new(mid.x, br.y, 0.0)));

        v
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}