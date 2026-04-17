# Test Coverage Audit

## Project Type Detection
- Declared in `repo/README.md`: **Fullstack**.
- Inferred type (light inspection): **fullstack**.

## Backend Endpoint Inventory

| Endpoint | Route File | Handler |
|---|---|---|
| DELETE /api/admin/users/:param/roles/:param | repo/backend/src/routes/admin.rs | remove_role |
| DELETE /api/cart/:param | repo/backend/src/routes/cart.rs | remove_item |
| DELETE /api/cart/clear | repo/backend/src/routes/cart.rs | clear_cart |
| DELETE /api/training/favorites/:param | repo/backend/src/routes/training.rs | remove_favorite |
| GET /api/admin/users | repo/backend/src/routes/admin.rs | list_users |
| GET /api/auth/me | repo/backend/src/routes/auth.rs | me |
| GET /api/cart | repo/backend/src/routes/cart.rs | get_cart |
| GET /api/dispatch/my-tasks | repo/backend/src/routes/dispatch.rs | my_tasks |
| GET /api/dispatch/queue | repo/backend/src/routes/dispatch.rs | queue |
| GET /api/dispatch/recommendations/:param | repo/backend/src/routes/dispatch.rs | recommendations |
| GET /api/dispatch/reputation/:param | repo/backend/src/routes/dispatch.rs | reputation |
| GET /api/dispatch/shifts | repo/backend/src/routes/dispatch.rs | shifts |
| GET /api/dispatch/zones | repo/backend/src/routes/dispatch.rs | list_zones |
| GET /api/exam/questions/:param | repo/backend/src/routes/exam.rs | get_question |
| GET /api/exam/questions | repo/backend/src/routes/exam.rs | list_questions |
| GET /api/exam/subjects/:param/chapters | repo/backend/src/routes/exam.rs | list_chapters |
| GET /api/exam/subjects | repo/backend/src/routes/exam.rs | list_subjects |
| GET /api/exam/versions/:param | repo/backend/src/routes/exam.rs | get_version |
| GET /api/exam/versions | repo/backend/src/routes/exam.rs | list_versions |
| GET /api/i18n/locales | repo/backend/src/routes/i18n.rs | get_locales |
| GET /api/i18n/translations/:param | repo/backend/src/routes/i18n.rs | get_translations |
| GET /api/orders/:param | repo/backend/src/routes/orders.rs | get_order |
| GET /api/orders | repo/backend/src/routes/orders.rs | list_orders |
| GET /api/products/:param | repo/backend/src/routes/products.rs | get_product |
| GET /api/products | repo/backend/src/routes/products.rs | list_products |
| GET /api/staff/dashboard/counts | repo/backend/src/routes/staff.rs | dashboard_counts |
| GET /api/staff/dashboard | repo/backend/src/routes/staff.rs | dashboard |
| GET /api/staff/orders/:param | repo/backend/src/routes/staff.rs | get_order |
| GET /api/staff/orders | repo/backend/src/routes/staff.rs | list_all_orders |
| GET /api/store/hours | repo/backend/src/routes/store.rs | get_store_hours |
| GET /api/store/pickup-slots | repo/backend/src/routes/store.rs | get_pickup_slots |
| GET /api/store/tax | repo/backend/src/routes/store.rs | get_tax |
| GET /api/training/analytics | repo/backend/src/routes/training.rs | get_analytics |
| GET /api/training/attempts/:param | repo/backend/src/routes/training.rs | get_attempt |
| GET /api/training/attempts | repo/backend/src/routes/training.rs | list_attempts |
| GET /api/training/favorites | repo/backend/src/routes/training.rs | list_favorites |
| GET /api/training/review-session | repo/backend/src/routes/training.rs | review_session |
| GET /api/training/wrong-notebook | repo/backend/src/routes/training.rs | wrong_notebook |
| GET /health/detailed | repo/backend/src/routes/health.rs | detailed |
| GET /health/live | repo/backend/src/routes/health.rs | live |
| GET /health/ready | repo/backend/src/routes/health.rs | ready |
| GET /health | repo/backend/src/routes/health.rs | health |
| GET /robots.txt | repo/backend/src/routes/sitemap.rs | robots |
| GET /sitemap.xml | repo/backend/src/routes/sitemap.rs | sitemap |
| POST /api/admin/products | repo/backend/src/routes/admin.rs | create_product |
| POST /api/admin/users/:param/roles | repo/backend/src/routes/admin.rs | assign_role |
| POST /api/auth/login | repo/backend/src/routes/auth.rs | login |
| POST /api/auth/logout | repo/backend/src/routes/auth.rs | logout |
| POST /api/auth/register | repo/backend/src/routes/auth.rs | register |
| POST /api/cart/add | repo/backend/src/routes/cart.rs | add_to_cart |
| POST /api/dispatch/accept/:param | repo/backend/src/routes/dispatch.rs | accept |
| POST /api/dispatch/assign | repo/backend/src/routes/dispatch.rs | assign |
| POST /api/dispatch/complete/:param | repo/backend/src/routes/dispatch.rs | complete |
| POST /api/dispatch/grab/:param | repo/backend/src/routes/dispatch.rs | grab |
| POST /api/dispatch/reject/:param | repo/backend/src/routes/dispatch.rs | reject |
| POST /api/dispatch/shifts | repo/backend/src/routes/dispatch.rs | create_shift |
| POST /api/dispatch/start/:param | repo/backend/src/routes/dispatch.rs | start |
| POST /api/exam/generate | repo/backend/src/routes/exam.rs | generate_exam |
| POST /api/exam/import | repo/backend/src/routes/exam.rs | import_questions |
| POST /api/exam/questions/import | repo/backend/src/routes/exam.rs | import_questions_alias |
| POST /api/orders/:param/cancel | repo/backend/src/routes/orders.rs | cancel_order |
| POST /api/orders/:param/confirm | repo/backend/src/routes/orders.rs | confirm_order |
| POST /api/orders/checkout | repo/backend/src/routes/orders.rs | checkout |
| POST /api/staff/scan | repo/backend/src/routes/staff.rs | scan_voucher |
| POST /api/training/answer | repo/backend/src/routes/training.rs | submit_answer |
| POST /api/training/favorites/:param | repo/backend/src/routes/training.rs | add_favorite |
| POST /api/training/finish/:param | repo/backend/src/routes/training.rs | finish_exam |
| POST /api/training/start/:param | repo/backend/src/routes/training.rs | start_exam |
| PUT /api/admin/products/:param | repo/backend/src/routes/admin.rs | update_product |
| PUT /api/admin/store-hours | repo/backend/src/routes/admin.rs | update_store_hours |
| PUT /api/admin/tax | repo/backend/src/routes/admin.rs | update_tax |
| PUT /api/auth/locale | repo/backend/src/routes/auth.rs | update_locale |
| PUT /api/cart/:param | repo/backend/src/routes/cart.rs | update_item |
| PUT /api/staff/orders/:param/status | repo/backend/src/routes/staff.rs | update_order_status |

