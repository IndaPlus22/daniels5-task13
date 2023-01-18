use crate::Hittable;
use crate::Ray;
use crate::HitRecord;
use crate::aabb::Aabb;


pub struct HittableList {
    list: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList{
        HittableList {list}
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;

        for object in &self.list {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_anything = Some(rec);
            }
        }
        return hit_anything;
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: Aabb) -> Option<Aabb> {
        //let mut temp_box: Aabb;
        let mut first_box = true;
        if(!&self.list.is_empty()) {
            for object in &self.list {
                if let temp_box = object.bounding_box(time0, time1, output_box) {
                    if(first_box) {
                        return temp_box
                    } else {
                        return Some(Aabb::surrounding_box(output_box, temp_box?))
                    }
                }
            }
        }
        return None
        
    }
}