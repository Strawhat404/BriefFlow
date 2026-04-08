# BriefFlow — API Specification

Base URL: `http://localhost:8000`

All responses follow the envelope: `{ "data": <T>, "error": null }` or `{ "data": null, "error": "<message>" }`.

Authentication: session cookie `brewflow_session` (HMAC-signed). All protected endpoints require this cookie. There is no JWT bearer fallback — cookie-only authentication is enforced at the guard level.

---

## Auth

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | `/api/auth/login` | — | Login with username/password; sets `brewflow_session` cookie and returns `{ session_cookie, user }` |
| POST | `/api/auth/logout` | ✓ | Invalidate session |
| GET | `/api/auth/me` | ✓ | Get current user profile |
| POST | `/api/auth/register` | — | Register new customer account |

---

## Products

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/api/products` | — | List products with filters |
| GET | `/api/products/:id` | — | Get product detail |
| POST | `/api/products` | Admin | Create product |
| PUT | `/api/products/:id` | Admin | Update product |

---

## Cart

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/api/cart` | Customer | Get current cart |
| POST | `/api/cart/items` | Customer | Add item to cart |
| PUT | `/api/cart/items/:id` | Customer | Update cart item quantity |
| DELETE | `/api/cart/items/:id` | Customer | Remove cart item |
| POST | `/api/cart/voucher` | Customer | Apply voucher code |

---

## Store & Pickup Slots

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/api/stores` | — | List stores |
| GET | `/api/stores/:id/slots` | Customer | Get available pickup slots |

---

## Orders

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | `/api/orders` | Customer | Place order (checkout) |
| GET | `/api/orders` | Customer | List own orders |
| GET | `/api/orders/:id` | Customer | Get order detail |
| POST | `/api/orders/:id/cancel` | Customer/Admin | Cancel order |

---

## Staff / Fulfilment

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/api/staff/orders` | Staff | List orders for fulfilment |
| PUT | `/api/staff/orders/:id/status` | Staff | Transition order status |
| GET | `/api/staff/dispatch` | Staff | List dispatch tasks |
| POST | `/api/staff/dispatch/:id/accept` | Staff | Accept dispatch task |

---

## Training & Exams

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/api/training/modules` | Staff | List training modules |
| GET | `/api/training/modules/:id` | Staff | Get module detail |
| POST | `/api/exams/:id/start` | Staff | Start exam attempt |
| POST | `/api/exams/:id/submit` | Staff | Submit exam answers |
| GET | `/api/exams/attempts` | Staff | List own attempts |

---

## Admin

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/api/admin/users` | Admin | List users |
| PUT | `/api/admin/users/:id/role` | Admin | Change user role |
| GET | `/api/admin/dashboard` | Admin | Dashboard metrics |
| POST | `/api/admin/products/import` | Admin | Bulk import products via CSV |

---

## Health

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/health` | — | Basic health check |
| GET | `/health/detailed` | Admin | Detailed health + job status |
