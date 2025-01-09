material.rs
/*


this include the material cost conversion
*/
mod materials{ 
   pub struct Material{
   type_of_materials : Vec<str>
   materials_cost: f64,
   material_used_in_whole_digit : i128
   materials_when_used: f64,
   materials_lefts: Vec<f64>,
   }
   impl  Material{
       pub fn new() -> Material{
           Material {
           type_of_materials: Vec![],
           materials_cost: 25.99,
           material_left_in_whole_digit: 100,
           material_when_used : 5.00
           materials_left Vec![],
          }
       }
       pub fn subtraction_of_materials(materails_left_in_whole_digits i128,material_when_used f64) -> Material {
       
       }
  }
}

fn conversion_of_material(material_used_up:i64) ->f64{
   use materials::*
   
   let mtrl: Materials = materials::new();
      
   let material_being_used=material_used_up/100.0;
   return material_being_usedl;
   }
fn converstion_of_material(material_used_Up:f64) ->f64{
   return materail_used_up;
   }
