use openapi_to_loco_scaf::analyzer::{OpenAPIAnalyzer, ResourceInfo, FieldType};
use openapi_to_loco_scaf::generator::LocoScaffoldGenerator;
use openapiv3::OpenAPI;
use serde_json;
use tempfile::tempdir;
use std::fs;

// テスト用のOpenAPI仕様
const TEST_OPENAPI_JSON: &str = r#"
{
  "openapi": "3.0.0",
  "info": {
    "title": "Test API",
    "version": "1.0.0"
  },
  "components": {
    "schemas": {
      "User": {
        "type": "object",
        "required": ["name", "email"],
        "properties": {
          "id": {
            "type": "integer"
          },
          "name": {
            "type": "string"
          },
          "email": {
            "type": "string",
            "format": "email"
          },
          "bio": {
            "type": "string",
            "maxLength": 500
          }
        }
      },
      "Post": {
        "type": "object",
        "required": ["title", "user_id"],
        "properties": {
          "id": {
            "type": "integer"
          },
          "title": {
            "type": "string"
          },
          "content": {
            "type": "string",
            "maxLength": 2000
          },
          "user_id": {
            "type": "integer"
          },
          "created_at": {
            "type": "string",
            "format": "date-time"
          }
        }
      }
    }
  },
  "paths": {
    "/users": {
      "get": {
        "summary": "List users",
        "responses": {
          "200": {
            "description": "OK"
          }
        }
      },
      "post": {
        "summary": "Create user",
        "responses": {
          "201": {
            "description": "Created"
          }
        }
      }
    },
    "/users/{id}": {
      "get": {
        "summary": "Get user",
        "responses": {
          "200": {
            "description": "OK"
          }
        }
      },
      "put": {
        "summary": "Update user",
        "responses": {
          "200": {
            "description": "OK"
          }
        }
      },
      "delete": {
        "summary": "Delete user",
        "responses": {
          "204": {
            "description": "No Content"
          }
        }
      }
    },
    "/posts": {
      "get": {
        "summary": "List posts",
        "responses": {
          "200": {
            "description": "OK"
          }
        }
      },
      "post": {
        "summary": "Create post",
        "responses": {
          "201": {
            "description": "Created"
          }
        }
      }
    }
  }
}
"#;

#[test]
fn test_full_pipeline() {
    // OpenAPI仕様をパース
    let openapi: OpenAPI = serde_json::from_str(TEST_OPENAPI_JSON)
        .expect("Failed to parse test OpenAPI JSON");

    // 解析実行
    let analyzer = OpenAPIAnalyzer::new(openapi);
    let resources = analyzer.analyze()
        .expect("Failed to analyze OpenAPI");

    // リソースが検出されることを確認
    assert!(!resources.is_empty(), "No resources detected");
    
    // User リソースの確認
    let user_resource = resources.iter()
        .find(|r| r.name == "users")
        .expect("Users resource not found");
    
    assert_eq!(user_resource.name, "users");
    assert!(user_resource.operations.contains(&"GET".to_string()));
    assert!(user_resource.operations.contains(&"POST".to_string()));
    assert!(user_resource.operations.contains(&"PUT".to_string()));
    assert!(user_resource.operations.contains(&"DELETE".to_string()));

    // User フィールドの確認
    assert!(user_resource.fields.len() >= 4); // id, name, email, bio
    
    let name_field = user_resource.fields.iter()
        .find(|f| f.name == "name")
        .expect("name field not found");
    assert!(matches!(name_field.field_type, FieldType::String));
    assert!(name_field.required);

    let email_field = user_resource.fields.iter()
        .find(|f| f.name == "email")
        .expect("email field not found");
    assert!(matches!(email_field.field_type, FieldType::String));
    assert!(email_field.required);

    // Post リソースの確認
    let post_resource = resources.iter()
        .find(|r| r.name == "posts")
        .expect("Posts resource not found");
    
    assert_eq!(post_resource.name, "posts");
    
    // Post フィールドの確認 - 参照関係
    let user_id_field = post_resource.fields.iter()
        .find(|f| f.name == "user_id")
        .expect("user_id field not found");
    
    if let FieldType::References(ref_table) = &user_id_field.field_type {
        assert_eq!(ref_table, "user");
    } else {
        panic!("user_id should be a reference field");
    }

    // コマンド生成テスト
    let generator = LocoScaffoldGenerator::new();
    let commands = generator.generate_commands(&resources, true);
    
    assert!(!commands.is_empty(), "No commands generated");
    
    // Users コマンドの確認
    let user_command = commands.iter()
        .find(|cmd| cmd.contains("users"))
        .expect("Users scaffold command not found");
    
    assert!(user_command.contains("--api"));
    assert!(user_command.contains("name:string!"));
    assert!(user_command.contains("email:string^"));

    // Posts コマンドの確認
    let post_command = commands.iter()
        .find(|cmd| cmd.contains("posts"))
        .expect("Posts scaffold command not found");
    
    assert!(post_command.contains("--api"));
    assert!(post_command.contains("title:string!"));
    assert!(post_command.contains("user_id:references:user!"));
}

