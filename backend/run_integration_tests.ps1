# Script PowerShell pour lancer tous les tests d'intégration
Write-Host "=== Tests d'intégration pour Randomi GO ===" -ForegroundColor Green
Write-Host ""

# Test 1: Tests d'intégration simples
Write-Host "1. Lancement des tests d'intégration simples..." -ForegroundColor Yellow
cargo test --test simple_integration_test
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Tests simples réussis" -ForegroundColor Green
} else {
    Write-Host "❌ Tests simples échoués" -ForegroundColor Red
}
Write-Host ""

# Test 2: Tests des vraies routes
Write-Host "2. Lancement des tests des vraies routes..." -ForegroundColor Yellow
cargo test --test real_routes_test
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Tests des vraies routes réussis" -ForegroundColor Green
} else {
    Write-Host "❌ Tests des vraies routes échoués" -ForegroundColor Red
}
Write-Host ""

# Test 3: Tests des routes authentifiées
Write-Host "3. Lancement des tests des routes authentifiées..." -ForegroundColor Yellow
cargo test --test authenticated_routes_test
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Tests des routes authentifiées réussis" -ForegroundColor Green
} else {
    Write-Host "❌ Tests des routes authentifiées échoués" -ForegroundColor Red
}
Write-Host ""

# Test 4: Tests de gestion d'erreurs
Write-Host "4. Lancement des tests de gestion d'erreurs..." -ForegroundColor Yellow
cargo test --test error_handling_test
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Tests de gestion d'erreurs réussis" -ForegroundColor Green
} else {
    Write-Host "❌ Tests de gestion d'erreurs échoués" -ForegroundColor Red
}
Write-Host ""

# Test 5: Tous les tests d'intégration ensemble
Write-Host "5. Lancement de tous les tests d'intégration..." -ForegroundColor Yellow
cargo test --test "*_test"
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Tous les tests d'intégration réussis" -ForegroundColor Green
} else {
    Write-Host "❌ Certains tests d'intégration ont échoué" -ForegroundColor Red
}
Write-Host ""

Write-Host "=== Résumé des tests d'intégration ===" -ForegroundColor Green
Write-Host "Tests créés :" -ForegroundColor Cyan
Write-Host "  - simple_integration_test.rs : Tests de base d'Actix-web" -ForegroundColor White
Write-Host "  - real_routes_test.rs : Tests des vraies routes de l'app" -ForegroundColor White
Write-Host "  - authenticated_routes_test.rs : Tests avec JWT" -ForegroundColor White
Write-Host "  - error_handling_test.rs : Tests de gestion d'erreurs" -ForegroundColor White
Write-Host ""
Write-Host "Pour lancer un test spécifique :" -ForegroundColor Cyan
Write-Host "  cargo test --test nom_du_test" -ForegroundColor White
Write-Host ""
Write-Host "Pour lancer tous les tests :" -ForegroundColor Cyan
Write-Host "  cargo test --test \"*_test\"" -ForegroundColor White 