## API Test Mapping Table

| Endpoint | Covered | Test Type | Test Files | Evidence |
|---|---|---|---|---|
| DELETE /api/admin/users/:param/roles/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | admin_remove_role_requires_admin (repo/backend/src/api_tests.rs:1853:        let resp = client.delete("/api/admin/users/2/roles/Teacher")) |
| DELETE /api/cart/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | cart_delete_item_requires_auth (repo/backend/src/api_tests.rs:1630:        let resp = client.delete("/api/cart/1").dispatch();) |
| DELETE /api/cart/clear | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | cart_empty_after_clear (repo/backend/src/api_tests.rs:1178:            .delete("/api/cart/clear")) |
| DELETE /api/training/favorites/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | training_remove_favorite_nonexistent_returns_ok (repo/backend/src/api_tests.rs:2428:        let resp = client.delete("/api/training/favorites/1")) |
| GET /api/admin/users | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | admin_can_list_users (repo/backend/src/api_tests.rs:710:            .get("/api/admin/users")) |
| GET /api/auth/me | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | e2e_customer_registration_through_session_expiry (repo/backend/src/api_tests.rs:1243:            .get("/api/auth/me")) |
| GET /api/cart | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | cart_empty_after_clear (repo/backend/src/api_tests.rs:1183:            .get("/api/cart/")) |
| GET /api/dispatch/my-tasks | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | dispatch_my_tasks_requires_auth (repo/backend/src/api_tests.rs:1095:        let resp = client.get("/api/dispatch/my-tasks").dispatch();) |
| GET /api/dispatch/queue | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | dispatch_queue_requires_staff (repo/backend/src/api_tests.rs:2022:        let resp = client.get("/api/dispatch/queue")) |
| GET /api/dispatch/recommendations/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | dispatch_recommendations_requires_admin (repo/backend/src/api_tests.rs:2186:        let resp = client.get("/api/dispatch/recommendations/9000")) |
| GET /api/dispatch/reputation/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | dispatch_reputation_requires_admin (repo/backend/src/api_tests.rs:2277:        let resp = client.get("/api/dispatch/reputation/3")) |
| GET /api/dispatch/shifts | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | dispatch_shifts_requires_staff (repo/backend/src/api_tests.rs:2212:        let resp = client.get("/api/dispatch/shifts?user_id=3&date=2026-04-16").dispatch();) |
| GET /api/dispatch/zones | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | dispatch_zones_returns_array_for_staff (repo/backend/src/api_tests.rs:1105:            .get("/api/dispatch/zones")) |
| GET /api/exam/questions | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | exam_questions_list_requires_teacher_role (repo/backend/src/api_tests.rs:2456:        let resp = client.get("/api/exam/questions?page=1&per_page=10")) |
| GET /api/exam/questions/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | exam_question_detail_for_admin_returns_200_or_404 (repo/backend/src/api_tests.rs:2497:        let resp = client.get("/api/exam/questions/1")) |
| GET /api/exam/subjects | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | exam_subjects_list_is_public_and_returns_array (repo/backend/src/api_tests.rs:1049:        let resp = client.get("/api/exam/subjects").dispatch();) |
| GET /api/exam/subjects/:param/chapters | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | exam_chapters_returns_array (repo/backend/src/api_tests.rs:2442:        let resp = client.get("/api/exam/subjects/1/chapters").dispatch();) |
| GET /api/exam/versions | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | exam_versions_requires_auth (repo/backend/src/api_tests.rs:1060:        let resp = client.get("/api/exam/versions").dispatch();) |
| GET /api/exam/versions/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | exam_version_detail_nonexistent_returns_404 (repo/backend/src/api_tests.rs:2600:        let resp = client.get("/api/exam/versions/99999999")) |
| GET /api/i18n/locales | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | e2e_i18n_surface_area_matches_frontend_contract (repo/backend/src/api_tests.rs:1502:        let list = client.get("/api/i18n/locales").dispatch();) |
| GET /api/i18n/translations/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | i18n_translations_en_contains_known_key (repo/backend/src/api_tests.rs:878:        let resp = client.get("/api/i18n/translations/en").dispatch();) |
| GET /api/orders | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | customer_can_list_own_orders (repo/backend/src/api_tests.rs:464:            .get("/api/orders")) |
| GET /api/orders/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | order_detail_nonexistent_returns_404 (repo/backend/src/api_tests.rs:1705:        let resp = client.get("/api/orders/99999999")) |
| GET /api/products | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | e2e_customer_browses_and_adds_to_cart (repo/backend/src/api_tests.rs:1283:        let list = client.get("/api/products/").dispatch();) |
| GET /api/products/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | e2e_customer_browses_and_adds_to_cart (repo/backend/src/api_tests.rs:1287:        let detail = client.get("/api/products/1").dispatch();) |
| GET /api/staff/dashboard | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | staff_dashboard_requires_staff_role (repo/backend/src/api_tests.rs:1799:        let resp = client.get("/api/staff/dashboard").dispatch();) |
| GET /api/staff/dashboard/counts | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | e2e_voucher_scan_of_cancelled_order_reports_mismatch (repo/backend/src/api_tests.rs:1488:            .get("/api/staff/dashboard/counts")) |
| GET /api/staff/orders | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | customer_cannot_access_staff_orders_returns_403 (repo/backend/src/api_tests.rs:246:            .get("/api/staff/orders")) |
| GET /api/staff/orders/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | staff_order_detail_requires_staff_role (repo/backend/src/api_tests.rs:1744:        let resp = client.get("/api/staff/orders/9000")) |
| GET /api/store/hours | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | store_hours_returns_seven_day_schedule (repo/backend/src/api_tests.rs:949:        let resp = client.get("/api/store/hours").dispatch();) |
| GET /api/store/pickup-slots | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | store_pickup_slots_invalid_date_returns_400 (repo/backend/src/api_tests.rs:984:            .get("/api/store/pickup-slots?date=not-a-date")) |
| GET /api/store/tax | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | store_tax_returns_active_rate (repo/backend/src/api_tests.rs:1007:        let resp = client.get("/api/store/tax").dispatch();) |
| GET /api/training/analytics | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | training_analytics_returns_data_for_customer (repo/backend/src/api_tests.rs:506:            .get("/api/training/analytics")) |
| GET /api/training/attempts | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | training_attempts_requires_auth (repo/backend/src/api_tests.rs:495:        let resp = client.get("/api/training/attempts").dispatch();) |
| GET /api/training/attempts/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | training_attempt_detail_nonexistent_returns_404 (repo/backend/src/api_tests.rs:2383:        let resp = client.get("/api/training/attempts/99999999")) |
| GET /api/training/favorites | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | training_favorites_requires_auth (repo/backend/src/api_tests.rs:818:        let resp = client.get("/api/training/favorites").dispatch();) |
| GET /api/training/review-session | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | training_review_session_returns_shape (repo/backend/src/api_tests.rs:836:            .get("/api/training/review-session")) |
| GET /api/training/wrong-notebook | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | training_wrong_notebook_requires_auth (repo/backend/src/api_tests.rs:826:        let resp = client.get("/api/training/wrong-notebook").dispatch();) |
| GET /health | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | health_root_returns_ok_when_db_healthy (repo/backend/src/api_tests.rs:1123:        let resp = client.get("/health/").dispatch();) |
| GET /health/detailed | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | e2e_role_enforcement_across_layers (repo/backend/src/api_tests.rs:1412:                .get("/health/detailed")) |
| GET /health/live | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | health_live_body_is_valid_json (repo/backend/src/api_tests.rs:354:        let resp = client.get("/health/live").dispatch();) |
| GET /health/ready | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | health_ready_returns_ok_when_no_critical_degraded (repo/backend/src/api_tests.rs:1134:        let resp = client.get("/health/ready").dispatch();) |
| GET /robots.txt | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | e2e_sitemap_and_robots_are_publicly_reachable (repo/backend/src/api_tests.rs:1540:        let robots = client.get("/robots.txt").dispatch();) |
| GET /sitemap.xml | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | e2e_sitemap_and_robots_are_publicly_reachable (repo/backend/src/api_tests.rs:1545:        let sitemap = client.get("/sitemap.xml").dispatch();) |
| POST /api/admin/products | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | admin_create_product_requires_admin (repo/backend/src/api_tests.rs:1939:        let resp = client.post("/api/admin/products")) |
| POST /api/admin/users/:param/roles | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | admin_assign_role_requires_admin (repo/backend/src/api_tests.rs:1823:        let resp = client.post("/api/admin/users/2/roles")) |
| POST /api/auth/login | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | login_response_contains_user_info (repo/backend/src/api_tests.rs:394:            .post("/api/auth/login")) |
| POST /api/auth/logout | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | e2e_customer_registration_through_session_expiry (repo/backend/src/api_tests.rs:1262:            .post("/api/auth/logout")) |
| POST /api/auth/register | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | register_and_login (repo/backend/src/api_tests.rs:1217:            .post("/api/auth/register")) |
| POST /api/cart/add | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | add_to_cart_without_required_option_group_returns_422 (repo/backend/src/api_tests.rs:274:            .post("/api/cart/add")) |
| POST /api/dispatch/accept/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | dispatch_accept_nonexistent_task_returns_conflict (repo/backend/src/api_tests.rs:2078:        let resp = client.post("/api/dispatch/accept/99999999")) |
| POST /api/dispatch/assign | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | dispatch_assign_as_admin_enqueues_for_grab (repo/backend/src/api_tests.rs:2168:        let resp = client.post("/api/dispatch/assign")) |
| POST /api/dispatch/complete/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | dispatch_complete_nonexistent_returns_404 (repo/backend/src/api_tests.rs:2141:        let resp = client.post("/api/dispatch/complete/99999999")) |
| POST /api/dispatch/grab/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | dispatch_grab_nonexistent_task_returns_conflict (repo/backend/src/api_tests.rs:2057:        let resp = client.post("/api/dispatch/grab/99999999")) |
| POST /api/dispatch/reject/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | dispatch_reject_nonexistent_returns_conflict (repo/backend/src/api_tests.rs:2099:        let resp = client.post("/api/dispatch/reject/99999999")) |
| POST /api/dispatch/shifts | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | dispatch_create_shift_as_admin (repo/backend/src/api_tests.rs:2252:        let resp = client.post("/api/dispatch/shifts")) |
| POST /api/dispatch/start/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | dispatch_start_nonexistent_returns_404 (repo/backend/src/api_tests.rs:2120:        let resp = client.post("/api/dispatch/start/99999999")) |
| POST /api/exam/generate | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | exam_generate_rejected_for_customer (repo/backend/src/api_tests.rs:1071:            .post("/api/exam/generate")) |
| POST /api/exam/import | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | exam_import_as_admin_processes_csv (repo/backend/src/api_tests.rs:2538:        let resp = client.post("/api/exam/import")) |
| POST /api/exam/questions/import | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | exam_questions_import_alias_requires_teacher_role (repo/backend/src/api_tests.rs:2556:        let resp = client.post("/api/exam/questions/import")) |
| POST /api/orders/:param/cancel | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | order_cancel_nonexistent_order_returns_404 (repo/backend/src/api_tests.rs:1729:        let resp = client.post("/api/orders/99999999/cancel")) |
| POST /api/orders/:param/confirm | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | confirm_nonexistent_order_returns_404 (repo/backend/src/api_tests.rs:802:            .post("/api/orders/99999999/confirm")) |
| POST /api/orders/checkout | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | checkout_requires_auth (repo/backend/src/api_tests.rs:1651:        let resp = client.post("/api/orders/checkout")) |
| POST /api/staff/scan | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | customer_cannot_scan_voucher_returns_403 (repo/backend/src/api_tests.rs:755:            .post("/api/staff/scan")) |
| POST /api/training/answer | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | training_answer_in_review_mode_returns_ok (repo/backend/src/api_tests.rs:2335:        let resp = client.post("/api/training/answer")) |
| POST /api/training/favorites/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | training_add_favorite_nonexistent_question_returns_500 (repo/backend/src/api_tests.rs:2407:        let resp = client.post("/api/training/favorites/1")) |
| POST /api/training/finish/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | training_finish_nonexistent_attempt_returns_404 (repo/backend/src/api_tests.rs:2362:        let resp = client.post("/api/training/finish/99999999")) |
| POST /api/training/start/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | training_start_exam_requires_auth (repo/backend/src/api_tests.rs:2302:        let resp = client.post("/api/training/start/1").dispatch();) |
| PUT /api/admin/products/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | admin_update_nonexistent_product_returns_404 (repo/backend/src/api_tests.rs:2007:        let resp = client.put("/api/admin/products/99999999")) |
| PUT /api/admin/store-hours | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | admin_update_store_hours_requires_admin (repo/backend/src/api_tests.rs:1879:        let resp = client.put("/api/admin/store-hours")) |
| PUT /api/admin/tax | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | admin_update_tax_requires_admin (repo/backend/src/api_tests.rs:1909:        let resp = client.put("/api/admin/tax")) |
| PUT /api/auth/locale | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | auth_update_locale_requires_auth (repo/backend/src/api_tests.rs:1576:        let resp = client.put("/api/auth/locale")) |
| PUT /api/cart/:param | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | cart_update_item_requires_auth (repo/backend/src/api_tests.rs:1604:        let resp = client.put("/api/cart/1")) |
| PUT /api/staff/orders/:param/status | yes | true no-mock HTTP | repo/backend/src/api_tests.rs | staff_update_order_status_invalid_transition_returns_error (repo/backend/src/api_tests.rs:1785:        let resp = client.put("/api/staff/orders/9000/status")) |

