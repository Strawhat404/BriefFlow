# BrewFlow API Specification

## Authentication

**Method: HMAC-signed session cookie (`brewflow_session`)**

All authenticated endpoints require a valid `brewflow_session` cookie.  The
cookie is HMAC-SHA256-signed with `COOKIE_SECRET`, stored as `HttpOnly`,
`SameSite=Strict`, `Secure`.

**WASM / API clients** that cannot auto-send cross-origin cookies must read
the `session_cookie` field returned by `POST /api/auth/login` and replay it
as `Cookie: brewflow_session=<value>` on every request.

The only accepted credential is the signed session cookie.  The
`Authorization` header is not read by any endpoint.

### Guard types

| Guard               | Meaning                                                 |
|---------------------|---------------------------------------------------------|
| None                | Public — no cookie required                             |
| AuthenticatedUser   | Any signed-in user                                      |
| Optional            | Works without auth; auth adds extra context              |
| StaffGuard          | Requires `Staff` or `Admin` role                        |
| AdminGuard          | Requires `Admin` role                                   |
| TeacherGuard        | Requires `Teacher`, `AcademicAffairs`, or `Admin` role  |

---

## Routes

### Auth — mounted at `/api/auth`

| Method | Path      | Guard              | Description                    |
|--------|-----------|--------------------|--------------------------------|
| POST   | `/login`  | None               | Authenticate and create session|
| POST   | `/register` | None             | Create new user account        |
| POST   | `/logout` | None               | Destroy session                |
| GET    | `/me`     | AuthenticatedUser  | Return current user info       |
| PUT    | `/locale` | AuthenticatedUser  | Update preferred locale        |

**POST /api/auth/login** request:
```json
{ "username": "string", "password": "string" }
```
Response `200`:
```json
{
  "success": true,
  "data": {
    "session_cookie": "<signed-cookie-value>",
    "user": { "id": 1, "username": "...", "roles": ["Customer"], "preferred_locale": "en" }
  }
}
```

### Products — mounted at `/api/products`

| Method | Path    | Guard | Description                                    |
|--------|---------|-------|------------------------------------------------|
| GET    | `/`     | None  | List SPUs (`?featured=true&limit=N`)           |
| GET    | `/<id>` | None  | Product detail with option groups              |

### Cart — mounted at `/api/cart`

| Method | Path          | Guard             | Description       |
|--------|---------------|--------------------|-------------------|
| GET    | `/`           | AuthenticatedUser | Get current cart  |
| POST   | `/add`        | AuthenticatedUser | Add item to cart  |
| PUT    | `/<item_id>`  | AuthenticatedUser | Update item qty   |
| DELETE | `/<item_id>`  | AuthenticatedUser | Remove item       |
| DELETE | `/clear`      | AuthenticatedUser | Clear entire cart |

**POST /api/cart/add** enforces required option groups.  Requests that omit a
selection for a required group receive `422 Unprocessable Entity`.

### Orders — mounted at `/api/orders`

| Method | Path              | Guard             | Description                                              |
|--------|-------------------|--------------------|----------------------------------------------------------|
| POST   | `/checkout`       | AuthenticatedUser | Create order from cart                                   |
| GET    | `/`               | AuthenticatedUser | List user's orders                                       |
| GET    | `/<id>`           | AuthenticatedUser | Order detail                                             |
| POST   | `/<id>/confirm`   | AuthenticatedUser | Confirm order (requires Held reservation, unexpired hold)|
| POST   | `/<id>/cancel`    | AuthenticatedUser | Cancel order (Pending or Accepted only)                  |

### Staff — mounted at `/api/staff`

| Method | Path                  | Guard      | Description                                  |
|--------|-----------------------|------------|----------------------------------------------|
| GET    | `/orders`             | StaffGuard | List all orders (`?status=` filter)          |
| GET    | `/orders/<id>`        | StaffGuard | Order detail                                 |
| PUT    | `/orders/<id>/status` | StaffGuard | Transition order status                      |
| POST   | `/scan`               | StaffGuard | Scan pickup voucher (validates order state)  |
| GET    | `/dashboard`          | StaffGuard | Dashboard stats                              |
| GET    | `/dashboard/counts`   | StaffGuard | Dashboard stats (alias)                      |

### Training — mounted at `/api/training`

| Method | Path                       | Guard             | Description                         |
|--------|----------------------------|--------------------|-------------------------------------|
| POST   | `/start/<exam_id>`         | AuthenticatedUser | Start exam attempt                  |
| POST   | `/answer`                  | AuthenticatedUser | Submit answer (exam or review mode) |
| POST   | `/finish/<attempt_id>`     | AuthenticatedUser | Finish exam attempt                 |
| GET    | `/attempts`                | AuthenticatedUser | List user's attempts                |
| GET    | `/attempts/<id>`           | AuthenticatedUser | Attempt detail                      |
| GET    | `/analytics`               | AuthenticatedUser | Score analytics                     |
| POST   | `/favorites/<question_id>` | AuthenticatedUser | Add question to favorites           |
| DELETE | `/favorites/<question_id>` | AuthenticatedUser | Remove question from favorites      |
| GET    | `/favorites`               | AuthenticatedUser | List favorited questions            |
| GET    | `/wrong-notebook`          | AuthenticatedUser | Wrong-answer notebook               |
| GET    | `/review-session`          | AuthenticatedUser | Wrong-answer review queue           |

