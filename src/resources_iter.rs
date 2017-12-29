#[macro_export]
macro_rules! CGM_iter_resource_enum_vala {
     ($(($idx:expr,$type:expr, $path:expr)),* $(,)*) => {
        pub fn iter_resource_enum_vala_new(map:&mut HashMap<ResourceEnum,Vala>){
            $(map.insert($idx,Vala{source_type:$type,path:$path});)*
        }
     }
}
#[macro_export]
macro_rules! CGM_iter_resource_enum_vala_pump{
    ()=>{
        pub fn iter_resource_enum_vala_next(display:&glium::Display,kk:ResourceEnum,v:Vala)->
        (ResourceEnum,Option<glium::Texture2d>,Option<conrod::text::Font>){ //(for resultMap,image_map,ui)
                if v.source_type =="image"{
                    let rust_logo = load_image(display, v.path);
                    return (kk,Some(rust_logo),None)
                } else if v.source_type =="image90"{
                    let rust_logo = load_image90(display, v.path);
                     return (kk,Some(rust_logo),None)
                }else if v.source_type =="image270"{
                    let rust_logo = load_image270(display, v.path);
                     return (kk,Some(rust_logo),None)
                }else if v.source_type =="texture"{
                    let texture_logo = load_image(display,v.path);
                     return (kk,Some(texture_logo),None)                  
                } else {
                     return (kk,None,Some(support::assets::load_font(v.path)))
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
         fn load_image90(display: &glium::Display, path: &str) -> glium::Texture2d
        
    {
       let rgba_image = support::assets::load_90image(path).to_rgba();
        let image_dimensions = rgba_image.dimensions();
        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(),
                                                                        image_dimensions);
        let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
        texture
    }
    fn load_image270(display: &glium::Display, path: &str) -> glium::Texture2d
        
    {
       let rgba_image = support::assets::load_270image(path).to_rgba();
        let image_dimensions = rgba_image.dimensions();
        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(),
                                                                        image_dimensions);
        let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
        texture
    }
    }
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