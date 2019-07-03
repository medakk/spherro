use crate::util::{Vector3f};
use crate::accelerators::{HasPosition, Accelerator};
use cgmath::{MetricSpace, InnerSpace};

const MIN_POINTS: usize = 5;
const MAX_DEPTH: usize = 16;

//TODO: Quadtree variable names and such use the "y axis is down" assumption,
// unlike the rest of the code. While its still functional, it should follow
// the same convention
//TODO: Write tests for quadtree

pub struct Quadtree<'a, T> {
    root: Node,
    width: f32,
    height: f32,
    items: &'a [T],
}

// The node only stores the index of the item
struct Node {
    items: Vec<usize>,
    children: Vec<Node>,
}

struct Circle(Vector3f, f32);
struct Line(Vector3f, Vector3f);
struct Rect(Vector3f, Vector3f);

impl Circle {
    pub fn contains_pt(&self, point: &Vector3f) -> bool {
        (point - self.0).magnitude2() < self.1.powi(2)
    }
}

fn circle_line_distance2(circle: &Circle, line: &Line) -> Option<f32> {
    let p = circle.0;

    //TODO: Avoid this square root
    let line_vec = (line.1 - line.0).normalize();
    let lambda = line_vec.dot(p - line.0);

    let poi = lambda * line_vec + line.0;
    let t = if line.1.x - line.0.x > 1e-6 {
        (poi.x - line.0.x) / (line.1.x - line.0.x)
    } else {
        (poi.y - line.0.y) / (line.1.y - line.0.y)
    };

    if t > 0.0 && t < 1.0 {
        let intersect_length = (poi - p).magnitude2();
        Some(intersect_length)
    } else {
        None
    }
}

fn circle_rect_collide(circle: &Circle, rect: &Rect) -> bool {
    let circle_in_rect = circle.0.x > rect.0.x &&
                         circle.0.x < rect.1.x &&
                         circle.0.y > rect.0.y &&
                         circle.0.y < rect.1.y;
    if circle_in_rect {
        return true;
    }

    let p3 = Vector3f::new(rect.1.x, rect.0.y, 0.0);
    let p4 = Vector3f::new(rect.0.x, rect.1.y, 0.0);

    if circle.contains_pt(&rect.0) ||
       circle.contains_pt(&rect.1) ||
       circle.contains_pt(&p3) ||
       circle.contains_pt(&p4) {
           return true;
    }

    let l1 = Line(rect.0, p3);
    let l2 = Line(p3, rect.1);
    let l3 = Line(p4, rect.1);
    let l4 = Line(p4, rect.0);

    let check = |l| {
        match circle_line_distance2(&circle, &l) {
            Some(d) => { d < circle.1.powi(2) },
            _ => { false }
        }
    };

    check(l1) || check(l2) || check(l3) || check(l4)
}

impl<'a, T> Accelerator for Quadtree<'a, T> where T: HasPosition {
    fn nearest_neighbours(&self, i: usize, r: f32) -> Vec<usize> {
        let tl = Vector3f::new(0.0, 0.0, 0.0);
        let br = Vector3f::new(self.width, self.height, 0.0);

        self.node_search(&self.root, i, r, tl, br)
    }
}

impl<'a, T> Quadtree<'a, T> where T: HasPosition {
    pub fn new(width: f32, height: f32, items: &'a [T]) -> Self {
        let tl = Vector3f::new(0.0, 0.0, 0.0);
        let br = Vector3f::new(width, height, 0.0);

        let mut indices: Vec<usize> = Vec::new();
        for i in 0..items.len() {
            indices.push(i);
        }

        let root = Quadtree::<T>::node_construct(tl, br, items, &indices, 0);
        Quadtree{
            root: root,
            width: width,
            height: height,
            items: items,
        }
    }

    fn node_search(&self, node: &Node, i: usize, r: f32, tl: Vector3f, br: Vector3f) -> Vec<usize> {
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

        let mid = (tl + br) / 2.0;

        let rect0 = Rect(tl, mid);
        let rect1 = Rect(Vector3f::new(mid.x, tl.y, 0.0), Vector3f::new(br.x, mid.y, 0.0));
        let rect2 = Rect(mid, br);
        let rect3 = Rect(Vector3f::new(tl.x, mid.y, 0.0), Vector3f::new(mid.x, br.y, 0.0));
        let circle = Circle(pos, r);

        for (child, rect) in izip!(node.children.iter(), [rect0, rect1, rect2, rect3].iter()) {
            if circle_rect_collide(&circle, rect) {
                let found = self.node_search(&child, i, r, rect.0, rect.1);
                v.extend(found);
            }
        }

        v
    }

    fn node_construct(tl: Vector3f, br: Vector3f, items: &'a [T], indices: &[usize], depth: usize) -> Node {
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

        children.push(Quadtree::<T>::node_construct(
            tl, mid, items, &nw, depth+1
        ));
        children.push(Quadtree::<T>::node_construct(
            Vector3f::new(mid.x, tl.y, 0.0), Vector3f::new(br.x, mid.y, 0.0), items, &ne, depth+1
        ));
        children.push(Quadtree::<T>::node_construct(
            mid, br, items, &se, depth+1
        ));
        children.push(Quadtree::<T>::node_construct(
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