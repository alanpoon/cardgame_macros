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
        pub fn iter_resource_enum_vala_next(kk:ResourceEnum,v:Vala)->
        (ResourceEnum,Option<image::RgbaImage>,Option<conrod::text::Font>,Option<support::assets::AudioType>){ //(for resultMap,image_map,ui)
                if v.source_type =="image"{
                    let rust_logo = load_image(v.path);
                    return (kk,Some(rust_logo),None,None)
                } else if v.source_type =="image90"{
                    let rust_logo = load_image90( v.path);
                     return (kk,Some(rust_logo),None,None)
                }else if v.source_type =="image270"{
                    let rust_logo = load_image270(v.path);
                     return (kk,Some(rust_logo),None,None)
                }else if v.source_type =="texture"{
                    let texture_logo = load_image(v.path);
                     return (kk,Some(texture_logo),None,None)                  
                }else if v.source_type =="music"{
                    let texture_logo = load_audio(v.path);
                     return (kk,None,None,Some(texture_logo))                  
                }else {
                     return (kk,None,Some(support::assets::load_font(v.path)),None)
                }
        }
        
     fn load_image( path: &str) -> image::RgbaImage
    {
        support::assets::load_image(path).to_rgba()
    }
    fn load_image90( path: &str) -> image::RgbaImage
    {
       support::assets::load_90image(path).to_rgba()
    }
    fn load_image270( path: &str) -> image::RgbaImage
    {
        support::assets::load_270image(path).to_rgba()
    }

    fn load_audio( path: &str) -> support::assets::AudioType
    {
        support::assets::load_audio(path)
    }
    }
}

#[macro_export]
macro_rules! SupportIdType{
    ()=>{
        pub enum SupportIdType {
            ImageId(conrod::image::Id),
            FontId(conrod::text::font::Id),
            TextureId(glium::Texture2d),
            MusicId(support::assets::AudioType)
        }
    }
}
