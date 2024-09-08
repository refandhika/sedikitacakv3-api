
# Sedikit Acak V3 - API

A *blazing fast* API for my personal site Sedikit Acak using Rust.

Notable library used:

- Actix Web
- Diesel - PostgreSQL
- Json Web Token
- Bcrypt
- Chrono
- Serde
- Lettre

---

To Do:
- Improve mail send endpoint protection
- Add guest account functionality for demo purpose
- Make documentation for the endpoint requests

---

`/users`
- GET: Get all available users (Authorized)

`/user`
- POST: Create new users - **Currently Disabled**

`/user/:id`
- GET: Get a user by ID
- UPDATE: Update a user by ID (Authorized)
- DELETE: Soft delete a user by ID (Authorized)

`/user/:id/restore`
- POST: Restore a user by ID (Authorized)

`/login`
- POST: Login to Authorized Page

`/guest`
- POST: Login with demo capability - **Not Available**

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

`/posts/active`
- GET: Get all post

`/posts`
- GET: Get all post (Authorized)

`/post`
- POST: Save a new post (Authorized)

`/post/:slug`
- GET: Get a post by slug
- UPDATE: Update a post by slug  (Authorized)
- DELETE: Soft delete a post by slug  (Authorized)

`/post/:slug/restore`
- POST: Restore a post by slug (Authorized)

`/post-categories/active`
- GET: Get all post categories (Authorized)

`/post-categories`
- GET: Get all post categories (Authorized)

`/post-category`
- POST: Save new post category (Authorized)

`/post-category/:id`
- GET: Get a post category by ID  (Authorized)
- UPDATE: Update a post category by ID  (Authorized)
- DELETE: Soft delete a post category by ID  (Authorized)

`/post-category/:id/restore`
- POST: Restore a post category by ID (Authorized)

`/projects`
- GET: Get all project lists

`/project`
- POST: Save new project (Authorized)

`/project/:id`
- GET: Get a project by ID (Authorized)
- UPDATE: Update a project by ID  (Authorized)
- DELETE: Soft delete a project by ID  (Authorized)

`/project/:id/restore`
- POST: Restore a project by ID (Authorized)

`/techs`
- GET: Get all tech stack (Authorized)

`/tech`
- POST: Create new tech stack (Authorized)

`/tech/:id`
- GET: Get a tech by ID (Authorized)
- UPDATE: Update a tech by ID  (Authorized)
- DELETE: Soft delete a tech by ID  (Authorized)

`/tech/:id/restore`
- POST: Restore a tech by ID (Authorized)

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

`/settings`
- GET: Get all settings (Authorized)

`/setting`
- POST: Set a global param  (Authorized)

`/setting/:param`
- GET: Get a global param
- UPDATE: Update a global param  (Authorized)
- DELETE: Delete a global param  (Authorized)

`/setting/:param`
- POST: Restore a global param (Authorized)

`/images`
- Get: Get all images (Authorized)

`/image`
- POST: Upload an image (Authorized)
- GET: Get an image (Authorized)
- DELETE: Delete an image (Authorized)

`/assets/:filename`
- GET: Serve image on the web

`/contact`
- POST: Send email from a form 