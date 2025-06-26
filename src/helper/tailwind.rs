use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

pub const OUTPUT_CSS: &str = include_str!("../../assets/output.css");

pub fn extract_classes_from_css(output_file: &str) -> Result<(), String> {
    let content = OUTPUT_CSS;

    // regex จับ class selector แบบ .classname {
    // - เริ่มด้วยจุด \.
    // - ตามด้วย group ของตัวอักษรและเครื่องหมายที่อนุญาต (ไม่รวมช่องว่าง, {, :, ฯลฯ)
    // - ตามด้วย space/tab และ { (ไม่รวม { ใน group)
    let class_re = Regex::new(r"\.([a-zA-Z0-9_\-\:\\\/]+)\s*\{").map_err(|e| e.to_string())?;

    let mut unique_classes = HashSet::new();

    for cap in class_re.captures_iter(content) {
        if let Some(m) = cap.get(1) {
            let mut class_name = m.as_str().to_string();
            // แปลง escaped colon และ slash กลับเป็นจริง
            class_name = class_name.replace(r"\:", ":").replace(r"\/", "/");
            unique_classes.insert(class_name);
        }
    }

    let mut output =
        File::create(output_file).map_err(|e| format!("Failed to create output file: {}", e))?;

    for class_name in unique_classes.iter() {
        writeln!(output, "{}", class_name)
            .map_err(|e| format!("Failed to write to output file: {}", e))?;
    }

    Ok(())
}