#[test]
fn test_batch_script_generation() {
    let openapi: OpenAPI = serde_json::from_str(TEST_OPENAPI_JSON)
        .expect("Failed to parse test OpenAPI JSON");

    let analyzer = OpenAPIAnalyzer::new(openapi);
    let resources = analyzer.analyze()
        .expect("Failed to analyze OpenAPI");

    let generator = LocoScaffoldGenerator::new();
    let script = generator.generate_batch_script(&resources, true);

    // スクリプトの基本構造確認
    assert!(script.contains("#!/bin/bash"));
    assert!(script.contains("set -e"));
    assert!(script.contains("cargo loco generate scaffold"));
    assert!(script.contains("--api"));
    assert!(script.contains("cargo loco db migrate"));
    assert!(script.contains("cargo loco db entities"));
}

#[test]
fn test_markdown_report_generation() {
    let openapi: OpenAPI = serde_json::from_str(TEST_OPENAPI_JSON)
        .expect("Failed to parse test OpenAPI JSON");

    let analyzer = OpenAPIAnalyzer::new(openapi);
    let resources = analyzer.analyze()
        .expect("Failed to analyze OpenAPI");

    let generator = LocoScaffoldGenerator::new();
    let report = generator.generate_report(&resources);

    // レポートの基本構造確認
    assert!(report.contains("# OpenAPI to Loco Scaffold Generation Report"));
    assert!(report.contains("## Resources Overview"));
    assert!(report.contains("### User")); // Titleケース
    assert!(report.contains("### Post")); // Titleケース
    assert!(report.contains("| Field | Type | Required | Unique | Reference |"));
    assert!(report.contains("```bash"));
}

#[test]
fn test_json_serialization() {
    let openapi: OpenAPI = serde_json::from_str(TEST_OPENAPI_JSON)
        .expect("Failed to parse test OpenAPI JSON");

    let analyzer = OpenAPIAnalyzer::new(openapi);
    let resources = analyzer.analyze()
        .expect("Failed to analyze OpenAPI");

    // JSON シリアル化/デシリアル化テスト
    let json_str = serde_json::to_string_pretty(&resources)
        .expect("Failed to serialize to JSON");
    
    assert!(json_str.contains("\"name\":"));
    assert!(json_str.contains("\"fields\":"));
    assert!(json_str.contains("\"operations\":"));

    // デシリアル化テスト
    let deserialized_resources: Vec<ResourceInfo> = serde_json::from_str(&json_str)
        .expect("Failed to deserialize from JSON");
    
    assert_eq!(deserialized_resources.len(), resources.len());
    assert_eq!(deserialized_resources[0].name, resources[0].name);
}

#[test]
fn test_file_operations() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let test_file = temp_dir.path().join("test_openapi.json");
    
    // テストファイル作成
    fs::write(&test_file, TEST_OPENAPI_JSON)
        .expect("Failed to write test file");

    // ファイルからの読み込みテスト
    let content = fs::read_to_string(&test_file)
        .expect("Failed to read test file");
    
    let openapi: OpenAPI = serde_json::from_str(&content)
        .expect("Failed to parse OpenAPI from file");

    let analyzer = OpenAPIAnalyzer::new(openapi);
    let resources = analyzer.analyze()
        .expect("Failed to analyze OpenAPI from file");

    assert!(!resources.is_empty(), "No resources detected from file");
}

// プライベートメソッドのテストは統合テストのスコープを超えるため削除
// 代わりに公開APIを通じて動作を検証