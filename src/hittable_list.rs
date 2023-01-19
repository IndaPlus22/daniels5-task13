use crate::Hittable;
use crate::Ray;
use crate::HitRecord;


pub struct HittableList {
    list: Vec<Box<dyn Hittable>> //Since the size is unknown at compilation time we need to use Box so that it is stored in the heap.
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList{
        HittableList {list}
    }
}

//Go through all the the objects in the list and calculate if the given ray hits any of the objects, if so take the closest hit point. (For example if we have 2 balls that the ray can hit we only want the closest since the other is behined and should not be shown)

impl Hittable for HittableList { 
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;

        for object in &self.list {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) { //Calculate the if the ray hits an object based on the given t value.
                closest_so_far = rec.t; //Take the t value AKA the max length of the ray
                hit_anything = Some(rec);
            }
        }
        return hit_anything; //Return the hit record for the ray. 
    }
    
}