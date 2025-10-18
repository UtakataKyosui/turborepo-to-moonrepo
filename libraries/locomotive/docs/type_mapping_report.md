# OpenAPI to Loco Scaffold Generation Report

Generated 13 resources from OpenAPI specification.

## Resources Overview

### Categories

**Operations**: GET

**Generated Command**:
```bash
cargo loco generate scaffold categories  --api
```

### Reviews

**Operations**: GET, POST

**Fields**:

| Field | Type | Required | Reference |
|-------|------|----------|----------|
| id | big_int | ✓ |  |
| user_id | big_int | ✓ |  |
| product_id | big_int | ✓ |  |
| rating | int | ✓ |  |
| title | string |  |  |
| comment | text |  |  |
| is_verified_purchase | bool |  |  |
| helpful_votes | int |  |  |
| created_at | date_time |  |  |
| updated_at | date_time |  |  |

**Generated Command**:
```bash
cargo loco generate scaffold reviews id:big_int! user_id:big_int! product_id:big_int! rating:int! title:string comment:text is_verified_purchase:bool helpful_votes:int created_at:date_time updated_at:date_time --api
```

### User Create

**Operations**: GET, POST

**Fields**:

| Field | Type | Required | Reference |
|-------|------|----------|----------|
| email | string | ✓ |  |
| username | string | ✓ |  |
| password | string | ✓ |  |
| first_name | string |  |  |
| last_name | string |  |  |

**Generated Command**:
```bash
cargo loco generate scaffold user_create email:string! username:string! password:string! first_name:string last_name:string --api
```

### User Profile

**Operations**: GET, POST

**Fields**:

| Field | Type | Required | Reference |
|-------|------|----------|----------|
| bio | string |  |  |
| avatar_url | string |  |  |
| website | string |  |  |
| social_links | json |  |  |

**Generated Command**:
```bash
cargo loco generate scaffold user_profile bio:string avatar_url:string website:string social_links:json --api
```

### Address

**Operations**: GET, POST

**Fields**:

| Field | Type | Required | Reference |
|-------|------|----------|----------|
| id | big_int |  |  |
| user_id | big_int |  |  |
| type | string |  |  |
| street | string | ✓ |  |
| city | string | ✓ |  |
| state | string |  |  |
| postal_code | string |  |  |
| country | string | ✓ |  |
| is_default | bool |  |  |

**Generated Command**:
```bash
cargo loco generate scaffold address id:big_int user_id:big_int type:string street:string! city:string! state:string postal_code:string country:string! is_default:bool --api
```

### Orders

**Operations**: GET, POST

**Fields**:

| Field | Type | Required | Reference |
|-------|------|----------|----------|
| id | big_int | ✓ |  |
| user_id | big_int | ✓ |  |
| order_number | string |  |  |
| status | string | ✓ |  |
| total_amount | decimal | ✓ |  |
| subtotal | decimal |  |  |
| tax_amount | decimal |  |  |
| shipping_cost | decimal |  |  |
| discount_amount | decimal |  |  |
| payment_method | string |  |  |
| items | array |  |  |
| notes | string |  |  |
| created_at | date_time |  |  |
| updated_at | date_time |  |  |
| shipped_at | date_time |  |  |
| delivered_at | date_time |  |  |

**Generated Command**:
```bash
cargo loco generate scaffold orders id:big_int! user_id:big_int! order_number:string status:string! total_amount:decimal! subtotal:decimal tax_amount:decimal shipping_cost:decimal discount_amount:decimal payment_method:string items:array notes:string created_at:date_time updated_at:date_time shipped_at:date_time delivered_at:date_time --api
```

### Category

**Operations**: GET, POST

**Fields**:

| Field | Type | Required | Reference |
|-------|------|----------|----------|
| id | big_int | ✓ |  |
| name | string | ✓ |  |
| description | string |  |  |
| parent_id | big_int |  |  |
| slug | string |  |  |
| image_url | string |  |  |
| sort_order | int |  |  |
| is_active | bool |  |  |

**Generated Command**:
```bash
cargo loco generate scaffold category id:big_int! name:string! description:string parent_id:big_int slug:string image_url:string sort_order:int is_active:bool --api
```

### Order Item

**Operations**: GET, POST

