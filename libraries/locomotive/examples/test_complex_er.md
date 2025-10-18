# Complex E-commerce Database Design

This is a complex ER diagram for an e-commerce system.

```mermaid
erDiagram
    USER ||--o{ ORDER : places
    USER ||--o{ REVIEW : writes
    USER }|--|| PROFILE : has
    USER }o--o{ PRODUCT : favorites
    
    ORDER ||--o{ ORDER_ITEM : contains
    ORDER }o--|| ADDRESS : shipped_to
    ORDER }o--|| PAYMENT : processed_with
    
    PRODUCT ||--o{ REVIEW : receives
    PRODUCT }o--|| CATEGORY : belongs_to
    PRODUCT }o--|| BRAND : manufactured_by
    PRODUCT }o--o{ TAG : tagged_with
    
    CATEGORY ||--o{ CATEGORY : parent_child
    
    ADDRESS }o--|| USER : belongs_to
    PAYMENT }o--|| USER : belongs_to
    
    USER {
        int id PK
        string username UK
        string email UK
        string password
        datetime created_at
        datetime updated_at
    }
    
    PROFILE {
        int id PK
        int user_id FK
        string first_name
        string last_name
        string phone
        text bio
        string avatar_url
    }
    
    ORDER {
        int id PK
        int user_id FK
        int address_id FK
        int payment_id FK
        string status
        decimal total_amount
        datetime created_at
        datetime shipped_at
    }
    
    ORDER_ITEM {
        int id PK
        int order_id FK
        int product_id FK
        int quantity
        decimal unit_price
        decimal total_price
    }
    
    PRODUCT {
        int id PK
        string name
        text description
        decimal price
        int stock_quantity
        int category_id FK
        int brand_id FK
        string sku UK
        datetime created_at
        datetime updated_at
    }
    
    CATEGORY {
        int id PK
        string name UK
        text description
        int parent_id FK
        string slug UK
    }
    
    BRAND {
        int id PK
        string name UK
        text description
        string logo_url
        string website
    }
    
    TAG {
        int id PK
        string name UK
        string color
    }
    
    REVIEW {
        int id PK
        int user_id FK
        int product_id FK
        int rating
        text comment
        datetime created_at
    }
    
    ADDRESS {
        int id PK
        int user_id FK
        string type
        string street
        string city
        string state
        string postal_code
        string country
        bool is_default
    }
    
    PAYMENT {
        int id PK
        int user_id FK
        string type
        string card_last_four
        string provider
        string token
        bool is_default
        datetime expires_at
    }
```

## Relationship Summary

- Users can place multiple orders and write reviews
- Users have one profile and can favorite multiple products
- Orders contain multiple items and are associated with shipping addresses and payments
- Products belong to categories and brands, can have multiple reviews and tags
- Categories can have parent-child relationships (hierarchical)
- Addresses and payments belong to users
- Many-to-many relationships: User-Product (favorites), Product-Tag