# Testing di `rust-clean`

## Dua lapis

**Unit test** — logic murni di domain service, di-mock total (repository,
cache, audit log). Cepat, tidak butuh Docker. Ditulis inline di file yang
sama (`#[cfg(test)] mod tests` di bagian bawah `service_impl.rs`), mengikuti
konvensi standar Rust.

**Integration test** — repository asli lawan MySQL beneran, via
`testcontainers`. Lambat (butuh Docker), tapi menangkap bug yang mock tidak
bisa: SQL yang salah, constraint, soft-delete generated column, dll.
Ditaruh di `tests/*.rs` (test binary terpisah per file).

## Menjalankan

```bash
# Unit test saja (cepat, tanpa Docker)
cargo test --lib

# Integration test (butuh Docker jalan)
cargo test --test api_key_repository

# Semuanya
cargo test
```

## Pola: bikin trait bisa di-mock

Repository/service trait yang mau di-unit-test butuh `mockall::automock`,
diletakkan **di atas** `#[async_trait]` (urutan ini penting — mockall
mendeteksi pola `async_trait` secara otomatis):

```rust
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ApiKeyRepository: Send + Sync {
    async fn create(&self, ...) -> Result<ApiKey>;
    // ...
}
```

Ini generate `MockApiKeyRepository` (hanya ada saat `cfg(test)`), dengan
`expect_create()`, `expect_find_by_id()`, dst — satu per method trait.

Sudah diterapkan ke:
- `ApiKeyRepository`
- `AuditLogService`
- `CacheRepository`

Untuk modul lain, tinggal tempel atribut yang sama di trait repository/
service-nya masing-masing.

## Pola: unit test service

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::api_key::repository::MockApiKeyRepository;
    use crate::domain::audit_log::services::MockAuditLogService;
    use crate::infrastructure::cache::{CacheHelper, MockCacheRepository};

    #[tokio::test]
    async fn some_business_rule() {
        let mut repo = MockApiKeyRepository::new();
        repo.expect_create().returning(|..| Ok(/* ... */));

        let mut audit = MockAuditLogService::new();
        audit.expect_record().returning(|_| ());

        let cache = CacheHelper::new(Arc::new(MockCacheRepository::new()));
        let svc = DefaultApiKeyService::new(Arc::new(repo), cache, Arc::new(audit));

        // act + assert
    }
}
```

Lihat `src/domain/api_key/services/service_impl.rs` untuk contoh lengkap —
termasuk test untuk guard privilege-escalation dan seluruh jalur `verify()`
(malformed key, secret salah, key nonaktif, key expired, key valid).

## Pola: integration test repository

`tests/common/mod.rs` menyediakan `mysql_test_pool()`: start container MySQL
sekali pakai, jalankan semua file `migrations/*.sql` terhadapnya, dan
mengembalikan pool siap pakai. Setiap test function memanggil ini sendiri
(container baru per test — lebih lambat tapi tidak ada state bocor antar
test).

```rust
mod common;

#[tokio::test]
async fn my_repo_test() {
    let (_container, pool) = common::mysql_test_pool().await.unwrap();
    let repo = MySqlApiKeyRepository::new(Arc::new(pool));
    // ...
}
```

`_container` harus tetap hidup selama test berjalan (jangan di-drop lebih
awal) — begitu di-drop, container-nya dimatikan.

**Catatan**: API persis `testcontainers-modules` (versi 0.11) untuk image
MySQL — nama user/password/database default — ditulis berdasarkan pola
dokumentasi yang tersedia, belum sempat di-`cargo check` di sini (environment
ini tidak ada toolchain Rust). Jalankan `cargo test --test api_key_repository`
duluan; kalau ada mismatch kecil (nama field/method), cocokkan dengan
`cargo doc -p testcontainers-modules --open`.

## Kenapa migration runner di `bootstrap/migration.rs` tetap no-op

File-file di `migrations/` semuanya `DROP TABLE IF EXISTS` lalu
`CREATE TABLE` — destruktif, bukan `ALTER`. Kalau ini dijalankan otomatis
tiap start aplikasi, setiap restart akan **menghapus semua data**. Jadi
`bootstrap::migration::run` sengaja dibiarkan no-op untuk aplikasi asli;
versi yang benar-benar menjalankan file ada di `tests/common/mod.rs`,
khusus untuk database sekali-pakai di test.

## Perluas ke modul lain

Untuk bikin unit test modul lain (misal `role` atau `menus`):
1. Tempel `#[cfg_attr(test, mockall::automock)]` di atas `#[async_trait]`
   pada trait repository & service (kalau service-nya dipanggil dari
   service lain).
2. Tulis `#[cfg(test)] mod tests` di bawah `service_impl.rs`-nya, pola sama
   seperti `api_key`.
3. Kalau butuh integration test, tambah file baru di `tests/`, `mod common;`
   di baris pertama, pakai `common::mysql_test_pool()`.
