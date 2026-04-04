# Pharmacy Backend

Repositorio: pharmacy_backend

Resumen
- Backend en Rust (Axum + SeaORM + SQLx) para gestión de farmacia: usuarios, roles, permisos, inventario, ventas, pagos, etc.
- API agrupada bajo `/v1/api` (ver colección de ejemplos: [collections/root.http](collections/root.http)).

Contenido del README
- Requisitos
- Comandos comunes
- Variables de entorno (JWT y claves RSA)
- Generar claves RSA (opcional, RS256)
- Seguridad y notas sobre dependencias
- API: endpoints principales (resumen)
- Troubleshooting rápido

Requisitos
- Rust toolchain (rustc + cargo) — probar con la versión usada en CI (ej. rust 1.93+)
- OpenSSL (para generar claves RSA localmente, opcional)

Comandos útiles
- Formatear código:

```bash
cargo fmt
```

- Linter y correcciones automáticas (recomendado revisar cambios antes de commitear):

```bash
cargo clippy --all-targets --all-features
cargo clippy --fix --allow-dirty --allow-staged
```

- Ejecutar tests:

```bash
cargo test --workspace
```

- Auditar dependencias (recomendado):

```bash
cargo install cargo-audit || true
cargo audit
```

- Construir / ejecutar:

```bash
cargo build
cargo run --bin pharmacy_backend
```

Variables de entorno (esenciales)
- `API_JWT_SECRET` — secreto HMAC (HS256 fallback). Requerido si no se usa RS256.
- `API_JWT_SECRET_REFRESH` — secreto HMAC para refresh tokens.
- `API_JWT_PRIVATE_PEM` — (opcional) contenido PEM de la clave privada RSA para firmar con RS256. Si está presente, la app usará RS256.
- `API_JWT_PUBLIC_PEM` — (opcional) contenido PEM de la clave pública RSA para verificar tokens RS256.

Nota: por razones de seguridad no commits claves privadas. En entornos de producción use un gestor de secretos (Vault/Secret Manager) o monte los archivos PEM como secretos.

Ejemplo `.env` (local, opcional)

```
# HMAC (fallback)
API_JWT_SECRET=supersecret_hmac_key_very_long
API_JWT_SECRET_REFRESH=another_supersecret_refresh_key

# Opcional: RS256 PEM values (llaves largas; en despliegue prefiera secret manager)
# Puede copiar el contenido completo de jwt_private.pem e jwt_public.pem aquí
API_JWT_PRIVATE_PEM="$(cat pem/jwt_private.pem)"
API_JWT_PUBLIC_PEM="$(cat pem/jwt_public.pem)"
```

Generar claves RSA (local, ejemplo)

```bash
# Genera clave privada (2048 bits)
openssl genrsa -out pem/jwt_private.pem 2048
# Extrae la clave pública
openssl rsa -in pem/jwt_private.pem -pubout -out pem/jwt_public.pem
```

- Para cargar en variables de entorno (solo local/test):

```bash
export API_JWT_PRIVATE_PEM="$(cat pem/jwt_private.pem)"
export API_JWT_PUBLIC_PEM="$(cat pem/jwt_public.pem)"
```

Cómo funciona JWT en este repo
- El código buscará `API_JWT_PRIVATE_PEM` y `API_JWT_PUBLIC_PEM` y usará RS256 si encuentra las claves.
- Si no hay claves RSA, se usa HMAC con `API_JWT_SECRET`/`API_JWT_SECRET_REFRESH`.
- Las funciones principales: `generate_jwt`, `get_jwt_token_with_role`, `validate_token`, `validate_token_refresh` en `src/config/config_jwt/validate_jwt.rs`.

Seguridad y dependencias
- Ejecutamos `cargo audit` y se detectaron advisories transitivas (p. ej. `rsa v0.9.10` con RUSTSEC-2023-0071). Algunas recomendaciones:
  - Evitar operaciones RSA si no son necesarias y preferir HMAC con claves fuertes cuando proceda.
  - Mantener dependencias actualizadas y monitorizar advisories.
  - Usar secretos gestionados en producción (no variables de entorno largas si puede evitarse).

APIs: resumen (extraído de `collections/root.http`)
- Autenticación
  - POST `/v1/api/auth/login` — login (genera token)

- Permisos
  - PUT `/v1/api/permission` — crear/actualizar permiso
  - GET `/v1/api/permission/{id}`
  - GET `/v1/api/permission/name` — buscar por nombre
  - GET `/v1/api/permission` — list
  - PATCH `/v1/api/permission/{id}`
  - DELETE `/v1/api/permission/{id}`

- Roles
  - PUT `/v1/api/role`
  - GET `/v1/api/role/{id}`
  - GET `/v1/api/role/name`
  - GET `/v1/api/role`
  - PATCH `/v1/api/role/{id}`
  - DELETE `/v1/api/role/{id}`

- Usuarios
  - PUT `/v1/api/user` — crear usuario
  - GET `/v1/api/user/{id}`
  - GET `/v1/api/user` — listar
  - GET `/v1/api/user/username` — buscar por username
  - PATCH `/v1/api/user` — actualizar
  - DELETE `/v1/api/user` — eliminar
  - PUT `/v1/api/user/status` — actualizar estado

- Role-Permissions / User-Roles
  - PUT `/v1/api/role_permissions`
  - GET `/v1/api/role_permissions/{roleId}`
  - GET `/v1/api/role_permissions/list`
  - PATCH `/v1/api/role_permissions/{roleId}/{permissionId}`
  - DELETE `/v1/api/role_permissions/{roleId}/{permissionId}`

  - PUT `/v1/api/user_role`
  - GET `/v1/api/user_role/{userId}/{roleId}`
  - GET `/v1/api/user_role`
  - PATCH `/v1/api/user_role/{userId}/{roleId}`
  - DELETE `/v1/api/user_role/{userId}/{roleId}`

- Otros recursos (ejemplos):
  - Payment methods: `/v1/api/payment_methods`
  - Inventory locations: `/v1/api/inventory_locations`
  - Units: `/v1/api/units`
  - Tax profiles: `/v1/api/tax_profiles`

> Para ver todos los ejemplos y cuerpos de requests, abra: [collections/root.http](collections/root.http)

Buenas prácticas y siguientes pasos recomendados
- Añadir tests para endpoints críticos (auth, DTO mappings, date handling).
- Reemplazar `unwrap()`/`expect()` en puntos de producción por manejo de errores robusto. (Se identificaron usos por `cargo clippy` y búsqueda estática.)
- Considerar mover la carga de PEM desde variables largas a archivos montados o a secretos gestionados; actualizar `validate_jwt.rs` si desea `*_PATH` en lugar del contenido en la variable.
- Añadir `SECURITY.md`/`README` sección sobre cómo rotar claves y dónde mantener secretos.

Contacto / mantenimiento
- Autor: ver historial de commits en el repo.
- Para cambios de seguridad: abrir issue/PR y ejecutar `cargo audit` + `cargo clippy` en la CI.

---
Generado automáticamente con ayuda de la colección de ejemplos `collections/root.http` y el estado actual del repositorio.
