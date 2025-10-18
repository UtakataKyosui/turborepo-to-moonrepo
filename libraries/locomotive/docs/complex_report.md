# OpenAPI to Loco Scaffold Generation Report

Generated 10 resources from OpenAPI specification.

## Resources Overview

### Category

**Operations**: GET, POST

**Fields**:

| Field | Type | Required | Unique | Reference |
|-------|------|----------|--------|----------|
| id | int |  |  |  |
| name | string | ✓ |  |  |
| description | string |  |  |  |
| parent_id | references:parent |  |  | parent |
| slug | string | ✓ | ✓ |  |

**Generated Command**:
```bash
cargo loco generate scaffold category id:int name:string! description:string parent_id:references:parent slug:string^ --api
```

### Reviews

**Operations**: GET, POST

**Fields**:

| Field | Type | Required | Unique | Reference |
|-------|------|----------|--------|----------|
| id | int |  |  |  |
| user_id | references:user | ✓ |  | user |
| product_id | references:product | ✓ |  | product |
| rating | int | ✓ |  |  |
| comment | text |  |  |  |
| created_at | date_time |  |  |  |

**Relationships**:

- REVIEW → USER (N:1) - writes
- REVIEW → PRODUCT (N:1) - receives

**Generated Command**:
```bash
cargo loco generate scaffold reviews id:int user_id:references:user! product_id:references:product! rating:int! comment:text created_at:date_time --api
```

### Order Item

**Operations**: GET, POST

**Fields**:

| Field | Type | Required | Unique | Reference |
|-------|------|----------|--------|----------|
| id | int |  |  |  |
| order_id | references:order | ✓ |  | order |
| product_id | references:product | ✓ |  | product |
| quantity | int | ✓ |  |  |
| unit_price | decimal | ✓ |  |  |
| total_price | decimal |  |  |  |

**Generated Command**:
```bash
cargo loco generate scaffold order_item id:int order_id:references:order! product_id:references:product! quantity:int! unit_price:decimal! total_price:decimal --api
```

### Profile

**Operations**: GET, POST

**Fields**:

| Field | Type | Required | Unique | Reference |
|-------|------|----------|--------|----------|
| id | int |  |  |  |
| user_id | references:user | ✓ |  | user |
| first_name | string | ✓ |  |  |
| last_name | string | ✓ |  |  |
| phone | string |  |  |  |
| bio | text |  |  |  |
| avatar_url | string |  |  |  |

**Generated Command**:
```bash
cargo loco generate scaffold profile id:int user_id:references:user! first_name:string! last_name:string! phone:string bio:text avatar_url:string --api
```

### Users

**Operations**: GET, POST, GET, PUT, DELETE

**Fields**:

| Field | Type | Required | Unique | Reference |
|-------|------|----------|--------|----------|
| id | int |  |  |  |
| username | string | ✓ | ✓ |  |
| email | string | ✓ | ✓ |  |
| password | string | ✓ |  |  |
| created_at | date_time |  |  |  |
| updated_at | date_time |  |  |  |

**Relationships**:

- USER → ORDER (1:N) - places
- USER → REVIEW (1:N) - writes
- USER → PROFILE (1:1) - has
- USER → PRODUCT (N:N) - favorites
- USER → ADDRESS (1:N) - belongs_to
- USER → PAYMENT (1:N) - belongs_to

**Generated Command**:
```bash
cargo loco generate scaffold users id:int username:string^ email:string^ password:string! created_at:date_time updated_at:date_time --api
```

### Brand

**Operations**: GET, POST

**Fields**:

| Field | Type | Required | Unique | Reference |
|-------|------|----------|--------|----------|
| id | int |  |  |  |
| name | string | ✓ |  |  |
| description | string |  |  |  |
| logo_url | string |  |  |  |
| website | string |  |  |  |

**Generated Command**:
```bash
cargo loco generate scaffold brand id:int name:string! description:string logo_url:string website:string --api
```

### Orders

**Operations**: GET, POST, GET, PUT

**Fields**:

| Field | Type | Required | Unique | Reference |
|-------|------|----------|--------|----------|
| id | int |  |  |  |
| user_id | references:user | ✓ |  | user |
| address_id | references:address | ✓ |  | address |
| payment_id | references:payment |  |  | payment |
| status | string | ✓ |  |  |
| total_amount | decimal | ✓ |  |  |
| created_at | date_time |  |  |  |
| shipped_at | date_time |  |  |  |

**Relationships**:

- ORDER → USER (N:1) - places
- ORDER → ORDER_ITEM (1:N) - contains
- ORDER → ADDRESS (N:1) - shipped_to
- ORDER → PAYMENT (N:1) - processed_with

**Generated Command**:
```bash
cargo loco generate scaffold orders id:int user_id:references:user! address_id:references:address! payment_id:references:payment status:string! total_amount:decimal! created_at:date_time shipped_at:date_time --api
```

### Address

**Operations**: GET, POST

**Fields**:

| Field | Type | Required | Unique | Reference |
|-------|------|----------|--------|----------|
| id | int |  |  |  |
| user_id | references:user | ✓ |  | user |
| type | string | ✓ |  |  |
| street | string | ✓ |  |  |
| city | string | ✓ |  |  |
| state | string | ✓ |  |  |
| postal_code | string | ✓ |  |  |
| country | string | ✓ |  |  |
| is_default | bool |  |  |  |

**Generated Command**:
```bash
cargo loco generate scaffold address id:int user_id:references:user! type:string! street:string! city:string! state:string! postal_code:string! country:string! is_default:bool --api
```

### Categories

**Operations**: GET, POST

**Generated Command**:
```bash
cargo loco generate scaffold categories  --api
```

### Products

**Operations**: GET, POST, GET, PUT, DELETE

**Fields**:

| Field | Type | Required | Unique | Reference |
|-------|------|----------|--------|----------|
| id | int |  |  |  |
| name | string | ✓ |  |  |
| description | text |  |  |  |
| price | decimal | ✓ |  |  |
| stock_quantity | int |  |  |  |
| category_id | references:category | ✓ |  | category |
| brand_id | references:brand | ✓ |  | brand |
| sku | string | ✓ | ✓ |  |
| created_at | date_time |  |  |  |
| updated_at | date_time |  |  |  |

**Relationships**:

- PRODUCT → USER (N:N) - favorites
- PRODUCT → REVIEW (1:N) - receives
- PRODUCT → CATEGORY (N:1) - belongs_to
- PRODUCT → BRAND (N:1) - manufactured_by
- PRODUCT → TAG (N:N) - tagged_with

**Generated Command**:
```bash
cargo loco generate scaffold products id:int name:string! description:text price:decimal! stock_quantity:int category_id:references:category! brand_id:references:brand! sku:string^ created_at:date_time updated_at:date_time --api
```

## Notes

- All commands use `--api` flag for API-only scaffolding
- Reference relationships are detected based on field naming conventions (`*_id`, `*Id`)
- Field types are mapped from OpenAPI types to Loco types
- Required fields are marked with `!` suffix

---
*Generated by [openapi-to-loco-scaf](https://github.com/your-repo/openapi-to-loco-scaf)*
