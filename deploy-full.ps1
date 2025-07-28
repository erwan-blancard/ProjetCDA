# Script de deploiement complet Randomi GO sur DigitalOcean
Write-Host "Deploiement complet Randomi GO sur DigitalOcean" -ForegroundColor Green

# Verifier que doctl est installe
if (-not (Get-Command doctl -ErrorAction SilentlyContinue)) {
    Write-Host "doctl n'est pas installe. Installez-le depuis: https://docs.digitalocean.com/reference/doctl/how-to/install/" -ForegroundColor Red
    exit 1
}

# Verifier l'authentification
Write-Host "Verification de l'authentification..." -ForegroundColor Yellow
$auth = doctl auth list
if (-not $auth) {
    Write-Host "Non authentifie. Executez: doctl auth init" -ForegroundColor Red
    exit 1
}

Write-Host "Authentification OK" -ForegroundColor Green

# Demander le nom du repository GitHub
$repo = Read-Host "Entrez votre nom d'utilisateur GitHub (ex: username/ProjetCDA)"

# Mettre a jour le fichier do-app.yaml avec le bon repository
Write-Host "Mise a jour de la configuration..." -ForegroundColor Yellow
$content = Get-Content "do-app.yaml" -Raw
$content = $content -replace "repo: your-username/ProjetCDA", "repo: $repo"
$content | Set-Content "do-app.yaml"

Write-Host "Configuration mise a jour" -ForegroundColor Green

# Construire le frontend localement pour verifier
Write-Host "Construction du frontend..." -ForegroundColor Yellow
Set-Location frontend
npm install
npm run build
Set-Location ..

if ($LASTEXITCODE -ne 0) {
    Write-Host "Erreur lors de la construction du frontend" -ForegroundColor Red
    exit 1
}

Write-Host "Construction frontend reussie" -ForegroundColor Green

# Deployer l'application
Write-Host "Deploiement en cours..." -ForegroundColor Yellow
doctl apps create --spec do-app.yaml

if ($LASTEXITCODE -eq 0) {
    Write-Host "Deploiement complet reussi !" -ForegroundColor Green
    Write-Host "Votre application sera disponible dans quelques minutes." -ForegroundColor Cyan
    Write-Host "Surveillez le statut avec: doctl apps list" -ForegroundColor Cyan
    Write-Host "URLs:" -ForegroundColor Cyan
    Write-Host "   - Frontend: https://randomigo-app.ondigitalocean.app" -ForegroundColor White
    Write-Host "   - Backend API: https://randomigo-app.ondigitalocean.app/api" -ForegroundColor White
} else {
    Write-Host "Erreur lors du deploiement" -ForegroundColor Red
    exit 1
} 