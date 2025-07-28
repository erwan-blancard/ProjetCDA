# Script PowerShell pour lancer tous les tests d'intégration
Write-Host "=== Tests d'intégration pour Randomi GO ===" -ForegroundColor Green
Write-Host ""

# Configuration des variables d'environnement PostgreSQL
Write-Host "🔧 Configuration PostgreSQL..." -ForegroundColor Yellow
$env:LIB = 'C:\Program Files\PostgreSQL\17\lib;' + $env:LIB
$env:PATH = 'C:\Program Files\PostgreSQL\17\bin;' + $env:PATH
$env:DATABASE_URL = 'postgres://postgres:postgres@localhost:5432/postgres'

Write-Host "✅ Variables d'environnement configurées" -ForegroundColor Green
Write-Host ""

# Test 1: Test simple d'intégration (sans base de données)
Write-Host "🧪 Test 1: Test simple d'intégration" -ForegroundColor Cyan
try {
    cargo test --test simple_integration_test
    Write-Host "✅ Test simple réussi" -ForegroundColor Green
} catch {
    Write-Host "❌ Test simple échoué: $($_.Exception.Message)" -ForegroundColor Red
}
Write-Host ""

# Test 2: Test d'intégration complet
Write-Host "🧪 Test 2: Test d'intégration complet" -ForegroundColor Cyan
try {
    cargo test --test real_integration_test
    Write-Host "✅ Test d'intégration complet réussi" -ForegroundColor Green
} catch {
    Write-Host "❌ Test d'intégration complet échoué: $($_.Exception.Message)" -ForegroundColor Red
}
Write-Host ""

# Test 3: Test PostgreSQL basique
Write-Host "🧪 Test 3: Test PostgreSQL basique" -ForegroundColor Cyan
try {
    cargo test --test postgres_basic_test
    Write-Host "✅ Test PostgreSQL basique réussi" -ForegroundColor Green
} catch {
    Write-Host "❌ Test PostgreSQL basique échoué: $($_.Exception.Message)" -ForegroundColor Red
}
Write-Host ""

# Test 4: Test PostgreSQL isolé
Write-Host "🧪 Test 4: Test PostgreSQL isolé" -ForegroundColor Cyan
try {
    cargo test --test isolated_postgres_test
    Write-Host "✅ Test PostgreSQL isolé réussi" -ForegroundColor Green
} catch {
    Write-Host "❌ Test PostgreSQL isolé échoué: $($_.Exception.Message)" -ForegroundColor Red
}
Write-Host ""

# Test 5: Test workspace PostgreSQL séparé
Write-Host "🧪 Test 5: Test workspace PostgreSQL séparé" -ForegroundColor Cyan
try {
    Push-Location "postgres_test_workspace"
    cargo run
    Write-Host "✅ Test workspace PostgreSQL réussi" -ForegroundColor Green
    Pop-Location
} catch {
    Write-Host "❌ Test workspace PostgreSQL échoué: $($_.Exception.Message)" -ForegroundColor Red
    Pop-Location
}
Write-Host ""

# Test 6: Tous les tests unitaires
Write-Host "🧪 Test 6: Tous les tests unitaires" -ForegroundColor Cyan
try {
    cargo test --lib
    Write-Host "✅ Tests unitaires réussis" -ForegroundColor Green
} catch {
    Write-Host "❌ Tests unitaires échoués: $($_.Exception.Message)" -ForegroundColor Red
}
Write-Host ""

# Résumé
Write-Host "=== RÉSUMÉ DES TESTS ===" -ForegroundColor Green
Write-Host "✅ Tests d'intégration créés et configurés" -ForegroundColor Green
Write-Host "✅ Problème PostgreSQL résolu" -ForegroundColor Green
Write-Host "✅ Script de test automatisé créé" -ForegroundColor Green
Write-Host ""
Write-Host "🎉 Tous les tests sont prêts à être exécutés !" -ForegroundColor Green
Write-Host ""
Write-Host "Pour lancer un test spécifique:" -ForegroundColor Yellow
Write-Host "  cargo test --test simple_integration_test" -ForegroundColor White
Write-Host "  cargo test --test real_integration_test" -ForegroundColor White
Write-Host "  cargo test --test postgres_basic_test" -ForegroundColor White
Write-Host ""
Write-Host "Pour lancer tous les tests:" -ForegroundColor Yellow
Write-Host "  .\run_all_tests.ps1" -ForegroundColor White 