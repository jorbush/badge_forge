#[cfg(test)]
mod tests {
    use badge_forge::{
        model::level::LevelRequest,
        queue::{BadgeUpdateQueue, InMemoryQueue},
    };
    use chrono::{TimeZone, Utc};
    use std::{sync::Arc, time::Duration};
    use uuid::Uuid;

    // Helper function to create a test LevelRequest
    fn create_test_request(user_id: &str) -> LevelRequest {
        LevelRequest {
            user_id: user_id.to_string(),
            request_id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
        }
    }

    // Helper function to create a test LevelRequest with specific ID
    fn create_test_request_with_id(user_id: &str, request_id: &str) -> LevelRequest {
        LevelRequest {
            user_id: user_id.to_string(),
            request_id: request_id.to_string(),
            created_at: Utc::now(),
        }
    }

    // Helper function to create a LevelRequest with empty fields
    fn create_empty_request(user_id: &str) -> LevelRequest {
        LevelRequest {
            user_id: user_id.to_string(),
            request_id: String::new(),
            created_at: Utc.timestamp_opt(0, 0).unwrap(), // Unix epoch (empty date)
        }
    }

    #[tokio::test]
    async fn test_queue_creation() {
        let (queue, _receiver) = InMemoryQueue::new(10);

        // Check that the queue starts empty
        let pending = queue.get_pending_requests().await;
        assert_eq!(pending.len(), 0, "Queue should be empty on creation");
    }

    #[tokio::test]
    async fn test_enqueue() {
        let (queue, mut receiver) = InMemoryQueue::new(10);
        let request = create_test_request("user123");

        // Enqueue a request
        let result = queue.enqueue(request.clone()).await;
        assert!(result.is_ok(), "Enqueue should succeed");

        // Verify it's in the pending list
        let pending = queue.get_pending_requests().await;
        assert_eq!(pending.len(), 1, "Queue should have 1 pending request");
        assert_eq!(pending[0].user_id, "user123", "User ID should match");

        // Verify it was sent to the channel
        let received = tokio::time::timeout(Duration::from_millis(100), receiver.recv()).await;
        assert!(
            received.is_ok(),
            "Should receive the request from the channel"
        );
        assert_eq!(
            received.unwrap().unwrap().user_id,
            "user123",
            "Received request should match"
        );
    }

    #[tokio::test]
    async fn test_enqueue_empty_fields() {
        let (queue, mut receiver) = InMemoryQueue::new(10);
        let empty_request = create_empty_request("user456");

        // Verify fields are empty
        assert!(
            empty_request.request_id.is_empty(),
            "request_id should be empty"
        );
        assert_eq!(
            empty_request.created_at.timestamp(),
            0,
            "created_at should be Unix epoch"
        );

        // Enqueue request with empty fields
        let result = queue.enqueue(empty_request).await;
        assert!(
            result.is_ok(),
            "Enqueue should succeed even with empty fields"
        );

        // Verify request in queue has generated fields
        let pending = queue.get_pending_requests().await;
        assert_eq!(pending.len(), 1, "Queue should have 1 pending request");
        assert!(
            !pending[0].request_id.is_empty(),
            "request_id should be generated"
        );
        assert!(
            pending[0].created_at.timestamp() > 0,
            "created_at should be set to current time"
        );

        // Verify the received request also has generated fields
        let received = tokio::time::timeout(Duration::from_millis(100), receiver.recv()).await;
        let received_req = received.unwrap().unwrap();
        assert!(
            !received_req.request_id.is_empty(),
            "Received request should have generated request_id"
        );
        assert!(
            received_req.created_at.timestamp() > 0,
            "Received request should have current timestamp"
        );
    }

    #[tokio::test]
    async fn test_remove_request() {
        let (queue, _receiver) = InMemoryQueue::new(10);
        let request_id = "test-request-id-123";
        let request = create_test_request_with_id("user123", request_id);

        // Add request to queue
        let _ = queue.enqueue(request).await;

        // Verify request is in the queue
        let pending_before = queue.get_pending_requests().await;
        assert_eq!(pending_before.len(), 1, "Queue should have 1 request");

        // Remove the request
        queue.remove_request(request_id).await;

        // Verify request was removed
        let pending_after = queue.get_pending_requests().await;
        assert_eq!(
            pending_after.len(),
            0,
            "Queue should be empty after removing request"
        );
    }

    #[tokio::test]
    async fn test_remove_nonexistent_request() {
        let (queue, _receiver) = InMemoryQueue::new(10);
        let request = create_test_request("user123");

        // Add request to queue
        let _ = queue.enqueue(request).await;

        // Try to remove non-existent request
        queue.remove_request("non-existent-id").await;

        // Verify the queue is unchanged
        let pending = queue.get_pending_requests().await;
        assert_eq!(pending.len(), 1, "Queue should still have 1 request");
    }

    #[tokio::test]
    async fn test_get_pending_requests_empty() {
        let (queue, _receiver) = InMemoryQueue::new(10);

        // Check empty queue
        let pending = queue.get_pending_requests().await;
        assert_eq!(pending.len(), 0, "Empty queue should return empty vector");
    }