**Fields**:

| Field | Type | Required | Reference |
|-------|------|----------|----------|
| id | big_int | ✓ |  |
| order_id | big_int | ✓ |  |
| product_id | big_int | ✓ |  |
| quantity | int | ✓ |  |
| unit_price | decimal | ✓ |  |
| total_price | decimal |  |  |
| discount_amount | decimal |  |  |

**Generated Command**:
```bash
cargo loco generate scaffold order_item id:big_int! order_id:big_int! product_id:big_int! quantity:int! unit_price:decimal! total_price:decimal discount_amount:decimal --api
```

### Products

**Operations**: GET, POST

**Fields**:

| Field | Type | Required | Reference |
|-------|------|----------|----------|
| id | big_int | ✓ |  |
| name | string | ✓ |  |
| description | text |  |  |
| price | decimal | ✓ |  |
| discounted_price | decimal |  |  |
| sku | string |  |  |
| category_id | big_int | ✓ |  |
| brand | string |  |  |
| weight | decimal |  |  |
| dimensions | json |  |  |
| stock_quantity | int |  |  |
| is_active | bool |  |  |
| tags | array |  |  |
| images | array |  |  |
| created_at | date_time |  |  |
| updated_at | date_time |  |  |
| average_rating | float |  |  |
| review_count | int |  |  |

**Generated Command**:
```bash
cargo loco generate scaffold products id:big_int! name:string! description:text price:decimal! discounted_price:decimal sku:string category_id:big_int! brand:string weight:decimal dimensions:json stock_quantity:int is_active:bool tags:array images:array created_at:date_time updated_at:date_time average_rating:float review_count:int --api
```

### Product Create

**Operations**: GET, POST

**Fields**:

| Field | Type | Required | Reference |
|-------|------|----------|----------|
| name | string | ✓ |  |
| description | text |  |  |
| price | decimal | ✓ |  |
| sku | string |  |  |
| category_id | big_int | ✓ |  |
| brand | string |  |  |

**Generated Command**:
```bash
cargo loco generate scaffold product_create name:string! description:text price:decimal! sku:string category_id:big_int! brand:string --api
```

### Order Create

**Operations**: GET, POST

**Fields**:

| Field | Type | Required | Reference |
|-------|------|----------|----------|
| user_id | big_int | ✓ |  |
| items | array | ✓ |  |
| shipping_address_id | big_int |  |  |
| billing_address_id | big_int |  |  |
| notes | string |  |  |

**Generated Command**:
```bash
cargo loco generate scaffold order_create user_id:big_int! items:array! shipping_address_id:big_int billing_address_id:big_int notes:string --api
```

### Review Create

**Operations**: GET, POST

**Fields**:

| Field | Type | Required | Reference |
|-------|------|----------|----------|
| user_id | big_int | ✓ |  |
| product_id | big_int | ✓ |  |
| rating | int | ✓ |  |
| title | string |  |  |
| comment | text |  |  |

**Generated Command**:
```bash
cargo loco generate scaffold review_create user_id:big_int! product_id:big_int! rating:int! title:string comment:text --api
```

### Users

**Operations**: GET, POST, GET

**Fields**:

| Field | Type | Required | Reference |
|-------|------|----------|----------|
| id | big_int | ✓ |  |
| email | string | ✓ |  |
| username | string | ✓ |  |
| first_name | string |  |  |
| last_name | string |  |  |
| date_of_birth | date |  |  |
| phone | string |  |  |
| is_active | bool |  |  |
| registration_date | date_time |  |  |
| last_login | date_time |  |  |
| addresses | array |  |  |
| preferences | json |  |  |

**Generated Command**:
```bash
cargo loco generate scaffold users id:big_int! email:string! username:string! first_name:string last_name:string date_of_birth:date phone:string is_active:bool registration_date:date_time last_login:date_time addresses:array preferences:json --api
```

## Notes

- All commands use `--api` flag for API-only scaffolding
- Reference relationships are detected based on field naming conventions (`*_id`, `*Id`)
- Field types are mapped from OpenAPI types to Loco types
- Required fields are marked with `!` suffix

---
*Generated by [openapi-to-loco-scaf](https://github.com/your-repo/openapi-to-loco-scaf)*
