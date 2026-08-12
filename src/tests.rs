use super::*;
use serial_test::serial;
use std::env;
use tempfile::tempdir;

// Test are AI-generated

// --- Вспомогательные функции для тестов ---

/// Устанавливает XDG_*_HOME в указанную временную директорию.
/// Возвращает путь к этой директории.
fn setup_test_env() -> tempfile::TempDir {
    let dir = tempdir().expect("failed to create temp dir");
    let path = dir.path().to_str().unwrap().to_string();
    unsafe { env::set_var("XDG_DATA_HOME", &path) };
    unsafe { env::set_var("XDG_CONFIG_HOME", &path) };
    dir
}

/// Создаёт файл data.ron с заданным содержимым в директории программы.
async fn create_data_file(dir: &std::path::Path, content: &str) {
    let data_dir = dir.join("notes");
    tokio::fs::create_dir_all(&data_dir).await.unwrap();
    tokio::fs::write(data_dir.join("data.ron"), content)
        .await
        .unwrap();
}

/// Создаёт файл config.toml с заданным содержимым.
async fn create_config_file(dir: &std::path::Path, content: &str) {
    let config_dir = dir.join("notes");
    tokio::fs::create_dir_all(&config_dir).await.unwrap();
    tokio::fs::write(config_dir.join("config.toml"), content)
        .await
        .unwrap();
}

// --- Тесты для fsf ---

#[tokio::test]
#[serial]
async fn test_get_data_empty() {
    let _dir = setup_test_env();
    let data = fsf::get_data().await.unwrap();
    assert!(data.special.is_empty());
    assert!(data.common.is_empty());
}

#[tokio::test]
#[serial]
async fn test_get_data_existing() {
    let dir = setup_test_env();
    let ron_content = r#"(
        special: [
            (name: "Special1", content: Some("Content1")),
        ],
        common: [
            (name: "Common1", content: None),
        ],
    )"#;
    create_data_file(dir.path(), ron_content).await;

    let data = fsf::get_data().await.unwrap();
    assert_eq!(data.special.len(), 1);
    assert_eq!(data.special[0].name, "Special1");
    assert_eq!(data.special[0].content, Some("Content1".to_string()));
    assert_eq!(data.common.len(), 1);
    assert_eq!(data.common[0].name, "Common1");
    assert_eq!(data.common[0].content, None);
}

#[tokio::test]
#[serial]
async fn test_save_data() {
    let dir = setup_test_env();
    let root = logic::Root {
        special: vec![logic::Task {
            name: "Test".to_string(),
            content: Some("Content".to_string()),
        }],
        common: vec![],
    };
    fsf::save_data(root).await.unwrap();

    let data_path = dir.path().join("notes/data.ron");
    assert!(data_path.exists());
    let content = tokio::fs::read_to_string(&data_path).await.unwrap();
    // Проверяем, что в файле есть наши данные (можно проверить через десериализацию)
    let parsed: logic::Root = ron::from_str(&content).unwrap();
    assert_eq!(parsed.special.len(), 1);
    assert_eq!(parsed.special[0].name, "Test");
}

#[tokio::test]
#[serial]
async fn test_get_config_empty() {
    let _dir = setup_test_env();
    let config = fsf::get_config().await.unwrap();
    assert!(config.style.is_none());
}

#[tokio::test]
#[serial]
async fn test_get_config_existing() {
    let dir = setup_test_env();
    let toml_content = r#"
        [style]
        before_text = "BEFORE"
        after_text = "AFTER"
        before_specials = "==SPECIALS==\n"
        after_specials = "\n==END SPECIALS=="
    "#;
    create_config_file(dir.path(), toml_content).await;

    let config = fsf::get_config().await.unwrap();
    let style = config.style.unwrap();
    assert_eq!(style.before_text, "BEFORE");
    assert_eq!(style.after_text, "AFTER");
    assert_eq!(style.before_specials, "==SPECIALS==\n");
    assert_eq!(style.after_specials, "\n==END SPECIALS==");
}

// --- Тесты для logic ---

// Для тестов логики нам нужно, чтобы данные сохранялись во временную директорию.
// Используем тот же подход.

#[tokio::test]
#[serial]
async fn test_execute_new_common() {
    let _dir = setup_test_env();
    logic::execute_new("Task1".to_string(), Some("Desc".to_string()), false)
        .await
        .unwrap();

    let data = fsf::get_data().await.unwrap();
    assert_eq!(data.common.len(), 1);
    assert_eq!(data.common[0].name, "Task1");
    assert_eq!(data.common[0].content, Some("Desc".to_string()));
    assert!(data.special.is_empty());
}