    #[tokio::test]
    async fn test_get_pending_requests_with_items() {
        let (queue, _receiver) = InMemoryQueue::new(10);

        // Add multiple requests
        let _ = queue.enqueue(create_test_request("user1")).await;
        let _ = queue.enqueue(create_test_request("user2")).await;
        let _ = queue.enqueue(create_test_request("user3")).await;

        // Check queue has all items
        let pending = queue.get_pending_requests().await;
        assert_eq!(pending.len(), 3, "Queue should have 3 requests");

        // Verify the user IDs
        let user_ids: Vec<String> = pending.iter().map(|req| req.user_id.clone()).collect();
        assert!(
            user_ids.contains(&"user1".to_string()),
            "Should contain user1"
        );
        assert!(
            user_ids.contains(&"user2".to_string()),
            "Should contain user2"
        );
        assert!(
            user_ids.contains(&"user3".to_string()),
            "Should contain user3"
        );
    }

    #[tokio::test]
    async fn test_concurrent_enqueue() {
        let (queue, _receiver) = InMemoryQueue::new(100);
        let queue_arc = Arc::new(queue);

        // Create multiple tasks that enqueue concurrently
        let mut handles = vec![];
        for i in 0..10 {
            let queue_clone = queue_arc.clone();
            let handle = tokio::spawn(async move {
                let user_id = format!("user{}", i);
                let _ = queue_clone.enqueue(create_test_request(&user_id)).await;
            });
            handles.push(handle);
        }

        // Wait for all tasks to complete
        for handle in handles {
            let _ = handle.await;
        }

        // Check all requests were added
        let pending = queue_arc.get_pending_requests().await;
        assert_eq!(
            pending.len(),
            10,
            "Queue should have 10 requests after concurrent enqueue"
        );
    }

    #[tokio::test]
    async fn test_concurrent_enqueue_and_remove() {
        let (queue, _receiver) = InMemoryQueue::new(100);
        let queue_arc = Arc::new(queue);

        // First add some requests and collect their IDs
        let mut request_ids = Vec::new();
        for i in 0..5 {
            let user_id = format!("user{}", i);
            let request_id = Uuid::new_v4().to_string();
            request_ids.push(request_id.clone());
            let _ = queue_arc
                .enqueue(create_test_request_with_id(&user_id, &request_id))
                .await;
        }

        // Create tasks that enqueue and remove concurrently
        let mut handles = vec![];

        // Enqueue tasks
        for i in 5..10 {
            let queue_clone = queue_arc.clone();
            let handle = tokio::spawn(async move {
                let user_id = format!("user{}", i);
                let _ = queue_clone.enqueue(create_test_request(&user_id)).await;
            });
            handles.push(handle);
        }

        // Remove tasks (for the first 5 requests)
        for id in request_ids {
            let queue_clone = queue_arc.clone();
            let handle = tokio::spawn(async move {
                queue_clone.remove_request(&id).await;
            });
            handles.push(handle);
        }

        // Wait for all tasks to complete
        for handle in handles {
            let _ = handle.await;
        }

        // Check we have just the newly added requests
        let pending = queue_arc.get_pending_requests().await;
        assert_eq!(
            pending.len(),
            5,
            "Queue should have 5 requests after concurrent operations"
        );

        // Verify all remaining requests are from the second batch (user5-user9)
        for req in pending {
            let user_id_num: String = req.user_id.chars().filter(|c| c.is_ascii_digit()).collect();
            let num: u32 = user_id_num.parse().unwrap_or(0);
            assert!(
                (5..10).contains(&num),
                "Only requests from the second batch should remain"
            );
        }
    }

    #[tokio::test]
    async fn test_queue_backpressure() {
        // Create a very small buffer to test backpressure
        let (queue, _receiver) = InMemoryQueue::new(2);

        // Fill the queue
        let _ = queue.enqueue(create_test_request("user1")).await;
        let _ = queue.enqueue(create_test_request("user2")).await;

        // Try to enqueue a third item with a timeout
        let third_request = create_test_request("user3");

        // Use timeout to prevent the test from hanging
        let enqueue_result = tokio::time::timeout(
            std::time::Duration::from_millis(100),
            queue.enqueue(third_request),
        )
        .await;

        // Verify that the operation timed out, which means it was blocking
        assert!(
            enqueue_result.is_err(),
            "Enqueue should block when buffer is full"
        );

        // Verify we still have the first two in the pending list
        let pending = queue.get_pending_requests().await;
        assert_eq!(
            pending.len(),
            2,
            "Queue should have 2 items despite attempted overflow"
        );
    }

    #[tokio::test]
    async fn test_same_user_different_requests() {
        let (queue, _receiver) = InMemoryQueue::new(10);

        // Add multiple requests for the same user but with different request_ids
        let _ = queue.enqueue(create_test_request("same_user")).await;
        let _ = queue.enqueue(create_test_request("same_user")).await;
        let _ = queue.enqueue(create_test_request("same_user")).await;

        // Check queue has all items despite same user_id
        let pending = queue.get_pending_requests().await;
        assert_eq!(
            pending.len(),
            3,
            "Queue should have 3 requests for the same user"
        );

        // Check they all have different request_ids
        let mut request_ids = Vec::new();
        for req in pending {
            assert_eq!(req.user_id, "same_user", "User ID should be the same");
            assert!(
                !request_ids.contains(&req.request_id),
                "Request IDs should be unique"
            );
            request_ids.push(req.request_id);
        }
    }
}
