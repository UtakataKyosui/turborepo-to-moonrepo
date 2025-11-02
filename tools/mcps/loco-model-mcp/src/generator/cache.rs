// Template cache using LRU for performance optimization

use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::Mutex;

/// Template cache for storing compiled Tera templates
pub struct TemplateCache {
    cache: Mutex<LruCache<String, String>>,
}

impl TemplateCache {
    /// Create a new template cache with the given capacity
    pub fn new(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity).expect("Capacity must be non-zero");
        Self {
            cache: Mutex::new(LruCache::new(cap)),
        }
    }

    /// Create a new template cache with default capacity (100 items)
    pub fn default() -> Self {
        Self::new(100)
    }

    /// Get a template from the cache
    pub fn get(&self, key: &str) -> Option<String> {
        let mut cache = self.cache.lock().unwrap();
        cache.get(key).cloned()
    }

    /// Put a template into the cache
    pub fn put(&self, key: String, value: String) {
        let mut cache = self.cache.lock().unwrap();
        cache.put(key, value);
    }

    /// Check if a template exists in the cache
    pub fn contains(&self, key: &str) -> bool {
        let mut cache = self.cache.lock().unwrap();
        cache.contains(key)
    }

    /// Clear all templates from the cache
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Get the current number of cached templates
    pub fn len(&self) -> usize {
        let cache = self.cache.lock().unwrap();
        cache.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        let cache = self.cache.lock().unwrap();
        cache.is_empty()
    }

    /// Get or compute a template
    pub fn get_or_insert<F>(&self, key: &str, compute: F) -> String
    where
        F: FnOnce() -> String,
    {
        // Check if it exists first (fast path)
        if let Some(value) = self.get(key) {
            return value;
        }

        // Compute and insert (slow path)
        let value = compute();
        self.put(key.to_string(), value.clone());
        value
    }
}

impl Default for TemplateCache {
    fn default() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic_operations() {
        let cache = TemplateCache::new(2);

        // Test put and get
        cache.put("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get("key1"), Some("value1".to_string()));

        // Test contains
        assert!(cache.contains("key1"));
        assert!(!cache.contains("key2"));

        // Test len
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_cache_lru_eviction() {
        let cache = TemplateCache::new(2);

        // Fill cache
        cache.put("key1".to_string(), "value1".to_string());
        cache.put("key2".to_string(), "value2".to_string());

        // Add one more, should evict key1
        cache.put("key3".to_string(), "value3".to_string());

        assert!(!cache.contains("key1"));
        assert!(cache.contains("key2"));
        assert!(cache.contains("key3"));
    }

    #[test]
    fn test_get_or_insert() {
        let cache = TemplateCache::new(10);

        // First call should compute
        let value = cache.get_or_insert("key1", || "computed_value".to_string());
        assert_eq!(value, "computed_value");

        // Second call should return cached value
        let value = cache.get_or_insert("key1", || "should_not_compute".to_string());
        assert_eq!(value, "computed_value");
    }

    #[test]
    fn test_clear() {
        let cache = TemplateCache::new(10);

        cache.put("key1".to_string(), "value1".to_string());
        cache.put("key2".to_string(), "value2".to_string());

        assert_eq!(cache.len(), 2);

        cache.clear();

        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }
}
