#[macro_export]
macro_rules! WindowResources {
    ($(($idx:ident,$renum:expr,$type:tt, $path:expr)),* $(,)*) => {
        use std::collections::HashMap;
        #[derive(Debug, Clone, Copy)]
        pub struct WindowResources{
            $(
                $idx:impl_field!{$idx,$renum,$type,$path}
            ),*
        }
        impl WindowResources {
            pub fn new() -> CrResult<Self> {
                Ok(WindowResources{
                    $(
                        $idx:impl_value!{$idx,$renum,$type,$path}
                    ),*
                })
            }
            pub fn pump(&self,map:&mut HashMap<ResourceEnum,SupportIdType>,image_map:&mut conrod_core::image::Map<TextureHandle>,ui:&mut conrod_core::Ui ){
                $(
                    impl_pump!{$idx,$renum,$type,$path,map,image_map,ui,self.$idx}
                )*
            }
            fn load_font(handle:BytesHandle) ->conrod_core::text::Font{
                FontCollection::from_bytes(crayon_bytes::create_bytes(handle).unwrap()).unwrap().into_font().unwrap()
            }
        }
        impl LatchProbe for WindowResources {
            fn is_set(&self) -> bool {
                let mut v=0;
                $(
                    impl_probe!{$idx,$renum,$type,$path,v,self.$idx}
                )*
                v ==0
            }
        }
    }
}
#[macro_export]
macro_rules! impl_field {
    ($idx:ident,$renum:expr,"image",$path:expr)=>{
        TextureHandle
    };
    ($idx:ident,$renum:expr,"texture",$path:expr)=>{
        TextureHandle
    };
    ($idx:ident,$renum:expr,"font",$path:expr)=>{
        BytesHandle
    };
    ($idx:ident,$renum:expr,"audio",$path:expr)=>{
        AudioClipHandle
    };
}
#[macro_export]
macro_rules! impl_value {
    ($idx:ident,$renum:expr,"image",$path:expr)=>{
        crayon::video::create_texture_from($path)?
    };
    ($idx:ident,$renum:expr,"texture",$path:expr)=>{
        crayon::video::create_texture_from($path)?
    };
    ($idx:ident,$renum:expr,"font",$path:expr)=>{
        crayon_bytes::create_bytes_from($path)?
    };
    ($idx:ident,$renum:expr,"audio",$path:expr)=>{
        crayon_audio::create_clip_from($path)?
    };
}
#[macro_export]
macro_rules! impl_pump {
    ($idx:ident,$renum:expr,"image",$path:expr,$map:expr,$image_map:expr,$ui:expr,$sidx:expr)=>{
        $map.insert($renum,SupportIdType::ImageId($image_map.insert($sidx)));
    };
    ($idx:ident,$renum:expr,"texture",$path:expr,$map:expr,$image_map:expr,$ui:expr,$sidx:expr)=>{
        $map.insert($renum,SupportIdType::TextureId(image_map.insert($sidx)));
    };
    ($idx:ident,$renum:expr,"font",$path:expr,$map:expr,$image_map:expr,$ui:expr,$sidx:expr)=>{
        $map.insert($renum,SupportIdType::FontId($ui.fonts.insert(Self::load_font($sidx))));
    };
    ($idx:ident,$renum:expr,"audio",$path:expr,$map:expr,$image_map:expr,$ui:expr,$sidx:expr)=>{
        $map.insert($renum,SupportIdType::TextureId($image_map.insert($sidx)));
    };
}
#[macro_export]
macro_rules! impl_probe {
    ($idx:ident,$renum:expr,"image",$path:expr,$map:expr,$sidx:expr)=>{
        if crayon::video::texture_state($sidx)==ResourceState::NotReady{
            $map+=1;
        }
    };
    ($idx:ident,$renum:expr,"texture",$path:expr,$map:expr,$sidx:expr)=>{
        if crayon::video::texture_state($sidx)==ResourceState::NotReady{
            $map+=1;
        }
    };
    ($idx:ident,$renum:expr,"font",$path:expr,$map:expr,$sidx:expr)=>{
        if crayon_bytes::bytes_state($sidx)==ResourceState::NotReady{
            $map+=1;
        }
    };
    ($idx:ident,$renum:expr,"audio",$path:expr,$map:expr,$sidx:expr)=>{
        if crayon_audio::clip_state($sidx)==ResourceState::NotReady{
            $map+=1;
        }
    };
}
#[macro_export]
macro_rules! CGM_iter_resource_enum_vala_pump{
    ()=>{
        
        pub fn iter_resource_enum_vala_next(kk:ResourceEnum,v:Vala)->
        (ResourceEnum,Option<TextureHandle>,Option<BytesHandle>,Option<AudioClipHandle>){ //(for resultMap,image_map,ui)
            if v.source_type =="image"{
                let rust_logo = load_image(v.path);
                return (kk,Some(rust_logo),None,None)
            }else if v.source_type =="texture"{
                let texture_logo = load_image(v.path);
                return (kk,Some(texture_logo),None,None)                  
            }else if v.source_type =="music"{
                let texture_logo = load_audio(v.path);
                return (kk,None,None,Some(texture_logo))                  
            }else {
                //let font = conrod_core::text::FontCollection::from_bytes(crayon_bytes::create_bytes(load_bytes(v.path)).unwrap()).unwrap().into_font().unwrap();
                let font = load_bytes(v.path);
                return (kk,None,Some(font),None)
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
    }
}

#[macro_export]
macro_rules! SupportIdType{
    ()=>{
        pub enum SupportIdType {
            ImageId(conrod_core::image::Id),
            FontBytes(BytesHandle),
            FontId(conrod_core::text::font::Id),
            TextureId(TextureHandle),
            AudioId(AudioClipHandle)
        }
    }
}