## Coverage Summary
- Total endpoints: **74**
- Endpoints with HTTP tests: **74**
- Endpoints with TRUE no-mock tests: **74**
- HTTP coverage: **100.00%**
- True API coverage: **100.00%**

## API Test Classification
1. True No-Mock HTTP
- `repo/backend/src/api_tests.rs` uses `Client::tracked(...)` and real Rocket-mounted production routes in `db_rocket()`.

2. HTTP with Mocking
- No production-endpoint mocking patterns (`jest.mock`, `vi.mock`, `sinon.stub`, DI override) found.

3. Non-HTTP (unit/integration without HTTP)
- Backend module unit tests in `repo/backend/src/services/*.rs`, `repo/backend/src/db/*.rs`, `repo/backend/src/middleware/log_mask.rs`.
- Frontend unit/contract tests in `repo/frontend/src/**/*.rs` and `repo/frontend/tests/*.rs`.

## Mock Detection
- Test-only stubs found in `repo/backend/src/api_tests.rs`: `stub_auth`, `stub_staff`, `stub_admin` (mounted under `/test/*` only).
- No evidence of mocked transport/controller/service in production endpoint tests.

## Unit Test Summary

### Backend Unit Tests
- Controllers/routes: API tested via HTTP integration (`repo/backend/src/api_tests.rs`).
- Services: tested (auth, crypto, session, pricing, fulfillment, pickup, reservation_lock, dispatch, health, resilience, csv_import).
- Repositories: tested (`repo/backend/src/db/*.rs`).
- Middleware: tested (`repo/backend/src/middleware/log_mask.rs`).
- Important backend modules not directly unit-tested: `repo/backend/src/middleware/auth_guard.rs` (no local `#[cfg(test)]` block).

