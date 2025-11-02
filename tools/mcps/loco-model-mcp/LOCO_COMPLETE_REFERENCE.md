# Loco.rs 完全リファレンスガイド

## 概要

Loco.rsフレームワークにおけるモデル・コントローラー・スキャフォールド生成のための完全なリファレンスドキュメント。CLIコマンド、データタイプ、ベストプラクティス、MCP実装ガイドを包括的にカバー。

**調査日**: 2025-10-18
**対象バージョン**: Loco.rs 2025年版（最新安定版）

---

## 目次

1. [コマンド体系](#1-コマンド体系)
2. [モデル生成](#2-モデル生成)
3. [コントローラー生成](#3-コントローラー生成)
4. [スキャフォールド生成](#4-スキャフォールド生成)
5. [データタイプ完全リファレンス](#5-データタイプ完全リファレンス)
6. [フィールド修飾子](#6-フィールド修飾子)
7. [外部キー参照](#7-外部キー参照)
8. [SeaORM統合](#8-seaorm統合)
9. [MCP実装ガイド](#9-mcp実装ガイド)
10. [ベストプラクティス](#10-ベストプラクティス)
11. [FAQ](#11-faq)

---

## 1. コマンド体系

### 1.1 基本コマンド構造

Locoは3つの主要なジェネレーターコマンドを提供:

```bash
cargo loco generate model <name> [field:type ...]
cargo loco generate controller <name> [OPTIONS]
cargo loco generate scaffold <name> [field:type ...] [OPTIONS]
```

**短縮形**: `cargo loco g` も使用可能

### 1.2 コマンド比較表

| コマンド | 生成物 | 用途 | CRUD実装 |
|---------|--------|------|----------|
| `model` | モデル + マイグレーション + テスト | データ構造のみ定義 | ❌ |
| `controller` | コントローラー + テスト | ルーティング/ビジネスロジック | ❌ |
| `scaffold` | モデル + コントローラー + マイグレーション + テスト | フルスタックCRUD | ✅ |

---

## 2. モデル生成

### 2.1 基本構文

```bash
cargo loco generate model <name> [field:type ...]
```

**エイリアス**: `cargo loco g model`

### 2.2 使用例

#### 基本的なモデル

```bash
# シンプルなモデル
cargo loco g model article title:string content:text

# 複数フィールド
cargo loco g model user \
    email:string! \
    username:string^ \
    full_name:string \
    is_active:bool!
```

#### 外部キー参照

```bash
# デフォルトフィールド名（user_id）
cargo loco g model post title:string! user:references

# カスタムフィールド名（authored_by）
cargo loco g model post title:string! user:references:authored_by

# 複数の参照
cargo loco g model comment \
    content:text! \
    post:references \
    user:references:author_id
```

### 2.3 生成されるファイル

1. **マイグレーションファイル** - `migration/src/m{timestamp}_{model_name}.rs`
2. **エンティティ** - `src/models/_entities/{model_name}.rs`
3. **テストファイル** - `tests/models/{model_name}.rs`
4. **自動登録** - `src/models/lib.rs` への自動追加

### 2.4 Migration-Firstアプローチ

Locoは **migration-first framework**:

```
1. マイグレーション作成 → 2. マイグレーション適用 → 3. エンティティ自動生成
```

**重要**: モデルファイルを直接編集せず、常にマイグレーションを通じて変更する。

---

## 3. コントローラー生成

### 3.1 基本構文

```bash
cargo loco generate controller <name> [OPTIONS]
```

### 3.2 オプション

| オプション | 説明 | 生成形式 |
|-----------|------|---------|
| `--api` | RESTful APIコントローラー | JSON応答 |
| `--html` | HTML描画コントローラー | HTMLビュー |
| `--htmx` | HTMX対応コントローラー | HTMXパーシャル |

### 3.3 使用例

```bash
# APIコントローラー
cargo loco g controller users --api

# HTMLコントローラー
cargo loco g controller pages --html

# HTMXコントローラー
cargo loco g controller dashboard --htmx
```

### 3.4 生成されるファイル

1. **コントローラー** - `src/controllers/{name}.rs`
2. **テスト** - `tests/requests/{name}.rs`
3. **自動登録** - `src/app.rs` へのルート登録

---

## 4. スキャフォールド生成

### 4.1 基本構文

```bash
cargo loco generate scaffold <name> [field:type ...] [OPTIONS]
```

**スキャフォールド = モデル + コントローラー + 完全CRUD実装**

### 4.2 オプション

`--api`, `--html`, `--htmx` （コントローラーと同様）

### 4.3 使用例

#### 基本的なスキャフォールド

```bash
# APIスキャフォールド
cargo loco g scaffold post title:string! content:text --api

# HTMLスキャフォールド
cargo loco g scaffold product \
    name:string! \
    price:decimal! \
    stock:int! \
    --html
```

#### 外部キー参照を含むスキャフォールド

```bash
# コメントシステム
cargo loco g scaffold comment \
    content:text! \
    article:references \
    user:references \
    --api

# 注文システム
cargo loco g scaffold order \
    order_number:string! \
    total_amount:money! \
    status:string! \
    user:references \
    product:references \
    --api
```

### 4.4 生成されるファイル（完全セット）

| カテゴリ | ファイル | 内容 |
|---------|---------|------|
| **モデル** | `src/models/{name}.rs` | データモデル定義 |
| **エンティティ** | `src/models/_entities/{name}.rs` | SeaORM自動生成エンティティ |
| **マイグレーション** | `migration/src/m{timestamp}_{name}.rs` | DBスキーマ変更 |
| **コントローラー** | `src/controllers/{name}.rs` | **CRUD実装済み** |
| **テスト** | `tests/models/{name}.rs` | モデルテスト |
| **テスト** | `tests/requests/{name}.rs` | コントローラーテスト |
| **自動登録** | `src/app.rs`, `src/models/lib.rs` | ルート/モデル登録 |

### 4.5 生成されるCRUD操作

スキャフォールドは以下のエンドポイントを自動生成（APIの場合）:

```
GET    /api/{resources}          # index - 一覧取得
GET    /api/{resources}/:id      # show - 詳細取得
POST   /api/{resources}          # create - 新規作成
PUT    /api/{resources}/:id      # update - 更新
DELETE /api/{resources}/:id      # delete - 削除
```

---

## 5. データタイプ完全リファレンス

### 5.1 UUID型

| Loco型 | SeaORM | 説明 |
|--------|--------|------|
| `uuid` | `Uuid` | NULL許可UUID |
| `uuid!` | `Uuid` | 必須UUID |
| `uuid^` | `Uuid` | ユニークUUID |

```bash
cargo loco g model user id:uuid! external_id:uuid
```

### 5.2 文字列型

#### String（可変長）

| Loco型 | SeaORM | データベース型 |
|--------|--------|---------------|
| `string` | `String(StringLen)` | VARCHAR |
| `string!` | `String(StringLen)` | VARCHAR NOT NULL |
| `string^` | `String(StringLen)` | VARCHAR UNIQUE |

#### Text（長文）

| Loco型 | SeaORM | データベース型 |
|--------|--------|---------------|
| `text` | `Text` | TEXT |
| `text!` | `Text` | TEXT NOT NULL |
| `text^` | `Text` | TEXT UNIQUE |

```bash
cargo loco g model post \
    title:string! \
    slug:string^ \
    content:text \
    summary:text
```

### 5.3 整数型

#### 符号付き整数

| Loco型 | SeaORM | ビット数 | 範囲 |
|--------|--------|---------|------|
| `int` | `Integer` | 32bit | -2,147,483,648 ~ 2,147,483,647 |
| `small_int` | `SmallInteger` | 16bit | -32,768 ~ 32,767 |
| `big_int` | `BigInteger` | 64bit | -9,223,372,036,854,775,808 ~ 9,223,372,036,854,775,807 |

#### 符号なし整数

| Loco型 | SeaORM | ビット数 | 範囲 |
|--------|--------|---------|------|
| `small_unsigned` | `SmallUnsigned` | 16bit | 0 ~ 65,535 |
| `big_unsigned` | `BigUnsigned` | 64bit | 0 ~ 18,446,744,073,709,551,615 |

**各型に `!` と `^` 修飾子が使用可能**

```bash
cargo loco g model product \
    price:int! \
    stock:small_unsigned! \
    views:big_unsigned \
    order_count:int
```

### 5.4 浮動小数点型

| Loco型 | SeaORM | 精度 |
|--------|--------|------|
| `float` | `Float` | 単精度（32bit） |
| `double` | `Double` | 倍精度（64bit） |

```bash
cargo loco g model measurement \
    temperature:float! \
    precision_value:double!
```

### 5.5 固定小数点型（Decimal）

| Loco型 | SeaORM | 説明 |
|--------|--------|------|
| `decimal` | `Decimal(Option<(u32, u32)>)` | 基本decimal |
| `decimal_len` | `Decimal(Some((precision, scale)))` | 精度指定decimal |

```bash
# 基本使用
cargo loco g model product price:decimal! discount:decimal

# 精度指定（マイグレーションで設定）
# DecimalLenNull(10, 2) = 全体10桁、小数点以下2桁
```

**マイグレーション例**:
```rust
add_column(m, "products", "price", ColType::DecimalLenNull(10, 2)).await?;
```

### 5.6 真偽値型

| Loco型 | SeaORM | 値 |
|--------|--------|-----|
| `bool` | `Boolean` | true/false |
| `bool!` | `Boolean` | true/false (NOT NULL) |
| `bool^` | `Boolean` | true/false (UNIQUE) |

```bash
cargo loco g model user \
    is_active:bool! \
    is_verified:bool \
    is_admin:bool!
```

### 5.7 日時型

#### Timestamp（タイムスタンプ）

| Loco型 | SeaORM | 説明 |
|--------|--------|------|
| `ts` | `Timestamp` | UNIX timestamp |
| `ts!` | `Timestamp` | UNIX timestamp (NOT NULL) |

**自動フィールド**: すべてのモデルに `created_at:ts!` と `updated_at:ts!` が自動追加

#### DateTime（日時）

| Loco型 | SeaORM | フォーマット |
|--------|--------|-------------|
| `datetime` | `DateTime` | YYYY-MM-DD HH:MM:SS |
| `datetime!` | `DateTime` | YYYY-MM-DD HH:MM:SS (NOT NULL) |

#### Date（日付）

| Loco型 | SeaORM | フォーマット |
|--------|--------|-------------|
| `date` | `Date` | YYYY-MM-DD |
| `date!` | `Date` | YYYY-MM-DD (NOT NULL) |

#### Time（時刻）

| Loco型 | SeaORM | フォーマット |
|--------|--------|-------------|
| `time` | `Time` | HH:MM:SS |
| `time!` | `Time` | HH:MM:SS (NOT NULL) |

```bash
cargo loco g model event \
    name:string! \
    starts_at:datetime! \
    ends_at:datetime! \
    published_date:date \
    reminder_time:time
```

### 5.8 バイナリ型

| Loco型 | SeaORM | 用途 |
|--------|--------|------|
| `blob` | `Blob` | バイナリデータ（画像、ファイル等） |
| `blob!` | `Blob` | バイナリデータ（NOT NULL） |

```bash
cargo loco g model file \
    name:string! \
    data:blob! \
    thumbnail:blob
```

### 5.9 金額型

| Loco型 | SeaORM | 説明 |
|--------|--------|------|
| `money` | `Money(Option<(u32, u32)>)` | 金額専用型 |
| `money!` | `Money(Option<(u32, u32)>)` | 金額専用型（NOT NULL） |

```bash
cargo loco g model transaction \
    amount:money! \
    fee:money \
    balance:money!
```

### 5.10 JSON型（手動設定）

**注意**: Locoジェネレーターは直接サポートしていません。マイグレーションで手動設定が必要。

```rust
// マイグレーションでJSON列追加
manager.alter_table(
    Table::alter()
        .table(Users::Table)
        .add_column(ColumnDef::new(Users::Metadata).json())
        .to_owned()
).await?;
```

### 5.11 その他のSeaORM型（手動設定のみ）

以下の型はマイグレーションで手動設定可能:

| SeaORM型 | 説明 |
|---------|------|
| `Char(Option<u32>)` | 固定長文字 |
| `TinyInteger` | 8bit整数 |
| `TinyUnsigned` | 8bit符号なし整数 |
| `Unsigned` | 符号なし整数 |
| `TimestampWithTimeZone` | タイムゾーン付きタイムスタンプ |
| `Year` | 年のみ |
| `Interval` | 時間間隔（PostgreSQL） |
| `Binary(u32)` | 固定長バイナリ |
| `VarBinary(StringLen)` | 可変長バイナリ |
| `Bit(Option<u32>)` | ビット列 |
| `VarBit(u32)` | 可変長ビット列 |
| `Enum` | 列挙型 |
| `Array` | 配列型（PostgreSQL） |
| `Vector` | ベクトル型（PostgreSQL） |
| `Cidr` | CIDR型（PostgreSQL） |
| `Inet` | INET型（PostgreSQL） |
| `MacAddr` | MACアドレス型（PostgreSQL） |
| `LTree` | ラベルツリー型（PostgreSQL） |

---

## 6. フィールド修飾子

### 6.1 修飾子一覧

| 修飾子 | 意味 | SQL制約 | 例 |
|--------|------|---------|-----|
| なし | Nullable | NULL許可 | `email:string` |
| `!` | Required | NOT NULL | `email:string!` |
| `^` | Unique | UNIQUE | `email:string^` |

### 6.2 使用パターン

```bash
# NULL許可
cargo loco g model user nickname:string

# 必須フィールド
cargo loco g model user email:string!

# ユニークフィールド
cargo loco g model user username:string^

# 複合（すべてのパターン）
cargo loco g model user \
    email:string! \
    username:string^ \
    bio:text \
    is_active:bool!
```

---

## 7. 外部キー参照

### 7.1 基本構文

```
<model>:references                    # デフォルトフィールド名
<model>:references:<field_name>       # カスタムフィールド名
```

### 7.2 デフォルト参照

```bash
# user_id フィールドが生成される
cargo loco g model post \
    title:string! \
    user:references
```

**生成されるフィールド**: `user_id: i32`

### 7.3 カスタムフィールド名

```bash
# author_id フィールドが生成される
cargo loco g model post \
    title:string! \
    user:references:author_id
```

**生成されるフィールド**: `author_id: i32`

### 7.4 複数参照

```bash
cargo loco g scaffold comment \
    content:text! \
    post:references \
    user:references:author_id \
    parent:references:parent_comment_id \
    --api
```

### 7.5 参照関係の例

#### ブログシステム

```bash
# ユーザー
cargo loco g model user \
    email:string! \
    username:string^

# 投稿
cargo loco g model post \
    title:string! \
    content:text \
    user:references:author_id

# コメント
cargo loco g model comment \
    content:text! \
    post:references \
    user:references:commenter_id
```

---

## 8. SeaORM統合

### 8.1 ColumnType完全リスト

SeaORMは39種類のColumnTypeをサポート:

```rust
pub enum ColumnType {
    Char(Option<u32>),
    String(StringLen),
    Text,
    TinyInteger,
    SmallInteger,
    Integer,
    BigInteger,
    TinyUnsigned,
    SmallUnsigned,
    Unsigned,
    BigUnsigned,
    Float,
    Double,
    Decimal(Option<(u32, u32)>),
    DateTime,
    Timestamp,
    TimestampWithTimeZone,
    Time,
    Date,
    Year,
    Interval(Option<PgInterval>, Option<u32>),
    Binary(u32),
    VarBinary(StringLen),
    Bit(Option<u32>),
    VarBit(u32),
    Boolean,
    Money(Option<(u32, u32)>),
    Json,
    JsonBinary,
    Uuid,
    Custom(SeaRc<dyn Iden>),
    Enum { name: SeaRc<dyn Iden>, variants: Vec<SeaRc<dyn Iden>> },
    Array(Arc<ColumnType>),
    Vector(Option<u32>),
    Cidr,
    Inet,
    MacAddr,
    LTree,
    Blob,
}
```

### 8.2 機能フラグ

追加型サポートのための`Cargo.toml`設定:

```toml
[dependencies]
sea-orm = { version = "1.0", features = [
    "with-chrono",        # Chrono日時型
    "with-time",          # Time日時型
    "with-json",          # JSON型
    "with-rust_decimal",  # Decimal型
    "with-bigdecimal",    # BigDecimal型
    "with-uuid",          # UUID型
    "postgres-array",     # PostgreSQL配列型
] }
```

### 8.3 データベース別型マッピング

#### SQLite

| Loco/SeaORM型 | SQLite保存形式 |
|---------------|----------------|
| `decimal`, `money` | `REAL` |
| `json`, `uuid`, `datetime` | `TEXT` |
| `blob` | `BLOB` |
| `int`, `big_int` | `INTEGER` |

#### PostgreSQL

| Loco/SeaORM型 | PostgreSQL型 |
|---------------|-------------|
| `uuid` | `UUID` |
| `json` | `JSON` or `JSONB` |
| `decimal` | `NUMERIC` |
| `money` | `MONEY` |
| `array` | `ARRAY` |
| `cidr` | `CIDR` |

---

## 9. MCP実装ガイド

### 9.1 必要なツール

MCP サーバーは3つの主要ツールを実装:

#### 1. generate_model

```rust
#[tool(description = "Generate a Loco model with migrations")]
fn generate_model(
    &self,
    name: String,                    // モデル名
    fields: Vec<Field>,              // フィールド定義
) -> Result<CallToolResult, Error>
```

#### 2. generate_controller

```rust
#[tool(description = "Generate a Loco controller")]
fn generate_controller(
    &self,
    name: String,                    // コントローラー名
    format: ControllerFormat,        // api/html/htmx
) -> Result<CallToolResult, Error>
```

#### 3. generate_scaffold

```rust
#[tool(description = "Generate a complete CRUD scaffold")]
fn generate_scaffold(
    &self,
    name: String,                    // リソース名
    fields: Vec<Field>,              // フィールド定義
    format: ControllerFormat,        // api/html/htmx
) -> Result<CallToolResult, Error>
```

### 9.2 データ構造定義

```rust
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub modifier: Option<FieldModifier>,
    pub reference: Option<ReferenceConfig>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    // UUID
    Uuid,

    // 文字列
    String,
    Text,

    // 整数
    Int,
    SmallInt,
    BigInt,
    SmallUnsigned,
    BigUnsigned,

    // 浮動小数点
    Float,
    Double,

    // 固定小数点
    Decimal,
    DecimalLen { precision: u32, scale: u32 },

    // 真偽値
    Bool,

    // 日時
    Ts,
    DateTime,
    Date,
    Time,

    // バイナリ
    Blob,

    // 金額
    Money,

    // 参照
    References,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FieldModifier {
    Required,   // !
    Unique,     // ^
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReferenceConfig {
    pub model: String,
    pub field_name: Option<String>,  // None = {model}_id
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ControllerFormat {
    Api,
    Html,
    Htmx,
}
```

### 9.3 コマンド実行関数

```rust
use std::process::Command;
use std::path::Path;

pub struct LocoGenerator {
    project_root: String,
}

impl LocoGenerator {
    pub fn new(project_root: impl Into<String>) -> Self {
        Self {
            project_root: project_root.into(),
        }
    }

    fn execute_command(&self, args: Vec<String>) -> Result<String, Error> {
        let output = Command::new("cargo")
            .arg("loco")
            .args(&args)
            .current_dir(&self.project_root)
            .output()
            .map_err(|e| Error::ExecutionFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::CommandFailed(stderr.to_string()));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn generate_model(
        &self,
        name: &str,
        fields: &[Field],
    ) -> Result<String, Error> {
        let mut args = vec![
            "generate".to_string(),
            "model".to_string(),
            name.to_string(),
        ];

        for field in fields {
            args.push(self.field_to_arg(field)?);
        }

        self.execute_command(args)
    }

    pub fn generate_controller(
        &self,
        name: &str,
        format: &ControllerFormat,
    ) -> Result<String, Error> {
        let mut args = vec![
            "generate".to_string(),
            "controller".to_string(),
            name.to_string(),
        ];

        match format {
            ControllerFormat::Api => args.push("--api".to_string()),
            ControllerFormat::Html => args.push("--html".to_string()),
            ControllerFormat::Htmx => args.push("--htmx".to_string()),
        }

        self.execute_command(args)
    }

    pub fn generate_scaffold(
        &self,
        name: &str,
        fields: &[Field],
        format: &ControllerFormat,
    ) -> Result<String, Error> {
        let mut args = vec![
            "generate".to_string(),
            "scaffold".to_string(),
            name.to_string(),
        ];

        for field in fields {
            args.push(self.field_to_arg(field)?);
        }

        match format {
            ControllerFormat::Api => args.push("--api".to_string()),
            ControllerFormat::Html => args.push("--html".to_string()),
            ControllerFormat::Htmx => args.push("--htmx".to_string()),
        }

        self.execute_command(args)
    }

    fn field_to_arg(&self, field: &Field) -> Result<String, Error> {
        let type_str = match &field.field_type {
            FieldType::Uuid => "uuid",
            FieldType::String => "string",
            FieldType::Text => "text",
            FieldType::Int => "int",
            FieldType::SmallInt => "small_int",
            FieldType::BigInt => "big_int",
            FieldType::SmallUnsigned => "small_unsigned",
            FieldType::BigUnsigned => "big_unsigned",
            FieldType::Float => "float",
            FieldType::Double => "double",
            FieldType::Decimal => "decimal",
            FieldType::DecimalLen { .. } => "decimal_len",
            FieldType::Bool => "bool",
            FieldType::Ts => "ts",
            FieldType::DateTime => "datetime",
            FieldType::Date => "date",
            FieldType::Time => "time",
            FieldType::Blob => "blob",
            FieldType::Money => "money",
            FieldType::References => {
                if let Some(ref_config) = &field.reference {
                    return Ok(format!(
                        "{}:references{}",
                        ref_config.model,
                        ref_config.field_name
                            .as_ref()
                            .map(|f| format!(":{}", f))
                            .unwrap_or_default()
                    ));
                }
                return Err(Error::InvalidReference);
            }
        };

        let modifier_str = match field.modifier {
            Some(FieldModifier::Required) => "!",
            Some(FieldModifier::Unique) => "^",
            None => "",
        };

        Ok(format!("{}:{}{}", field.name, type_str, modifier_str))
    }
}
```

### 9.4 エラーハンドリング

```rust
#[derive(Debug)]
pub enum Error {
    ExecutionFailed(String),
    CommandFailed(String),
    InvalidReference,
    InvalidFieldType,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ExecutionFailed(msg) => write!(f, "Failed to execute command: {}", msg),
            Error::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
            Error::InvalidReference => write!(f, "Invalid reference configuration"),
            Error::InvalidFieldType => write!(f, "Invalid field type"),
        }
    }
}

impl std::error::Error for Error {}
```

---

## 10. ベストプラクティス

### 10.1 命名規則

#### モデル名

```bash
# Good: 単数形、小文字、スネークケース
cargo loco g model user
cargo loco g model blog_post
cargo loco g model product_category

# Bad: 複数形
cargo loco g model users  # ❌
```

**自動変換**: テーブル名は自動的に複数形に変換される
- `user` → `users`
- `blog_post` → `blog_posts`

#### フィールド名

```bash
# Good: スネークケース
cargo loco g model user \
    email_address:string! \
    full_name:string \
    created_by:int

# Bad: キャメルケース
cargo loco g model user emailAddress:string  # ❌
```

### 10.2 必須フィールドの選択

```bash
# Good: 重要なデータは必須
cargo loco g model user \
    email:string! \
    password_hash:string! \
    is_active:bool! \
    bio:text          # オプション

# Bad: すべて必須
cargo loco g model user \
    email:string! \
    bio:text!         # ❌ 常に必要ではない
```

### 10.3 ユニーク制約の使用

```bash
# Good: 重複を許可しないフィールド
cargo loco g model user \
    email:string^ \
    username:string^ \
    full_name:string  # 重複OK

# Bad: 不必要なユニーク制約
cargo loco g model user full_name:string^  # ❌
```

### 10.4 適切な数値型の選択

```bash
# Good: 用途に応じた型選択
cargo loco g model product \
    price:decimal! \        # 金額は正確な計算が必要
    rating:float \          # 評価は近似値でOK
    stock:int! \            # 在庫は整数
    view_count:big_int      # 大きな数値の可能性

# Bad: すべてint
cargo loco g model product \
    price:int!              # ❌ 小数点が必要
```

### 10.5 外部キー参照の命名

```bash
# Good: 明確なフィールド名
cargo loco g model post \
    user:references:author_id \
    category:references:category_id

# Acceptable: デフォルト名が明確な場合
cargo loco g model comment \
    post:references \       # post_id は明確
    user:references         # user_id は明確

# Bad: 曖昧なデフォルト名
cargo loco g model article \
    user:references         # ❌ 著者?編集者?レビュアー?
```

### 10.6 スキャフォールド vs 個別生成

#### スキャフォールドを使用すべき場合

```bash
# ✅ 標準的なCRUDが必要
cargo loco g scaffold product \
    name:string! \
    price:decimal! \
    --api

# ✅ プロトタイピング
cargo loco g scaffold user \
    email:string! \
    username:string^ \
    --api
```

#### 個別生成を使用すべき場合

```bash
# ✅ カスタムロジックが必要
cargo loco g model payment \
    amount:money! \
    status:string!

cargo loco g controller payments --api
# → カスタム決済ロジックを実装

# ✅ モデルのみ必要（バックグラウンド処理等）
cargo loco g model log_entry \
    level:string! \
    message:text!
```

---

## 11. FAQ

### Q1: JSON型フィールドを追加するには？

**A**: マイグレーションで手動追加:

```rust
use sea_orm_migration::prelude::*;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(ColumnDef::new(Users::Metadata).json())
                    .to_owned(),
            )
            .await
    }
}
```

### Q2: ENUM型を使用するには?

**A**: 手動マイグレーション + SeaORM設定が必要。[Discussion #854](https://github.com/loco-rs/loco/discussions/854) 参照。

```rust
// マイグレーション
manager.create_type(
    Type::create()
        .as_enum(UserRole::Type)
        .values([UserRole::Admin, UserRole::User])
        .to_owned(),
).await?;
```

### Q3: decimal_lenの精度を指定するには?

**A**: マイグレーションファイルで `DecimalLenNull` を使用:

```rust
add_column(
    m,
    "products",
    "price",
    ColType::DecimalLenNull(10, 2)  // 全体10桁、小数2桁
).await?;
```

### Q4: 既存フィールドを変更するには?

**A**: 新しいマイグレーションを作成:

```bash
# マイグレーション作成
cargo loco g migration update_user_email_to_unique

# マイグレーション編集
# → alter_column() を使用
```

### Q5: 複数のデータベースをサポートするには?

**A**: 条件付きマイグレーション:

```rust
use sea_orm_migration::prelude::*;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        match manager.get_database_backend() {
            sea_orm::DatabaseBackend::Postgres => {
                // PostgreSQL固有の処理
            }
            sea_orm::DatabaseBackend::Sqlite => {
                // SQLite固有の処理
            }
            _ => {}
        }
        Ok(())
    }
}
```

### Q6: 自動生成される `created_at` と `updated_at` を無効化するには?

**A**: 現在、自動生成を無効化する方法はありません。これらはLocoの設計思想の一部です。

### Q7: スキャフォールドで生成されたコントローラーをカスタマイズするには?

**A**: 生成後に直接編集:

```rust
// src/controllers/posts.rs
pub async fn index(State(ctx): State<AppContext>) -> Result<Response> {
    // カスタムロジック追加
    let custom_filter = /* ... */;

    // 既存のクエリを拡張
    let posts = Posts::find()
        .filter(custom_filter)
        .all(&ctx.db)
        .await?;

    format::json(posts)
}
```

### Q8: マイグレーションをロールバックするには?

**A**: 特定のバージョンまでロールバック:

```bash
# 直前のマイグレーションをロールバック
cargo loco db migrate down

# 特定のバージョンまでロールバック
cargo loco db migrate down --to 20250101000000
```

---

## 12. 参考リンク

### 公式ドキュメント

- [Loco.rs 公式サイト](https://loco.rs/)
- [Loco Models Documentation](https://loco.rs/docs/the-app/models/)
- [Loco Controllers Documentation](https://loco.rs/docs/the-app/controller/)
- [Loco Getting Started Guide](https://loco.rs/docs/getting-started/guide/)

### SeaORM

- [SeaORM 公式サイト](https://www.sea-ql.org/SeaORM/)
- [SeaORM ColumnType Documentation](https://docs.rs/sea-orm/latest/sea_orm/entity/enum.ColumnType.html)
- [SeaORM Column Types Guide](https://www.sea-ql.org/SeaORM/docs/generate-entity/column-types/)

### GitHub

- [Loco GitHub Repository](https://github.com/loco-rs/loco)
- [SeaORM GitHub Repository](https://github.com/SeaQL/sea-orm)

### コミュニティ

- [Loco GitHub Discussions](https://github.com/loco-rs/loco/discussions)
- [外部キー参照に関する議論 #1297](https://github.com/loco-rs/loco/discussions/1297)
- [モデル生成時の既存モデル変更 #1283](https://github.com/loco-rs/loco/discussions/1283)
- [ENUM型の使用方法 #854](https://github.com/loco-rs/loco/discussions/854)

### チュートリアル

- [Getting Started with Loco | Shuttle](https://www.shuttle.dev/blog/2023/12/28/using-loco-rust-rails)
- [Getting Started with Loco & SeaORM | SeaQL](https://www.sea-ql.org/blog/2024-05-28-getting-started-with-loco-seaorm/)

---

## 変更履歴

- **v1.0.0** (2025-10-18)
  - 初版リリース
  - モデル・コントローラー・スキャフォールド生成の完全ガイド
  - 全データタイプの網羅的なドキュメント化
  - SeaORM統合情報の追加
  - MCP実装ガイドの追加
  - ベストプラクティスとFAQの追加
