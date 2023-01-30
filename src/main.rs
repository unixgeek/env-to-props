use java_properties::PropertiesWriter;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufWriter;

// fn main() {
//     let mut writer = BufWriter::new(File::create("/tmp/test.properties").expect("properties file"));
//
//     env::vars()
//         .filter(|(key, _)| key.starts_with("TALEND_"))
//         .for_each(|(key, value)| {
//             let property_name = key.strip_prefix("TALEND_").expect("prefix");
//             writeln!(writer, "{property_name}={value}").expect("write");
//         });
// }

fn main() {
    // let props: HashMap<String, String> = env::vars()
    //     .filter(|(key, _)| key.starts_with("TALEND_"))
    //     .map(|(key, value)| {
    //         let property_name = key.strip_prefix("TALEND_").expect("prefix");
    //         (property_name.to_string(), value)
    //     })
    //     .collect();
    //
    // let writer = PropertiesWriter::new(BufWriter::new(file::create("/tmp/test.properties")));
    //
    // java_properties::write(writer, &props).expect("writing file");

    let mut writer = PropertiesWriter::new(BufWriter::new(
        File::create("/tmp/test.properties").expect("creating file"),
    ));

    env::vars()
        .filter(|(key, _)| key.starts_with("TALEND_"))
        .for_each(|(key, value)| {
            let property_name = key.strip_prefix("TALEND_").expect("prefix");
            writer.write(property_name, &value).expect("write the prop");
        });
}
