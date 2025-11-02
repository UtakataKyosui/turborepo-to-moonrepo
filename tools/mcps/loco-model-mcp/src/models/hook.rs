// Lifecycle hook data structures

use serde::{Deserialize, Serialize};

/// Lifecycle hook for model events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleHook {
    /// Hook type (when it executes)
    pub hook_type: HookType,

    /// Code to execute for this hook
    pub code: String,

    /// Optional description of what this hook does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Types of lifecycle hooks
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HookType {
    /// Execute before saving (create or update)
    BeforeSave,

    /// Execute after saving (create or update)
    AfterSave,

    /// Execute before creating
    BeforeCreate,

    /// Execute after creating
    AfterCreate,

    /// Execute before updating
    BeforeUpdate,

    /// Execute after updating
    AfterUpdate,

    /// Execute before deleting
    BeforeDelete,

    /// Execute after deleting
    AfterDelete,
}

impl HookType {
    /// Get the SeaORM ActiveModelBehavior method name for this hook
    pub fn to_method_name(&self) -> &'static str {
        match self {
            Self::BeforeSave => "before_save",
            Self::AfterSave => "after_save",
            Self::BeforeCreate => "before_save", // SeaORM combines these
            Self::AfterCreate => "after_save",   // SeaORM combines these
            Self::BeforeUpdate => "before_save", // SeaORM combines these
            Self::AfterUpdate => "after_save",   // SeaORM combines these
            Self::BeforeDelete => "before_delete",
            Self::AfterDelete => "after_delete",
        }
    }

    /// Check if this is a "before" hook
    pub fn is_before(&self) -> bool {
        matches!(
            self,
            Self::BeforeSave
                | Self::BeforeCreate
                | Self::BeforeUpdate
                | Self::BeforeDelete
        )
    }

    /// Check if this is an "after" hook
    pub fn is_after(&self) -> bool {
        matches!(
            self,
            Self::AfterSave | Self::AfterCreate | Self::AfterUpdate | Self::AfterDelete
        )
    }

    /// Check if this is a save hook (create or update)
    pub fn is_save(&self) -> bool {
        matches!(
            self,
            Self::BeforeSave
                | Self::AfterSave
                | Self::BeforeCreate
                | Self::AfterCreate
                | Self::BeforeUpdate
                | Self::AfterUpdate
        )
    }

    /// Check if this is a delete hook
    pub fn is_delete(&self) -> bool {
        matches!(self, Self::BeforeDelete | Self::AfterDelete)
    }
}

impl LifecycleHook {
    /// Create a new lifecycle hook
    pub fn new(hook_type: HookType, code: String) -> Self {
        Self {
            hook_type,
            code,
            description: None,
        }
    }

    /// Create a before_save hook
    pub fn before_save(code: String) -> Self {
        Self::new(HookType::BeforeSave, code)
    }

    /// Create an after_save hook
    pub fn after_save(code: String) -> Self {
        Self::new(HookType::AfterSave, code)
    }

    /// Create a before_create hook
    pub fn before_create(code: String) -> Self {
        Self::new(HookType::BeforeCreate, code)
    }

    /// Create an after_create hook
    pub fn after_create(code: String) -> Self {
        Self::new(HookType::AfterCreate, code)
    }

    /// Create a before_update hook
    pub fn before_update(code: String) -> Self {
        Self::new(HookType::BeforeUpdate, code)
    }

    /// Create an after_update hook
    pub fn after_update(code: String) -> Self {
        Self::new(HookType::AfterUpdate, code)
    }

    /// Create a before_delete hook
    pub fn before_delete(code: String) -> Self {
        Self::new(HookType::BeforeDelete, code)
    }

    /// Create an after_delete hook
    pub fn after_delete(code: String) -> Self {
        Self::new(HookType::AfterDelete, code)
    }
}
