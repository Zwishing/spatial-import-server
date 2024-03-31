use std::path::Path;
use gdal::Dataset;
use gdal::vector::LayerAccess;

// 读取矢量数据的字段和字段类型
pub fn get_attrs(url:&str){
    let dataset = Dataset::open(Path::new(url)).unwrap();
    let layer = dataset.layer(0).unwrap();
    let spatial_ref = layer.spatial_ref();
    let count = layer.feature_count();
    let defn = layer.defn();
    for field in defn.fields(){
        println!("{},{}",field.name(),field.field_type());
        println!("12121212");
    }
    println!("{:?}",spatial_ref);
    println!("{}",count);
}