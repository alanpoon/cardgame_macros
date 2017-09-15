#[macro_export]
macro_rules! CGM_game{
    ()=>{
        let mut image_map = conrod::image::Map::new();
        let g = ImageIds::new();
        let mut result_map = HashMap::<ResourceEnum, SupportIdType>::new();
        g.pump(&mut result_map, &display, &mut ui, &mut image_map);
    }
}
