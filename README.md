# Learn AXUM

## Rust Axum Full Course 

### Beginner 

- First Router Hello World
- Quick Dev 
- Hello Params / Path
- Static File Router

## Intermediate

- First API Login (with frist error)
  - Add login API.
- First Response Mapper Layer
- Login with cookie
  - After login, we should add token into the cookie.
- Mock Model for CRUD
- REST API

## Advanced 

- Custom Middelware Auth `mw_auth` (vido: 40:38)
  - Use this to protect certain API, like create ticket and delete ticket.
  - parse token from cookie
- Custom Extractor `ctx`
  - Created a general context extract for header. 
- Ctx from REST to CRUD Model API
  - Use ctx for privileges and access control for web layer and model layer.
  - Use ctx extractor to extract user id for create and delete ticket API.
- Ctx middleware resolver (optimization) -- from (58:25) to (1:04:40)
  - Ctx extractor called two time problem. One from require auth middleware, another from create ticket.
- Advanced Error Handling (Client vs Server)
- Server Logging
  - This is not tracing. 
  - This is request log line: one log line per request with error or with other information.

## References 

- [Rust Axum Full Course - Web Development (GitHub repo updated to Axum 0.7)](https://www.youtube.com/watch?v=XZtlD_m59sM&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q&index=37)
- [Rust Axum Production Coding (E01 - Rust Web App Production Coding)](https://www.youtube.com/watch?v=3cA_mk4vdWY)