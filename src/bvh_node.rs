use std::{ops::Index, cmp::{self, Ordering}};

use crate::{hittable::Hittable, aabb::Aabb, random::random_double, vector::vector::Vec3};

pub enum BvhNode{
    Branch {left: Box<BVH>, right: Box<BVH>},
    Leaf(Box<dyn Hittable>)
}

pub struct BVH {
    tree: BvhNode,
    bbox: Aabb
}

impl BVH {
    pub fn new(mut hitable: Vec<Box<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        fn box_compare(time0: f64, time1: f64, axis: i32) -> impl FnMut(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering {
            let stupid = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
            move |a,b| {
                let a_bbox = a.bounding_box(time0, time1,stupid);
                let b_bbox = b.bounding_box(time0, time1, stupid);
                if let (Some(a), Some(b)) = (a_bbox, b_bbox) {
                    let ac = a.min().getByNumber(axis) + a.max().getByNumber(axis);
                    let bc = b.min().getByNumber(axis) + b.max().getByNumber(axis);
                    ac.partial_cmp(&bc).unwrap()
                } else {
                    panic!("No bounding box in bvh node");
                }
            }
        }

        fn axis_range(hitable: &Vec<Box<dyn Hittable>>, time0: f64, time1: f64, axis: i32) -> f64{
            let (min, max) = hitable.iter().fold((f64::MAX, f64::MIN), |(bmin, bmax), hit| {
                let stupid = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
                
                if let Some(aabb) = hit.bounding_box(time0, time1, stupid) {
                    (bmin.min(aabb.min().getByNumber(axis)), bmax.max(aabb.max().getByNumber(axis)))
                } else {
                    (bmin, bmax)
                }
            });
            max - min
        }

        let mut axis_ranges: Vec<(i32, f64)> = (0..3).map(|a| (a, axis_range(&hitable, time0, time1, a))).collect();

        axis_ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let axis = axis_ranges[0].0;

        hitable.sort_unstable_by(box_compare(time0, time1, axis));
        let len = hitable.len();
        match len {
            0 => panic!("no elements in scene"),
            1 => {
                let leaf = hitable.pop().unwrap();
                let stupid = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
                if let Some(bbox) = leaf.bounding_box(time0, time1, stupid) {
                    BVH {tree: BvhNode::Leaf(leaf), bbox}
                } else {
                    panic!("No bounding box in bvh node")
                }
            },
            _ => {
                let right = BVH::new(hitable.drain(len / 2..).collect(), time0, time1);
                let left = BVH::new(hitable, time0, time1);
                let bbox = Aabb::surrounding_box(left.bbox, right.bbox);
                BVH {tree: BvhNode::Branch { left: Box::new(left), right: Box::new(right) }, bbox}
            }
        }
    }
}




impl Hittable for BVH {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: Aabb) -> Option<Aabb> {
        return Some(self.bbox);
    }
    fn hit(&self, r: &crate::ray::ray::Ray, t_min: f64, mut t_max: f64) -> Option<crate::hittable::HitRecord> {
        if self.bbox.hit(*r, t_min, t_max) {
            match &self.tree {
                BvhNode::Leaf(leaf) => leaf.hit(r, t_min, t_max),
                BvhNode::Branch { left, right } => {
                    let left = left.hit(r, t_min, t_max);
                    if let Some(l) = &left {t_max = l.t};
                    let right = right.hit(r, t_min, t_max);
                    if right.is_some() {right} else {left}
                }
            }
        } else {
            None
        }
        
    }
}

pub fn random_int(min: i32, max: i32) -> i32 {
    return (random_double(min as f64, max as f64 +1.0) as i32);
}
