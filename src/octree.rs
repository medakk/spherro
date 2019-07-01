use crate::util::{Vector3f};
use cgmath::{MetricSpace, InnerSpace};

//TODO: Implement the OCTREE! :p
// this is currently a quadtree

const MIN_POINTS: usize = 5;
const MAX_DEPTH: usize = 16;

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

fn point_line_dist(p: Vector3f, line: &(Vector3f, Vector3f)) -> Option<f32> {
    let line_vec = (line.1 - line.0).normalize();
    let lambda = line_vec.dot(p - line.0);

    let poi = lambda * line_vec + line.0;

    let intersect_length = (poi - p).magnitude();

    // TODO: Figure out when poi doesn't actually lie on the line
    Some(intersect_length)
}

fn circle_rect_collide(circle: (Vector3f, f32), rect: &(Vector3f, Vector3f)) -> bool {
    let circle_in_rect = circle.0.x > rect.0.x &&
                         circle.0.x < rect.1.x &&
                         circle.0.y > rect.0.y &&
                         circle.0.y < rect.1.y;
    if circle_in_rect {
        return true;
    }

    let r = circle.1;
    let l1 = (
        rect.0,
        Vector3f::new(rect.1.x, rect.0.y, 0.0),
    );
    let l2 = (
        Vector3f::new(rect.1.x, rect.0.y, 0.0),
        rect.1,
    );
    let l3 = (
        Vector3f::new(rect.0.x, rect.1.y, 0.0),
        rect.1,
    );
    let l4 = (
        rect.0,
        Vector3f::new(rect.0.x, rect.1.y, 0.0),
    );

    let check = |l| {
        match point_line_dist(circle.0, l) {
            Some(v) => { v < r },
            _       => { false },
        }
    };

    check(&l1) ||
    check(&l2) ||
    check(&l3) ||
    check(&l4)
}

impl<'a, T> Octree<'a, T> where T: HasPosition + Clone {
    pub fn new(width: f32, height: f32, items: &'a [T]) -> Self {
        let tl = Vector3f::new(0.0, 0.0, 0.0);
        let br = Vector3f::new(width, height, 0.0);

        let mut indices: Vec<usize> = Vec::new();
        for i in 0..items.len() {
            indices.push(i);
        }

        let root = Octree::<T>::construct_tree(tl, br, items, &indices, 0);
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

        if node.children.len() == 0 {
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

            return v;
        }

        /*
        // Uncomment for brute force
        for child in node.children.iter() {
            v.extend(self.tree_search(&child, i, r, tl, br));
        }
        return v;
        */

        let mid = (tl + br) / 2.0;

        let rect0 = (tl, mid);
        let rect1 = (Vector3f::new(mid.x, tl.y, 0.0), Vector3f::new(br.x, mid.y, 0.0));
        let rect2 = (mid, br);
        let rect3 = (Vector3f::new(tl.x, mid.y, 0.0), Vector3f::new(mid.x, br.y, 0.0));

        for (child, rect) in izip!(node.children.iter(), [rect0, rect1, rect2, rect3].iter()) {
            if circle_rect_collide((pos, r), rect) {
                v.extend(self.tree_search(&child, i, r, rect.0, rect.1));
            }
        }

        v
    }

    fn construct_tree(tl: Vector3f, br: Vector3f, items: &'a [T], indices: &[usize], depth: usize) -> Node {
        if indices.len() < MIN_POINTS || depth == MAX_DEPTH {
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
            tl, mid, items, &nw, depth+1
        ));
        children.push(Octree::<T>::construct_tree(
            Vector3f::new(mid.x, tl.y, 0.0), Vector3f::new(br.x, mid.y, 0.0), items, &ne, depth+1
        ));
        children.push(Octree::<T>::construct_tree(
            mid, br, items, &se, depth+1
        ));
        children.push(Octree::<T>::construct_tree(
            Vector3f::new(tl.x, mid.y, 0.0), Vector3f::new(mid.x, br.y, 0.0), items, &sw, depth+1
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