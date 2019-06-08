#[macro_export]
macro_rules! CGM_game{
    ()=>{
        let mut image_map = conrod_core::image::Map::new();
        let g = ImageIds::new();
        let mut result_map = HashMap::<ResourceEnum, SupportIdType>::new();
        g.pump(&mut result_map, &mut ui, &mut image_map);
    }
}
