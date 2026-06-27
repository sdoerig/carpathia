/// Represents a template file with its type, path, name, and suffix.
/// Templates are expected to follow the naming convention: {type}.{name}.{suffix}.tera
/// or {type}.{suffix}.tera
/// where:
///
/// - {type} is the template type (e.g., summary, views, tables)
/// - {name} is the name of the template (e.g., index, details)
/// - {suffix} is the file extension (e.g., html, md)
///
/// The file name is parsed to extract the template type, name, and suffix for use in the generator.
///
/// It also constructs the full file path by combining the output path with the provided file path.
/// Note it prevents directory escalation by ensuring the file path is inside the output path.
/// Some words to the types available:
///
/// - Summary: A template that provides a high-level overview of the data, often used for dashboards or reports.
///   The summary gets passed the whole AbstractDbRepr (ADR)
/// - Views: Templates that define how specific data views should be rendered, such as detailed pages for individual records or lists of records.
///   Views are getting passed the AbstractTableRepr (ATR) of the database object they are rendering. This
///   means all entries of the ADR listed under the attribute "views".
/// - Tables: Templates that specify how tabular data should be rendered. It applies the same as for views
///   execpt the templates are getting passed all the entries of the ADR listed under the attribute "tables".
use std::{path::Path, path::PathBuf, str::FromStr};

use log::{debug, info};

use crate::return_values::carpathia_errors::{CarpathiaError, ErrorNumber};
pub(crate) struct Template {
    pub template_type: TemplateType,
    pub file_path: PathBuf,
    pub file_name: String,
    pub suffix: String,
}

impl Template {
    pub fn new(output_path: &Path, template_path: &PathBuf) -> Result<Self, CarpathiaError> {
        let canonical_target = check_and_provide_canonical_path(output_path, template_path)?;

        // 7. Extract file name and parse tokens
        let file_name = match canonical_target.file_name() {
            Some(name) => name.to_string_lossy().into_owned(),
            None => {
                info!("Invalid file name: {:?}", canonical_target);
                "unknown".into()
            }
        };

        let file_name_tokens: Vec<&str> = file_name.split('.').collect();
        if file_name_tokens.len() < 3 {
            info!(
                "Template will not be processed due to invalid file name format: {}",
                file_name
            );
            return Err(CarpathiaError {
                message: format!("Invalid template file name format: {}", file_name),
                error_type: ErrorNumber::InvalidConfiguration,
            });
        }

        let template_type: TemplateType =
            file_name_tokens[0].parse().unwrap_or(TemplateType::Unknown);

        let suffix = file_name_tokens[file_name_tokens.len() - 2].to_string();
        let file_name = if file_name_tokens.len() > 3 {
            file_name_tokens[1..(file_name_tokens.len() - 2)].join("_")
        } else {
            if file_name_tokens[1] == suffix {
                "".to_string()
            } else {
                file_name_tokens[1].to_string()
            }
        };

        Ok(Template {
            template_type,
            file_path: canonical_target,
            file_name,
            suffix,
        })
    }
    /// Writes the rendered template content to the file system at the location specified by the template's file path.
    /// The file name is constructed using the template's file name and suffix, and is placed
    /// in the same directory as the template file. The method ensures that the parent directory exists and
    /// handles any errors that may occur during the file writing process.
    pub(crate) fn write_rendered_template(
        &self,
        rendered_content: &str,
        db_object_name: &str,
    ) -> Result<(), CarpathiaError> {
        let parent_dir = self.file_path.parent().ok_or_else(|| CarpathiaError {
            message: format!(
                "Failed to get parent directory for template path: {:?}",
                self.file_path
            ),
            error_type: ErrorNumber::TemplateWriteError,
        })?;
        // Note that the file written can have an addtional name than the datebase object name
        // e.g. actor_repo.rs. If no addtional name is provided in the template e.g.
        // table.repo.rs.tera the addtional file name will be an empty string.
        let file_path = parent_dir.to_path_buf().join(format!(
            "{}{}.{}",
            db_object_name, self.file_name, self.suffix
        ));
        debug!("Writing rendered template to file: {:?}", file_path);
        std::fs::write(&file_path, rendered_content).map_err(|e| CarpathiaError {
            message: format!("Failed to write rendered template: {}", e),
            error_type: ErrorNumber::TemplateWriteError,
        })
    }
}