**POST /api/training/answer** request:
```json
{
  "attempt_id": 42,
  "question_id": 7,
  "selected_option_ids": [3]
}
```
`attempt_id` may be omitted or `null` for review-mode submissions.

Response `200`:
```json
{
  "success": true,
  "data": {
    "is_correct": false,
    "correct_option_ids": [2]
  }
}
```

### Exam — mounted at `/api/exam`

| Method | Path                          | Guard             | Description             |
|--------|-------------------------------|--------------------|-------------------------|
| GET    | `/subjects`                   | None              | List exam subjects      |
| GET    | `/subjects/<id>/chapters`     | None              | Chapters for a subject  |
| GET    | `/questions`                  | TeacherGuard      | Search / list questions (`?subject_id&chapter_id&difficulty&q&page&per_page`) |
| GET    | `/questions/<id>`             | TeacherGuard      | Question detail         |
| POST   | `/questions/import`           | TeacherGuard      | Bulk-import questions   |
| POST   | `/import`                     | TeacherGuard      | Import questions (alias)|
| POST   | `/generate`                   | TeacherGuard      | Generate exam paper     |
| GET    | `/versions`                   | AuthenticatedUser | List exam versions      |
| GET    | `/versions/<id>`              | AuthenticatedUser | Exam version detail     |

### Admin — mounted at `/api/admin`

| Method | Path                       | Guard      | Description                  |
|--------|----------------------------|------------|------------------------------|
| GET    | `/users`                   | AdminGuard | List all users               |
| POST   | `/users/<id>/roles`        | AdminGuard | Assign role to user          |
| DELETE | `/users/<id>/roles/<role>` | AdminGuard | Remove role from user        |
| PUT    | `/store-hours`             | AdminGuard | Update store operating hours |
| PUT    | `/tax`                     | AdminGuard | Update sales tax rate        |
| POST   | `/products`                | AdminGuard | Create product (SPU)         |
| PUT    | `/products/<id>`           | AdminGuard | Update product               |

### Store — mounted at `/api/store`

| Method | Path            | Guard    | Description                                  |
|--------|-----------------|----------|----------------------------------------------|
| GET    | `/hours`        | None     | Store operating hours                        |
| GET    | `/pickup-slots` | Optional | Available pickup windows (`?date&prep_time`) |
| GET    | `/tax`          | None     | Active sales tax rate                        |

### Dispatch — mounted at `/api/dispatch`

| Method | Path                          | Guard      | Description                    |
|--------|-------------------------------|------------|--------------------------------|
| GET    | `/zones`                      | StaffGuard | List dispatch zones            |
| GET    | `/queue`                      | StaffGuard | Queued tasks (`?zone_id`)      |
| POST   | `/grab/<task_id>`             | StaffGuard | Grab a queued task             |
| POST   | `/accept/<task_id>`           | StaffGuard | Accept an offered task         |
| POST   | `/reject/<task_id>`           | StaffGuard | Reject an offered task         |
| POST   | `/start/<task_id>`            | StaffGuard | Start working on task          |
| POST   | `/complete/<task_id>`         | StaffGuard | Complete task                  |
| GET    | `/my-tasks`                   | StaffGuard | Current user's assigned tasks  |
| POST   | `/assign`                     | AdminGuard | Auto-assign order to best staff|
| GET    | `/recommendations/<order_id>` | AdminGuard | Ranked staff recommendations  |
| GET    | `/shifts`                     | StaffGuard | Query shift windows (`?user_id&date`) |
| POST   | `/shifts`                     | AdminGuard | Create shift window            |
| GET    | `/reputation/<user_id>`       | AdminGuard | Staff reputation score         |

### Internationalization — mounted at `/api/i18n`

| Method | Path                     | Guard | Description                  |
|--------|--------------------------|-------|------------------------------|
| GET    | `/translations/<locale>` | None  | Translation bundle for locale|
| GET    | `/locales`               | None  | List available locales       |

### Sitemap — mounted at `/`

| Method | Path           | Guard | Description      |
|--------|----------------|-------|------------------|
| GET    | `/sitemap.xml` | None  | XML sitemap      |
| GET    | `/robots.txt`  | None  | Robots directive |

### Health — mounted at `/health`

| Method | Path        | Guard      | Description                                           |
|--------|-------------|------------|-------------------------------------------------------|
| GET    | `/`         | None       | Basic health — 200 if DB reachable, 503 otherwise     |
| GET    | `/live`     | None       | Liveness probe — always 200 if process is up          |
| GET    | `/ready`    | None       | Readiness probe — 200 if DB + critical services OK    |
| GET    | `/detailed` | AdminGuard | Full health report: jobs, degradation state, uptime   |

---

## Common response envelope

```json
{
  "success": true,
  "data": "<payload or null>",
  "error": "message or null"
}
```

Paginated endpoints wrap data in:
```json
{ "items": [...], "total": 100, "page": 1, "per_page": 20 }
```

## Error codes

| HTTP | Meaning                                         |
|------|-------------------------------------------------|
| 400  | Bad request / business rule violation           |
| 401  | Missing or invalid session cookie               |
| 403  | Authenticated but insufficient role             |
| 404  | Resource not found                              |
| 409  | Conflict (e.g. reservation hold expired)        |
| 422  | Validation error (e.g. missing required option) |
| 500  | Internal server error                           |
