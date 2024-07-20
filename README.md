
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

To Do:
- Join `post category` data to `post`
- Join `tech` data to `project`
- Add custom ordering capability to `project` and `hobby`
- Add guest account functionality for demo purpose
- Add mailing capability for contact forms

---

#### These are the planned endpoint

`/users`
- GET: Get all available users (Authorized) - **Done**

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

`/roles`
- GET: Get all available roles (Authorized) - **Done**

`/role`
- POST: Create new role (Authorized) - **Done**

`/role/:id`
- GET: Get a role by ID (Authorized) - **Done**
- UPDATE: Update a role by ID  (Authorized) - **Done**
- DELETE: Soft delete a role by ID  (Authorized) - **Done**

`/role/:id/restore`
- POST: Restore a role by id (Authorized) - **Done**

`/posts`
- GET: Get all post
    - Have `cat` data

`/post`
- POST: Save a new post (Authorized) - **Done**

`/post/:slug`
- GET: Get a post by slug
- UPDATE: Update a post by slug  (Authorized) - **Done**
- DELETE: Soft delete a post by slug  (Authorized) - **Done**

`/post/:slug/restore`
- POST: Restore a post by slug (Authorized) - **Done**

`/post-categories`
- GET: Get all post categories (Authorized) - **Done**

`/post-category`
- POST: Save new post category (Authorized) - **Done**

`/post-category/:id`
- GET: Get a post category by ID  (Authorized) - **Done**
- UPDATE: Update a post category by ID  (Authorized) - **Done**
- DELETE: Soft delete a post category by ID  (Authorized) - **Done**

`/post-category/:id/restore`
- POST: Restore a post category by ID (Authorized) - **Done**

`/projects`
- GET: Get all project lists
    - Have `tech` list data per project

`/project`
- POST: Save new project (Authorized) - **Done**

`/project/:id`
- GET: Get a project by ID (Authorized) - **Done**
- UPDATE: Update a project by ID  (Authorized) - **Done**
- DELETE: Soft delete a project by ID  (Authorized) - **Done**

`/project/:id/restore`
- POST: Restore a project by ID (Authorized) - **Done**

`/techs`
- GET: Get all tech stack (Authorized) - **Done**

`/tech`
- POST: Create new tech stack (Authorized) - **Done**

`/tech/:id`
- GET: Get a tech by ID (Authorized) - **Done**
- UPDATE: Update a tech by ID  (Authorized) - **Done**
- DELETE: Soft delete a tech by ID  (Authorized) - **Done**

`/tech/:id/restore`
- POST: Restore a tech by ID (Authorized) - **Done**

`/hobbies`
- GET: Get all hobbies lists - **Done**

`/hobby`
- POST: Save new hobby (Authorized) - **Done**

`/hobby/:id`
- GET: Get a hobby by ID (Authorized) - **Done**
- UPDATE: Update a hobby by ID  (Authorized) - **Done**
- DELETE: Soft delete a hobby by ID  (Authorized) - **Done**

`/hobby/:id/restore`
- POST: Restore a hobby by ID (Authorized) - **Done**

`/settings`
- GET: Get all settings (Authorized) - **Done**

`/setting`
- POST: Set a global param  (Authorized) - **Done**

`/setting/:param`
- GET: Get a global param - **Done**
- UPDATE: Update a global param  (Authorized) - **Done**
- DELETE: Delete a global param  (Authorized) - **Done**

`/setting/:param`
- POST: Restore a global param (Authorized) - **Done**

`/contact`
- POST: Send email from a form