#[tokio::test]
#[serial]
async fn test_execute_new_special() {
    let _dir = setup_test_env();
    logic::execute_new("Special".to_string(), None, true)
        .await
        .unwrap();

    let data = fsf::get_data().await.unwrap();
    assert_eq!(data.special.len(), 1);
    assert_eq!(data.special[0].name, "Special");
    assert_eq!(data.special[0].content, None);
    assert!(data.common.is_empty());
}

#[tokio::test]
#[serial]
async fn test_execute_delete_force_common() {
    let _dir = setup_test_env();
    // Сначала создаём задачу
    logic::execute_new("ToDelete".to_string(), None, false)
        .await
        .unwrap();

    // Удаляем с флагом force
    logic::execute_delete(0, false, true).await.unwrap();

    let data = fsf::get_data().await.unwrap();
    assert!(data.common.is_empty());
}

#[tokio::test]
#[serial]
async fn test_execute_delete_force_special() {
    let _dir = setup_test_env();
    logic::execute_new("SpecialDel".to_string(), None, true)
        .await
        .unwrap();

    logic::execute_delete(0, true, true).await.unwrap();

    let data = fsf::get_data().await.unwrap();
    assert!(data.special.is_empty());
}

// Тест execute_cat — нужны данные
#[tokio::test]
#[serial]
async fn test_execute_cat_common() {
    let _dir = setup_test_env();
    logic::execute_new("CatTask".to_string(), Some("Content".to_string()), false)
        .await
        .unwrap();

    // Проверяем, что функция не паникует и выводит (можно проверить вывод, но сложно)
    // Просто убедимся, что она возвращает Ok
    let result = logic::execute_cat(0, false).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[serial]
async fn test_execute_cat_special() {
    let _dir = setup_test_env();
    logic::execute_new(
        "SpecialCat".to_string(),
        Some("SpecialContent".to_string()),
        true,
    )
    .await
    .unwrap();

    let result = logic::execute_cat(0, true).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[serial]
async fn test_execute_list() {
    let _dir = setup_test_env();
    // Добавим несколько задач
    logic::execute_new("A".to_string(), None, false)
        .await
        .unwrap();
    logic::execute_new("B".to_string(), None, true)
        .await
        .unwrap();

    let result = logic::execute_list().await;
    assert!(result.is_ok());
}

#[tokio::test]
#[serial]
async fn test_execute_display_no_style() {
    let _dir = setup_test_env();
    logic::execute_new("Disp".to_string(), None, false)
        .await
        .unwrap();
    let result = logic::execute_display().await;
    assert!(result.is_ok());
}

#[tokio::test]
#[serial]
async fn test_execute_display_with_style() {
    let dir = setup_test_env();
    // Создаём конфиг со стилем
    let toml_content = r#"
        [style]
        before_text = "START\n"
        after_text = "END\n"
        before_specials = "--- Specials ---\n"
        after_specials = "--- End Specials ---\n"
        before_common_unit = "  * "
        after_common_unit = "\n"
    "#;
    create_config_file(dir.path(), toml_content).await;

    logic::execute_new("Styled".to_string(), None, false)
        .await
        .unwrap();
    let result = logic::execute_display().await;
    assert!(result.is_ok());
}

#[tokio::test]
#[serial]
async fn test_execute_printjson() {
    let _dir = setup_test_env();
    logic::execute_new("JsonTest".to_string(), Some("data".to_string()), false)
        .await
        .unwrap();

    let result = logic::execute_printjson().await;
    assert!(result.is_ok());
}

// Тесты сериализации/десериализации структур (опционально)
#[test]
fn test_task_serialization() {
    let task = logic::Task {
        name: "Test".to_string(),
        content: Some("Content".to_string()),
    };
    let serialized = ron::ser::to_string(&task).unwrap();
    let deserialized: logic::Task = ron::from_str(&serialized).unwrap();
    assert_eq!(task, deserialized);
}

#[test]
fn test_root_serialization() {
    let root = logic::Root {
        special: vec![],
        common: vec![logic::Task {
            name: "C".to_string(),
            content: None,
        }],
    };
    let serialized = ron::ser::to_string(&root).unwrap();
    let deserialized: logic::Root = ron::from_str(&serialized).unwrap();
    assert_eq!(root.special.len(), deserialized.special.len());
    assert_eq!(root.common.len(), deserialized.common.len());
}
