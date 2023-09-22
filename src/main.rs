use std::error::Error;
use std::fs::File;
use std::io::Write;

const HTML_TEMPLATE: &'static str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    
</body>
</html>
"#;

fn main() -> Result<(), Box<dyn Error>> {
	let mut file = File::create("index.html")?;
	file.write_all(HTML_TEMPLATE.as_bytes())?;
	println!("Done! Your HTML file is ready! :)");
	Ok(())
}
