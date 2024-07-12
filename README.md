
# Sedikit Acak V3 - API

A *blazing fast* API for my personal site Sedikit Acak using Rust.

Notable library used:

- Actix Web
- Diesel - PostgreSQL
- Json Web Token
- Bcrypt
- Chrono
- Serde

**Project Still On Going!**

---

#### These are the planned endpoint

`/user`
- POST: Create new users (Disabled) - **Done**

`/user/:id`
- GET: Get a user by ID - **Done**
- UPDATE: Update a user by ID (Authorized) - **Done**
- DELETE: Soft delete a user by ID (Authorized) - **Done**

`/user/:id/restore`
- POST: Restore a user by ID (Authorized) - **Done**

`/login`
- POST: Login to Authorized Page - **Done**

`/posts`
- GET: Get all post limited by 10 per page
    - Have `page` parameter
    - Have `cat` parameter

`/post`
- POST: Save a new post (Authorized) - **Done**

`/post/:id`
- GET: Get a post by slug - **Done**
- UPDATE: Update a post by slug  (Authorized) - **Done**
- DELETE: Soft delete a post by slug  (Authorized) - **Done**

`/post/:id/restore`
- POST: Restore a post by slug (Authorized) - **Done**

`/post-category`
- POST: Save new post category (Authorized) - **Done**

`/post-category/:id`
- UPDATE: Update a post category by ID  (Authorized) - **Done**
- DELETE: Soft delete a post category by ID  (Authorized) - **Done**

`/post-category/:id/restore`
- POST: Restore a post category by slug (Authorized) - **Done**

`/projects`
- GET: Get all project lists by 12 per page

`/project`
- POST: Save new project (Authorized)

`/project/:id`
- GET: Get a project by ID
- UPDATE: Update a project by ID  (Authorized)
- DELETE: Soft delete a project by ID  (Authorized)

`/project/:id/restore`
- POST: Restore a project by slug (Authorized)

`/hobbies`
- GET: Get all hobbies lists

`/hobby`
- POST: Save new hobby (Authorized)

`/hobby/:id`
- GET: Get a hobby by ID
- UPDATE: Update a hobby by ID  (Authorized)
- DELETE: Soft delete a hobby by ID  (Authorized)

`/hobby/:id/restore`
- POST: Restore a hobby by slug (Authorized)

`/setting`
- POST: Set a global param  (Authorized)

`/setting/:param`
- GET: Get a global param
- UPDATE: Update a global param  (Authorized)
- DELETE: Delete a global param  (Authorized)

`/contact`
- POST: Send email from a form