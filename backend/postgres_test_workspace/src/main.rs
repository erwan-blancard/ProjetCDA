use diesel::RunQueryDsl;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

#[actix_rt::main]
async fn main() {
    println!("🚀 Test PostgreSQL - Démarrage");
    
    // Test de connexion à PostgreSQL
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL doit être défini");
    
    println!("🔗 Connexion à PostgreSQL avec l'URL: {}", database_url);
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Impossible de créer le pool de connexions PostgreSQL");
    
    let mut conn = pool.get().expect("Impossible d'obtenir une connexion PostgreSQL");
    
    // Test 1: Connexion simple
    println!("✅ Test 1: Connexion établie");
    
    // Test 2: Requête simple
    let result1 = diesel::sql_query("SELECT 1 as test_value").execute(&mut conn);
    match result1 {
        Ok(_) => println!("✅ Test 2: Requête simple réussie"),
        Err(e) => {
            println!("❌ Test 2: Requête simple échouée: {:?}", e);
            std::process::exit(1);
        }
    }
    
    // Test 3: Version PostgreSQL
    let result2 = diesel::sql_query("SELECT version() as version").execute(&mut conn);
    match result2 {
        Ok(_) => println!("✅ Test 3: Version PostgreSQL récupérée"),
        Err(e) => {
            println!("❌ Test 3: Récupération de la version échouée: {:?}", e);
            std::process::exit(1);
        }
    }
    
    // Test 4: Liste des tables
    let result3 = diesel::sql_query(
        "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'"
    ).execute(&mut conn);
    match result3 {
        Ok(_) => println!("✅ Test 4: Liste des tables récupérée"),
        Err(e) => {
            println!("❌ Test 4: Récupération des tables échouée: {:?}", e);
            std::process::exit(1);
        }
    }
    
    // Test 5: Test de transaction
    let result4 = diesel::sql_query("BEGIN").execute(&mut conn);
    match result4 {
        Ok(_) => println!("✅ Test 5: Début de transaction réussi"),
        Err(e) => {
            println!("❌ Test 5: Début de transaction échoué: {:?}", e);
            std::process::exit(1);
        }
    }
    
    let result5 = diesel::sql_query("COMMIT").execute(&mut conn);
    match result5 {
        Ok(_) => println!("✅ Test 5: Commit de transaction réussi"),
        Err(e) => {
            println!("❌ Test 5: Commit de transaction échoué: {:?}", e);
            std::process::exit(1);
        }
    }
    
    println!("🎉 Test PostgreSQL - SUCCÈS COMPLET !");
    println!("✅ PostgreSQL fonctionne parfaitement avec Rust et Diesel !");
} 