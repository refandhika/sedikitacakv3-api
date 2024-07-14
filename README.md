
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
- Set up all main CRUD endpoint
- Join `post category` data to `post`
- Join `tech` data to `project`
- Add custom ordering capability to `project` and `hobby`
- Add guest account functionality for demo purpose
- Add mailing capability for contact forms

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

`/roles`
- GET: Get all available roles (Authorized)

`/role`
- POST: Create new role (Authorized)

`/role/:id`
- GET: Get a role by ID (Authorized)
- UPDATE: Update a role by ID  (Authorized)
- DELETE: Soft delete a role by ID  (Authorized)

`/role/:id/restore`
- POST: Restore a role by id (Authorized)

`/posts`
- GET: Get all post limited by 10 per page
    - Have `page` parameter
    - Have `cat` parameter
    - Have `cat` data

`/post`
- POST: Save a new post (Authorized) - **Done**

`/post/:slug`
- GET: Get a post by slug
- UPDATE: Update a post by slug  (Authorized) - **Done**
- DELETE: Soft delete a post by slug  (Authorized) - **Done**

`/post/:slug/restore`
- POST: Restore a post by slug (Authorized) - **Done**

`/post-category`
- POST: Save new post category (Authorized) - **Done**

`/post-category/:id`
- GET: Get a post category by ID  (Authorized) - **Done**
- UPDATE: Update a post category by ID  (Authorized) - **Done**
- DELETE: Soft delete a post category by ID  (Authorized) - **Done**

`/post-category/:id/restore`
- POST: Restore a post category by ID (Authorized) - **Done**

`/projects`
- GET: Get all project lists by 12 per page
    - Have `relevant` parameter
    - Have `tech` list data per project

`/project`
- POST: Save new project (Authorized) - **Done**

`/project/:id`
- GET: Get a project by ID (Authorized)
- UPDATE: Update a project by ID  (Authorized) - **Done**
- DELETE: Soft delete a project by ID  (Authorized) - **Done**

`/project/:id/restore`
- POST: Restore a project by ID (Authorized) - **Done**

`/techs`
- GET: Get tech stack by 20 per page (Authorized)

`/tech`
- POST: Create new tech stack (Authorized) - **Done**

`/tech/:id`
- GET: Get a tech by ID (Authorized) - **Done**
- UPDATE: Update a tech by ID  (Authorized) - **Done**
- DELETE: Soft delete a tech by ID  (Authorized) - **Done**

`/tech/:id/restore`
- POST: Restore a tech by ID (Authorized) - **Done**

`/hobbies`
- GET: Get all hobbies lists

`/hobby`
- POST: Save new hobby (Authorized)

`/hobby/:id`
- GET: Get a hobby by ID (Authorized)
- UPDATE: Update a hobby by ID  (Authorized)
- DELETE: Soft delete a hobby by ID  (Authorized)

`/hobby/:id/restore`
- POST: Restore a hobby by ID (Authorized)

`/setting`
- POST: Set a global param  (Authorized)

`/setting/:param`
- GET: Get a global param
- UPDATE: Update a global param  (Authorized)
- DELETE: Delete a global param  (Authorized)

`/setting/:param`
- POST: Restore a global param (Authorized)

`/contact`
- POST: Send email from a form