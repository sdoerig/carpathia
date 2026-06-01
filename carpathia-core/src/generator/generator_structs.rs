use std::{path::Path, path::PathBuf, str::FromStr};

use log::debug;

pub(crate) struct Template {
    pub template_type: TemplateType,
    pub file_path: PathBuf,
    pub file_name: String,
    pub suffix: String,
}

impl Template {
    pub fn new(output_path: &Path, file_path: &PathBuf) -> Self {
        let file_name = match file_path.file_name() {
            Some(name) => name.to_string_lossy(),
            None => {
                debug!("Invalid file name: {:?}", file_path);
                "unknown".into()
            }
        };
        let file_name_tokens: Vec<&str> = file_name.split('.').collect();

        let template_type: TemplateType = file_name_tokens
            .first()
            .unwrap()
            .parse()
            .unwrap_or(TemplateType::Unknown);
        let suffix = if file_name_tokens.len() > 2 {
            file_name_tokens[file_name_tokens.len() - 2].to_string()
        } else {
            "unknown".to_string()
        };
        let file_name = match template_type {
            TemplateType::Unknown => "unknown".to_string(),
            _ => file_name_tokens[1..(file_name_tokens.len() - 2)].join("_"),
        };

        Template {
            template_type,
            file_path: output_path.join(file_path),
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
    use std::path::PathBuf;
    struct TestCase {
        file_path: PathBuf,
        expected_template_type: TemplateType,
        expected_suffix: String,
        expected_file_name: String,
        error_message: String,
    }
    #[test]
    fn test_template() {
        let output_path = PathBuf::from("output/test");
        let test_caseses = vec![
            // summary
            TestCase {
                file_path: PathBuf::from("summary.index.html.tera"),
                expected_template_type: TemplateType::Summary,
                expected_suffix: "html".to_string(),
                expected_file_name: "index".to_string(),
                error_message: "Failed to create summary template".to_string(),
            },
            TestCase {
                file_path: PathBuf::from("a/summary.index.html.tera"),
                expected_template_type: TemplateType::Summary,
                expected_suffix: "html".to_string(),
                expected_file_name: "index".to_string(),
                error_message: "Failed to create summary template with directories".to_string(),
            },
            // views with one name token
            TestCase {
                file_path: PathBuf::from("views.index.html.tera"),
                expected_template_type: TemplateType::View,
                expected_suffix: "html".to_string(),
                expected_file_name: "index".to_string(),
                error_message: "Failed to create view template".to_string(),
            },
            TestCase {
                file_path: PathBuf::from("a/views.index.html.tera"),
                expected_template_type: TemplateType::View,
                expected_suffix: "html".to_string(),
                expected_file_name: "index".to_string(),
                error_message: "Failed to create view template with directories".to_string(),
            },
            TestCase {
                file_path: PathBuf::from("views.index.html.tera"),
                expected_template_type: TemplateType::View,
                expected_suffix: "html".to_string(),
                expected_file_name: "index".to_string(),
                error_message: "Failed to create view template".to_string(),
            },
            TestCase {
                file_path: PathBuf::from("a/views.index.html.tera"),
                expected_template_type: TemplateType::View,
                expected_suffix: "html".to_string(),
                expected_file_name: "index".to_string(),
                error_message: "Failed to create view template with directories".to_string(),
            },
            // views with multiple name tokens
            TestCase {
                file_path: PathBuf::from("views.index.chicken.html.tera"),
                expected_template_type: TemplateType::View,
                expected_suffix: "html".to_string(),
                expected_file_name: "index_chicken".to_string(),
                error_message: "Failed to create view template".to_string(),
            },
            TestCase {
                file_path: PathBuf::from("a/views.index.chicken.html.tera"),
                expected_template_type: TemplateType::View,
                expected_suffix: "html".to_string(),
                expected_file_name: "index_chicken".to_string(),
                error_message: "Failed to create view template with directories".to_string(),
            },
            TestCase {
                file_path: PathBuf::from("views.index.chicken.html.tera"),
                expected_template_type: TemplateType::View,
                expected_suffix: "html".to_string(),
                expected_file_name: "index_chicken".to_string(),
                error_message: "Failed to create view template".to_string(),
            },
            TestCase {
                file_path: PathBuf::from("a/views.index.chicken.html.tera"),
                expected_template_type: TemplateType::View,
                expected_suffix: "html".to_string(),
                expected_file_name: "index_chicken".to_string(),
                error_message: "Failed to create view template with directories".to_string(),
            },
        ];
        for test_case in test_caseses {
            let template = Template::new(&output_path, &test_case.file_path);
            assert_eq!(
                template.template_type, test_case.expected_template_type,
                "{}",
                test_case.error_message
            );
            assert_eq!(
                template.suffix, test_case.expected_suffix,
                "{}",
                test_case.error_message
            );
            assert_eq!(
                template.file_name, test_case.expected_file_name,
                "{}",
                test_case.error_message
            );
        }
    }

}