### Frontend Unit Tests (STRICT REQUIREMENT)
- Frontend test files present:
  - `repo/frontend/tests/components_test.rs`
  - `repo/frontend/tests/pages_test.rs`
  - `repo/frontend/tests/render_test.rs`
- Framework/tools detected:
  - Rust `#[cfg(test)]` / `#[test]` harness.
- Direct component/module tests present in real frontend modules:
  - Components: `repo/frontend/src/components/*.rs` include test blocks.
  - Pages: `repo/frontend/src/pages/*.rs` include test blocks.
  - Shared logic/state: `repo/frontend/src/logic.rs`, `repo/frontend/src/state/mod.rs` include test blocks.
- Important frontend modules not directly tested:
  - `repo/frontend/src/main.rs`
  - `repo/frontend/src/components/mod.rs`
  - `repo/frontend/src/pages/mod.rs`
- **Frontend unit tests: PRESENT**

### Cross-Layer Observation
- Backend and frontend both have meaningful test presence.
- Full browser FE↔BE E2E exists by documentation (`e2e/tests/*.spec.ts`), but execution outcome is not statically verifiable.

## API Observability Check
- Strong: tests explicitly show method+path, inputs (JSON/cookies/query), and response checks.
- Weak: some cases assert status-only with limited payload assertions.

