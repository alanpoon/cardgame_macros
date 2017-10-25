#[macro_export]
macro_rules! CGM_image_map {
     ($(($idx:expr,$type:expr, $path:expr)),* $(,)*) => {
        use std::path::{Path};
        struct Vala{
            source_type:&'static str,
            path:&'static str
        }
        pub struct ImageIds{
            map:HashMap<ResourceEnum,Vala>
        }

        impl ImageIds{
            pub fn new()->ImageIds{
                let mut map = HashMap::<ResourceEnum,Vala>::new();
                $(map.insert($idx,Vala{source_type:$type,path:$path});)*
                ImageIds{
                    map:map
                }
            }

            pub fn pump(&self,result_map:&mut HashMap<ResourceEnum,SupportIdType>,
            display:&glium::Display,ui:&mut conrod::Ui,image_m:&mut conrod::image::Map< glium::Texture2d>){
                for (k,v) in &self.map{
                    let  kk = k.clone();
                    if v.source_type =="image"{
                      let rust_logo = load_image(display, v.path);
                      let id_i = image_m.insert(rust_logo);
                       result_map.insert(kk,SupportIdType::ImageId(id_i));
                    } else if v.source_type =="texture"{
                      let texture_logo = load_image(display,v.path);
                       result_map.insert(kk,SupportIdType::TextureId(texture_logo));
                    } else {
                      let id_f= ui.fonts.insert(support::assets::load_font(v.path));
                        result_map.insert(kk,SupportIdType::FontId(id_f));
                    }
                   
                } 
            }
        }
     fn load_image(display: &glium::Display, path: &str) -> glium::Texture2d
        
    {
       let rgba_image = support::assets::load_image(path).to_rgba();
        let image_dimensions = rgba_image.dimensions();
        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(),
                                                                        image_dimensions);
        let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
        texture
    }
    };
}
#[macro_export]
macro_rules! SupportIdType{
    ()=>{
        pub enum SupportIdType {
            ImageId(conrod::image::Id),
            FontId(conrod::text::font::Id),
            TextureId(glium::Texture2d)
        }
    }
}
