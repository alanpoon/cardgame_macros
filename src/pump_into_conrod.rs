#[macro_export]
macro_rules! CGM_image_map {
     ($(($idx:expr,$type:expr, $path:expr)),* $(,)*) => {
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
                image_m:&mut conrod_core::image::Map<TextureHandle>){
                for (k,v) in &self.map{
                    let  kk = k.clone();
                    if v.source_type =="image"{
                        let rust_logo = load_image(v.path);
                        let id_i = image_m.insert(rust_logo);
                        result_map.insert(kk,SupportIdType::ImageId(id_i));
                    }else if v.source_type =="texture"{
                        let texture_logo = load_image(v.path);
                        result_map.insert(kk,SupportIdType::TextureId(texture_logo));
                    }else if v.source_type =="music"{
                        let texture_logo = load_audio(v.path);
                        result_map.insert(kk,SupportIdType::AudioId(texture_logo));
                    } else {
                        //let font = conrod_core::text::FontCollection::from_bytes(crayon_bytes::create_bytes(load_bytes(v.path)).unwrap()).unwrap().into_font().unwrap();
                        let font = load_bytes(v.path);
                        result_map.insert(kk,SupportIdType::FontBytes(font));
                    }
                } 
            }
        }
        fn load_bytes(path:&str) -> BytesHandle{
            crayon_bytes::create_bytes_from(path).unwrap()
        }
        fn load_image( path: &str) -> TextureHandle{
            video::create_texture_from(path).unwrap()
        }
        fn load_audio( path: &str) -> AudioClipHandle{
            crayon_audio::create_clip_from(path).unwrap()
        }
 };
}
