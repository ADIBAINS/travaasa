# Travaasa

Travaasa lets travelers find and save stays and lets hosts publish their own listings. It runs on a Rust and Axum API, a SvelteKit 3 and TypeScript frontend, and MongoDB.

## Run locally

You need Rust 1.100 or newer, Node.js 22.17 or newer, and a local MongoDB instance or Atlas cluster.

1. Copy `.env.example` to `.env`. Set `ATLASDB_URL` to your MongoDB connection string.
2. Start the API from the project root.

   ```sh
   cargo run --locked --manifest-path server/Cargo.toml
   ```

3. Start the frontend in a second terminal.

   ```sh
   npm ci --prefix web
   npm run dev --prefix web
   ```

4. Open http://localhost:5173. The API listens on http://127.0.0.1:3001 and exposes a database readiness check at `/health`.

Sign up, then select **Host a stay** to publish the first listing on an empty database. Search supports partial matches by title, location, and country. You can combine the search with a category and price range and sort by price or newest.

To load the sample listings into a fresh local database, run `mongoimport` from the MongoDB Database Tools.

```sh
mongoimport --uri mongodb://127.0.0.1:27017/travaasa --collection listings --jsonArray --file init/listings.json
```

## Configuration

- `ATLASDB_URL` uses the database in the connection string. With no database in a supplied URI it falls back to `test`, following the old Mongoose default. Set `MONGODB_DATABASE` to override it explicitly.
- Cloudinary photo uploads use `CLOUD_NAME`, `CLOUD_API_KEY`, and `CLOUD_API_SECRET`. Existing image URLs continue to display. Publishing without a photo works without Cloudinary credentials.
- `MAPBOX_TOKEN` enables permanent geocoding when creating a stay or changing its location. The listing address still works as a map search link when geocoding is unavailable. Set `PUBLIC_MAPBOX_TOKEN` in the web process to show a Mapbox preview for stays with coordinates.
- The web process reads `API_URL` from its environment. The default is `http://127.0.0.1:3001`. The browser communicates with SvelteKit, which forwards authenticated requests to the Rust API.

## Existing data

The new API reads the existing `users`, `listings`, and `reviews` collections. Back up your database and verify the cutover against a restored copy first. At cutover, users sign in again. On a successful sign-in, an existing Passport password hash is replaced with Argon2id. A rollback to the Express version requires restoring compatible account data from the backup. Existing reviews remain visible and new reviews are stored with their listing. Older listings appear in normal search and can be categorized by their host.

## Production

Build the API with `cargo build --locked --release --manifest-path server/Cargo.toml` and run `server/target/release/travaasa` on a private listener. Start the public SSR process with `npm ci --prefix web`, `npm run build --prefix web`, and `npm run start --prefix web`.

Set `API_URL` to the private API address, `ORIGIN` to the public HTTPS URL, and `BODY_SIZE_LIMIT=6M` on the SSR process to support 5 MB photos.

## Checks

```sh
cargo fmt --manifest-path server/Cargo.toml --check
cargo clippy --locked --manifest-path server/Cargo.toml -- -D warnings
npm run check --prefix web
npm run build --prefix web
```

Travaasa is licensed under the [MIT License](LICENSE).