## Test Quality & Sufficiency
- Success/failure/edge/auth coverage: broadly strong in backend API tests.
- Over-mocking risk: low in production endpoints.
- `run_tests.sh`:
  - Docker-based: **OK**.
  - Runtime local dependency install: **Not required in run script** (uses prebuilt `Dockerfile.test`).
  - Risk: E2E run variables appear miswired (`BASE_URL`/`API_URL` point to `${DB_CONTAINER}` host), and script warns E2E may be skipped if app not running.

## End-to-End Expectations
- Fullstack expectation: FE↔BE tests should exist.
- Evidence: README declares Playwright suite under `e2e/tests/*.spec.ts`; `run_tests.sh` includes optional E2E step.
- Remaining gap: no static proof that E2E actually targets live frontend/backend containers in this script path.

## Tests Check
- Backend Endpoint Inventory: included above.
- API Test Mapping Table: included above.
- Coverage Summary: included above.
- Unit Test Summary: included above.
- Test Coverage Score (0–100): **90**
- Score Rationale:
  - + 74/74 endpoint HTTP coverage with real handlers
  - + strong backend and frontend unit coverage evidence
  - - partial uncertainty in E2E execution wiring within `run_tests.sh`
  - - some assertions are status-centric
- Key Gaps:
  - `auth_guard.rs` lacks direct local unit tests
  - E2E runner wiring/guaranteed execution path is weak in current script
- Confidence & Assumptions:
  - High confidence on static inventory/mapping
  - Assumption: concrete path tests cover `:param` template endpoints

---

# README Audit

## README Location
- Found at required path: `repo/README.md`.

## High Priority Issues
- None.

## Medium Priority Issues
- README states Dockerized E2E flow; `run_tests.sh` E2E environment wiring appears inconsistent with declared app endpoints (static mismatch risk).

## Low Priority Issues
- None.

## Hard Gate Failures
- None.

## Hard Gate Checks
- Formatting/readability: PASS
- Startup instructions (`docker-compose up`): PASS
- Access method (URL+port): PASS
- Verification method (curl/UI): PASS
- Environment rules (no forbidden manual runtime install steps in README flow): PASS
- Demo credentials with roles: PASS

## Engineering Quality
- Tech stack clarity: strong
- Architecture explanation: strong
- Testing instructions: strong, with noted E2E wiring risk
- Security/roles/workflows: strong
- Presentation quality: strong

## README Verdict
- **PASS**

## Final Verdicts
- Test Coverage Audit Verdict: **PASS (with gaps)**
- README Audit Verdict: **PASS**
