use conrod::UiBuilder
pub struct UI{
    pub ui:UiBuilder,
}

impl UI{
    pub fn new(w:i32,h:i32)->Self{
       let mut ui = conrod::UiBuilder::new([w as f64, h as f64]).build();
       UI{
           ui:ui
       }
    }
    pub fn add_image<T>(imageids:T){
     let mut image_map = conrod::image::Map::new();
    }
}