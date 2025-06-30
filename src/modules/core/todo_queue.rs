//! Module todo queue for cross-module communication
//!
//! This provides a clean interface for modules to communicate with each other
//! without tight coupling, following the single point of contact principle.

use serde_json::Value;
use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

/// Priority levels for todo items
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TodoPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// A todo item for cross-module communication
#[derive(Debug, Clone)]
pub struct TodoItem {
    /// Target module name
    pub module: String,
    /// Action to perform
    pub action: String,
    /// Data payload (JSON)
    pub data: Value,
    /// Priority level
    pub priority: TodoPriority,
    /// Optional response channel
    pub response_channel: Option<Sender<TodoResponse>>,
}

/// Response from a todo item
#[derive(Debug, Clone)]
pub struct TodoResponse {
    /// Whether the action succeeded
    pub success: bool,
    /// Response data if successful
    pub data: Option<Value>,
    /// Error message if failed
    pub error: Option<String>,
}

/// Thread-safe todo queue for modules
#[derive(Clone)]
pub struct ModuleTodoQueue {
    queues: Arc<Mutex<std::collections::HashMap<String, VecDeque<TodoItem>>>>,
}

impl ModuleTodoQueue {
    /// Create a new module todo queue
    pub fn new() -> Self {
        Self {
            queues: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    /// Add a todo item to the queue
    pub fn add_todo(&self, todo: TodoItem) {
        let mut queues = self.queues.lock().unwrap();
        let module_queue = queues.entry(todo.module.clone()).or_default();

        // Insert based on priority
        let position = module_queue
            .iter()
            .position(|item| item.priority < todo.priority);
        match position {
            Some(pos) => module_queue.insert(pos, todo),
            None => module_queue.push_back(todo),
        }
    }

    /// Get the next todo for a specific module
    pub fn get_todo(&self, module: &str) -> Option<TodoItem> {
        let mut queues = self.queues.lock().unwrap();
        queues.get_mut(module).and_then(|queue| queue.pop_front())
    }

    /// Check if a module has pending todos
    pub fn has_todos(&self, module: &str) -> bool {
        let queues = self.queues.lock().unwrap();
        queues.get(module).is_some_and(|queue| !queue.is_empty())
    }

    /// Create a request/response pair for synchronous communication
    pub fn create_request(
        &self,
        module: String,
        action: String,
        data: Value,
        priority: TodoPriority,
    ) -> (TodoItem, Receiver<TodoResponse>) {
        let (tx, rx) = channel();
        let todo = TodoItem {
            module,
            action,
            data,
            priority,
            response_channel: Some(tx),
        };
        (todo, rx)
    }
}

impl Default for ModuleTodoQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_todo_queue_priority() {
        let queue = ModuleTodoQueue::new();

        // Add items with different priorities
        queue.add_todo(TodoItem {
            module: "test".to_string(),
            action: "low".to_string(),
            data: json!({}),
            priority: TodoPriority::Low,
            response_channel: None,
        });

        queue.add_todo(TodoItem {
            module: "test".to_string(),
            action: "high".to_string(),
            data: json!({}),
            priority: TodoPriority::High,
            response_channel: None,
        });

        queue.add_todo(TodoItem {
            module: "test".to_string(),
            action: "normal".to_string(),
            data: json!({}),
            priority: TodoPriority::Normal,
            response_channel: None,
        });

        // Should get high priority first
        let first = queue.get_todo("test").unwrap();
        assert_eq!(first.action, "high");

        // Then normal
        let second = queue.get_todo("test").unwrap();
        assert_eq!(second.action, "normal");

        // Finally low
        let third = queue.get_todo("test").unwrap();
        assert_eq!(third.action, "low");
    }
}
