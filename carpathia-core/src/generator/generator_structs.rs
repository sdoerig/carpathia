use std::str::FromStr;

use log::debug;
#[allow(dead_code)]
pub(crate) struct Template {
    pub template_type: TemplateType,
    pub file_name: String,
    pub suffix: String,
}

impl Template {
    #[allow(dead_code)]
    fn new(file_name: &str) -> Self {
        let file_name_tokens: Vec<&str> = file_name.split('.').collect();
        let template_type: TemplateType = file_name_tokens
            .first()
            .unwrap()
            .parse()
            .unwrap_or(TemplateType::Unknown);
        let (file_name, suffix) = match template_type {
            TemplateType::Summary => (
                file_name_tokens.get(1).unwrap_or(&"unknown").to_string(),
                file_name_tokens.get(2).unwrap_or(&"unknown").to_string(),
            ),
            TemplateType::View | TemplateType::Table => (
                file_name_tokens.first().unwrap_or(&"unknown").to_string(),
                file_name_tokens.get(1).unwrap_or(&"unknown").to_string(),
            ),
            TemplateType::Unknown => ("unknown".to_string(), "unknown".to_string()),
        };

        Template {
            template_type,
            file_name,
            suffix,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub(crate) enum TemplateType {
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

#[cfg(test)]
mod tests {
    use crate::generator::generator_structs::{Template, TemplateType};

    #[test]
    fn test_template_summary() {
        let template = Template::new("summary.index.html.tera");
        assert_eq!(template.template_type, TemplateType::Summary);
        assert_eq!(template.suffix, "html");
        assert_eq!(template.file_name, "index");
    }

    #[test]
    fn test_template_views() {
        let template = Template::new("views.html.tera");
        assert_eq!(template.template_type, TemplateType::View);
        assert_eq!(template.suffix, "html");
        assert_eq!(template.file_name, "views");
    }

    #[test]
    fn test_template_tables() {
        let template = Template::new("tables.html.tera");
        assert_eq!(template.template_type, TemplateType::Table);
        assert_eq!(template.suffix, "html");
        assert_eq!(template.file_name, "tables");
    }
    #[test]
    fn test_template_unknown() {
        let template = Template::new("scrapyard.rst");
        assert_eq!(template.template_type, TemplateType::Unknown);
        assert_eq!(template.suffix, "unknown");
        assert_eq!(template.file_name, "unknown");
    }
}
