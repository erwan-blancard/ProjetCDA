# Script de deploiement DigitalOcean
Write-Host "Deploiement Randomi GO sur DigitalOcean" -ForegroundColor Green

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
$content = $content -replace "your-username/ProjetCDA", $repo
$content | Set-Content "do-app.yaml"

Write-Host "Configuration mise a jour" -ForegroundColor Green

# Deployer l'application
Write-Host "Deploiement en cours..." -ForegroundColor Yellow
doctl apps create --spec do-app.yaml

if ($LASTEXITCODE -eq 0) {
    Write-Host "Deploiement reussi !" -ForegroundColor Green
    Write-Host "Votre application sera disponible dans quelques minutes." -ForegroundColor Cyan
    Write-Host "Surveillez le statut avec: doctl apps list" -ForegroundColor Cyan
} else {
    Write-Host "Erreur lors du deploiement" -ForegroundColor Red
    exit 1
} 