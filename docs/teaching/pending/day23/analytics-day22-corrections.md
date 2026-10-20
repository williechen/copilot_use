# Day23 第2節追加單元：Day22 更正補充

狀態：pending，尚未發布 Day23。執行前須重讀 main、規則、套件、makeup_queue；預期 next_day=23，但本文件不推進計數。只在本日新 Red／完整 Green／Refactor 結束後追加，不取代本日新實作。依 FIFO，每篇最多一個舊 Day 單元，發布與全文讀回後才可標結案。

來源核對基底 `cabacb75e946b03c67c455ce13a7a4ea85652f8e`，規則 daily-tdd-minimum-v1 ＋ 2026-10-20 封存順序補充。

## 舊 Day 更正補充（程式／套件差異）：Analytics Day22

main已有 bootstrap.rs與bootstrap_admin CLI，不重複新增。使用workspace argon2=0.6、Tokio1.54、rpassword7.5；lock為0.6.0／1.54.0／7.5.4。官方PasswordHasher 0.6的單參數hash_password會自動產生隨機鹽，不應誤改回0.5 API或判為無鹽。例：

```rust
use argon2::{Argon2, PasswordHasher};
let hash = Argon2::default()
    .hash_password(password.as_bytes())
    .map_err(|_| "密碼雜湊失敗")?
    .to_string();
```

來源：https://docs.rs/argon2/0.6.0/argon2/trait.PasswordHasher.html 。此snippet是在現有blocking工作內的雜湊片段，不是完整新初始化實作；保留現有transaction／表鎖／重複admin拒絕。

目前initialize接受8–128 bytes，舊Day22為12–128；下一篇明示行為差異並與當日需求對齊，不自行把既有8改回12。is_init_pd=true只是資料標記；舊key handler尚未查uac_user，不能宣稱已實現帳密登入或強制改密碼。

本輪cargo test --locked -p stats-server --features postgres-integration -- --include-ignored，在專用PostgreSQL18.6上26 passed／0 failed／0 ignored。包含並發初始化／密碼hash保持、既有key登入查詢登出；互動CLI、新帳密Session、改密碼、角色授權尚未驗收。第3、4節同步新版版本與證據，不借用舊候選成功結論。本單元待下一Day新實作後發布並讀回，尚未結案。