fn check_and_provide_canonical_path(
    output_path: &Path,
    template_path: &PathBuf,
) -> Result<PathBuf, CarpathiaError> {
    // 1. output_path is absolute or relative
    let output_path = if output_path.is_absolute() {
        output_path.to_path_buf()
    } else {
        std::env::current_dir()
            .expect("Failed to get current directory")
            .join(output_path)
    };

    // 2. template_path:
    //    - if asolute treat it as relative to the output_path
    //    - if absolute just add it to the output_path (e.g. /home/joan/templates → output_path/home/joan/templates)
    let full_template_path = if template_path.is_absolute() {
        // Transfer absolute to relative by stripping the leading `/` and joining with output_path
        // z. B. `/etc/passwd` → `output_path/etc/passwd`
        let stripped = template_path.strip_prefix("/").unwrap_or(template_path);
        output_path.join(stripped)
    } else {
        // Relative path joined with output_path
        output_path.join(template_path)
    };

    // 3. Crawls upward to check if the path contains `..` → Reject
    if full_template_path
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err(CarpathiaError {
            message: format!(
                "Path traversal (..) is not allowed: {:?}",
                full_template_path
            ),
            error_type: ErrorNumber::PathEscapesOutputDir,
        });
    }

    // 4. Create missirng directories if necessary (e.g. for absolute paths that are being replicated under output_path)
    if let Some(parent_dir) = full_template_path.parent() {
        std::fs::create_dir_all(parent_dir).map_err(|e| CarpathiaError {
            message: format!("Failed to create parent directories: {}", e),
            error_type: ErrorNumber::PathCanonicalizationError,
        })?;
    }

    // 5. check if the path is within the output_path
    //    (without canonicalize: simple string comparison)
    if !full_template_path.starts_with(&*output_path) {
        return Err(CarpathiaError {
            message: format!(
                "Template path escapes output directory: {:?} (resolved to: {:?})",
                template_path, full_template_path
            ),
            error_type: ErrorNumber::PathEscapesOutputDir,
        });
    }

    Ok(full_template_path)
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
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    struct TestCase {
        file_path: PathBuf,
        expected_template_type: TemplateType,
        expected_suffix: String,
        expected_file_name: String,
        error_message: String,
    }

    #[test]
    fn test_absolute_path_replication() {
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let output_path = temp_dir.path().join("output").as_path().to_owned();
        std::fs::create_dir_all(&output_path).expect("Failed to create output directory");
        // Testfälle mit absoluten und relativen Pfaden
        let test_cases = vec![
            // Relativer Pfad
            (PathBuf::from("summary.index.html.tera"), true),
            // Absoluter Pfad (wird unter output_path repliziert)
            (PathBuf::from("/etc/passwd/views.index.html.tera"), true),
            // Pfad mit Unterverzeichnis
            (PathBuf::from("subdir/file.html.tera"), true),
            // Pfad mit `..` (sollte fehlschlagen)
            (PathBuf::from("../../../etc/passwd"), false),
            // Absoluter Pfad mit `..` (sollte fehlschlagen)
            (PathBuf::from("/../etc/passwd"), false),
        ];

        for (template_path, should_succeed) in test_cases {
            let result = Template::new(&output_path, &template_path);
            if should_succeed {
                let template = result.expect("Expected success");
                assert!(
                    template.file_path.starts_with(&output_path),
                    "Path should be within output_path: {:?}",
                    template.file_path
                );
            } else {
                assert!(
                    result.is_err(),
                    "Path traversal should fail: {:?}",
                    template_path
                );
            }
        }
    }

    #[test]
    fn test_template_path_traversal() {
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let output_path = temp_dir.path();

        // Temporäre Dateien erstellen
        let test_cases = vec![
            // summary
            TestCase {
                file_path: PathBuf::from("summary.index.html.tera"),
                expected_template_type: TemplateType::Summary,
                expected_suffix: "html".to_string(),
                expected_file_name: "index".to_string(),
                error_message: "Failed to create summary template".to_string(),
            },
            TestCase {
                file_path: PathBuf::from("views.index.html.tera"),
                expected_template_type: TemplateType::View,
                expected_suffix: "html".to_string(),
                expected_file_name: "index".to_string(),
                error_message: "Failed to create view template".to_string(),
            },
            TestCase {
                file_path: PathBuf::from("views.index.chicken.html.tera"),
                expected_template_type: TemplateType::View,
                expected_suffix: "html".to_string(),
                expected_file_name: "index_chicken".to_string(),
                error_message: "Failed to create view template with multiple name tokens"
                    .to_string(),
            },
            TestCase {
                file_path: PathBuf::from("views.html.tera"),
                expected_template_type: TemplateType::View,
                expected_suffix: "html".to_string(),
                expected_file_name: "".to_string(),
                error_message: "Failed to create view template with multiple name tokens"
                    .to_string(),
            },
        ];

        for test_case in test_cases {
            // Datei im temporären Verzeichnis erstellen
            let full_path = output_path.join(&test_case.file_path);
            std::fs::create_dir_all(full_path.parent().unwrap())
                .expect("Failed to create parent directory");
            std::fs::File::create(&full_path).expect("Failed to create test file");

            let template = match Template::new(output_path, &test_case.file_path) {
                Ok(template) => template,
                Err(e) => panic!(
                    "Failed to create template for test case: {}, error: {}",
                    test_case.error_message, e
                ),
            };

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

    #[test]
    fn test_file_writing() {
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let output_path = temp_dir.path();

        // Try to write a rendered template using the Template struct
        let template = Template {
            template_type: TemplateType::View,
            file_path: output_path.join("views.index.html.tera"),
            file_name: "index".to_string(),
            suffix: "html".to_string(),
        };

        let rendered_content = "<html><body>Test</body></html>";
        let db_object_name = "test_object";

        // Writing...
        match template.write_rendered_template(rendered_content, db_object_name) {
            Ok(_) => {
                let expected_file_path = output_path
                    .join("views.index.html.tera")
                    .parent()
                    .unwrap()
                    .join(format!(
                        "{}{}.{}",
                        db_object_name, template.file_name, template.suffix
                    ));
                assert!(
                    expected_file_path.exists(),
                    "Expected file was not created: {:?}",
                    expected_file_path
                );
                let content = std::fs::read_to_string(expected_file_path)
                    .expect("Failed to read written file");
                assert_eq!(content, rendered_content, "File content does not match");
            }
            Err(e) => panic!("Failed to write rendered template: {}", e),
        }
    }
}
