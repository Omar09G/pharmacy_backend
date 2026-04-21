/// Integration tests for `PaginationParams` deserialisation.
///
/// These tests ensure the custom `deserialize_page` / `deserialize_limit`
/// validators enforce their boundaries correctly via `serde_json`.
use pharmacy_backend::api_utils::api_response::PaginationParams;
use serde_json;

// ── happy-path deserialisation ────────────────────────────────────────────────

#[test]
fn minimal_params_deserialise_correctly() {
    let json = r#"{"page":1,"limit":1,"total":0}"#;
    let p: PaginationParams = serde_json::from_str(json).expect("minimal values must be accepted");
    assert_eq!(p.page, 1);
    assert_eq!(p.limit, 1);
    assert_eq!(p.total, 0);
}

#[test]
fn maximum_boundary_values_are_accepted() {
    let json = r#"{"page":10000,"limit":10000,"total":9999}"#;
    let p: PaginationParams =
        serde_json::from_str(json).expect("maximum boundary values must be accepted");
    assert_eq!(p.page, 10_000);
    assert_eq!(p.limit, 10_000);
}

#[test]
fn all_optional_fields_deserialise_when_present() {
    let json = r#"{
        "page": 2,
        "limit": 20,
        "total": 100,
        "dateInit": "2024-01-01",
        "dateEnd": "2024-12-31",
        "username": "alice",
        "clientId": 5,
        "userId": 10,
        "name": "Aspirin",
        "active": true,
        "status": "ACTIVE"
    }"#;
    let p: PaginationParams = serde_json::from_str(json).expect("full params must deserialise");
    assert_eq!(p.page, 2);
    assert_eq!(p.limit, 20);
    assert_eq!(p.username, Some("alice".to_string()));
    assert_eq!(p.client_id, Some(5));
    assert_eq!(p.user_id, Some(10));
    assert_eq!(p.active, Some(true));
    assert_eq!(p.status, Some("ACTIVE".to_string()));
    assert_eq!(p.date_init, Some("2024-01-01".to_string()));
    assert_eq!(p.date_end, Some("2024-12-31".to_string()));
}

#[test]
fn optional_fields_are_none_when_absent() {
    let json = r#"{"page":3,"limit":15,"total":0}"#;
    let p: PaginationParams = serde_json::from_str(json).unwrap();
    assert!(p.username.is_none());
    assert!(p.client_id.is_none());
    assert!(p.user_id.is_none());
    assert!(p.name.is_none());
    assert!(p.code.is_none());
    assert!(p.date_init.is_none());
    assert!(p.date_end.is_none());
    assert!(p.active.is_none());
    assert!(p.status.is_none());
    assert!(p.email.is_none());
    assert!(p.phone.is_none());
    assert!(p.sku.is_none());
}

// ── page boundary violations ──────────────────────────────────────────────────

#[test]
fn page_zero_is_rejected() {
    let json = r#"{"page":0,"limit":10,"total":0}"#;
    assert!(
        serde_json::from_str::<PaginationParams>(json).is_err(),
        "page=0 must be rejected"
    );
}

#[test]
fn page_above_max_is_rejected() {
    let json = r#"{"page":10001,"limit":10,"total":0}"#;
    assert!(
        serde_json::from_str::<PaginationParams>(json).is_err(),
        "page=10001 must be rejected"
    );
}

// ── limit boundary violations ─────────────────────────────────────────────────

#[test]
fn limit_zero_is_rejected() {
    let json = r#"{"page":1,"limit":0,"total":0}"#;
    assert!(
        serde_json::from_str::<PaginationParams>(json).is_err(),
        "limit=0 must be rejected"
    );
}

#[test]
fn limit_above_max_is_rejected() {
    let json = r#"{"page":1,"limit":10001,"total":0}"#;
    assert!(
        serde_json::from_str::<PaginationParams>(json).is_err(),
        "limit=10001 must be rejected"
    );
}

// ── camelCase field names ─────────────────────────────────────────────────────

#[test]
fn camel_case_fields_deserialise_correctly() {
    let json = r#"{
        "page": 1,
        "limit": 10,
        "total": 0,
        "roleId": 3,
        "permissionId": 7,
        "productId": 42,
        "categoryId": 5,
        "supplierId": 9,
        "invoiceNo": "INV-001",
        "customerId": 11,
        "taxProfileId": 2
    }"#;
    let p: PaginationParams =
        serde_json::from_str(json).expect("camelCase fields must deserialise");
    assert_eq!(p.role_id, Some(3));
    assert_eq!(p.permission_id, Some(7));
    assert_eq!(p.product_id, Some(42));
    assert_eq!(p.category_id, Some(5));
    assert_eq!(p.supplier_id, Some(9));
    assert_eq!(p.invoice_no, Some("INV-001".to_string()));
    assert_eq!(p.customer_id, Some(11));
    assert_eq!(p.tax_profile_id, Some(2));
}

#[test]
fn boolean_optional_fields_deserialise() {
    let json = r#"{"page":1,"limit":10,"total":0,"isSellable":true,"trackBatches":false}"#;
    let p: PaginationParams = serde_json::from_str(json).unwrap();
    assert_eq!(p.is_sellable, Some(true));
    assert_eq!(p.track_batches, Some(false));
}

// ── serialisation round-trip ──────────────────────────────────────────────────

#[test]
fn round_trip_serialise_and_deserialise() {
    let json = r#"{"page":5,"limit":25,"total":200,"username":"bob"}"#;
    let p: PaginationParams = serde_json::from_str(json).unwrap();
    let serialised = serde_json::to_string(&p).expect("serialisation must not fail");
    // The serialised form must still be valid JSON that round-trips
    let p2: PaginationParams = serde_json::from_str(&serialised).unwrap();
    assert_eq!(p2.page, 5);
    assert_eq!(p2.limit, 25);
    assert_eq!(p2.username, Some("bob".to_string()));
}
