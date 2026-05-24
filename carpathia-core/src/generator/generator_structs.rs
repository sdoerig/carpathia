use std::str::FromStr;

use log::debug;
#[allow(dead_code)]
struct Template {
    template_type: TemplateType,
    suffix: String,
}

impl Template {
    #[expect(dead_code)]
    fn new(file_name: &str) -> Self {
        let _file_name_tokens: Vec<&str> = file_name.split('.').collect();
        Template {template_type: TemplateType::Unknown, suffix: String::new()}
    }
}

enum TemplateType {
    Summary,
    View,
    Table,
    Unknown,
}

impl FromStr for TemplateType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "summary" => Ok(TemplateType::Summary),
            "tables" => Ok(TemplateType::Table),
            "views" => Ok(TemplateType::View),
            _ => {
                debug!("Invalid template type found: {}", s);
                Ok(TemplateType::Unknown)
            }
        }
    }
}
