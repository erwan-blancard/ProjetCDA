use diesel::RunQueryDsl;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

#[actix_web::test]
async fn test_postgres_connection_isolated() {
    // Test PostgreSQL complètement isolé
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL doit être défini");
    
    println!("🔗 Test PostgreSQL isolé - URL: {}", database_url);
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Impossible de créer le pool de connexions PostgreSQL");
    
    let mut conn = pool.get().expect("Impossible d'obtenir une connexion PostgreSQL");
    
    // Test de connexion
    let result = diesel::sql_query("SELECT 1 as test_value").execute(&mut conn);
    
    match result {
        Ok(_) => {
            println!("✅ Connexion PostgreSQL réussie !");
            assert!(true, "La connexion à PostgreSQL fonctionne parfaitement");
        },
        Err(e) => {
            println!("❌ Erreur de connexion PostgreSQL: {:?}", e);
            panic!("La connexion à PostgreSQL a échoué: {:?}", e);
        }
    }
}

#[actix_web::test]
async fn test_postgres_version_isolated() {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL doit être défini");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Impossible de créer le pool de connexions PostgreSQL");
    
    let mut conn = pool.get().expect("Impossible d'obtenir une connexion PostgreSQL");
    
    // Test de la version PostgreSQL
    let result = diesel::sql_query("SELECT version() as version").execute(&mut conn);
    
    match result {
        Ok(_) => {
            println!("✅ Version PostgreSQL récupérée avec succès !");
            assert!(true, "La version PostgreSQL a été récupérée");
        },
        Err(e) => {
            println!("❌ Erreur lors de la récupération de la version: {:?}", e);
            panic!("Impossible de récupérer la version PostgreSQL: {:?}", e);
        }
    }
}

#[actix_web::test]
async fn test_postgres_tables_isolated() {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL doit être défini");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Impossible de créer le pool de connexions PostgreSQL");
    
    let mut conn = pool.get().expect("Impossible d'obtenir une connexion PostgreSQL");
    
    // Test de la liste des tables
    let result = diesel::sql_query(
        "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'"
    ).execute(&mut conn);
    
    match result {
        Ok(_) => {
            println!("✅ Liste des tables récupérée avec succès !");
            assert!(true, "La liste des tables a été récupérée");
        },
        Err(e) => {
            println!("❌ Erreur lors de la récupération des tables: {:?}", e);
            panic!("Impossible de récupérer la liste des tables: {:?}", e);
        }
    }
}

#[actix_web::test]
async fn test_postgres_full_workflow_isolated() {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL doit être défini");
    
    println!("🚀 Test complet PostgreSQL isolé - Démarrage");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Impossible de créer le pool de connexions PostgreSQL");
    
    // Test 1: Connexion simple
    let mut conn = pool.get().expect("Impossible d'obtenir une connexion PostgreSQL");
    println!("✅ Test 1: Connexion établie");
    
    // Test 2: Requête simple
    let result1 = diesel::sql_query("SELECT 1 as test_value").execute(&mut conn);
    assert!(result1.is_ok(), "Requête simple échouée");
    println!("✅ Test 2: Requête simple réussie");
    
    // Test 3: Version PostgreSQL
    let result2 = diesel::sql_query("SELECT version() as version").execute(&mut conn);
    assert!(result2.is_ok(), "Récupération de la version échouée");
    println!("✅ Test 3: Version PostgreSQL récupérée");
    
    // Test 4: Liste des tables
    let result3 = diesel::sql_query(
        "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'"
    ).execute(&mut conn);
    assert!(result3.is_ok(), "Récupération des tables échouée");
    println!("✅ Test 4: Liste des tables récupérée");
    
    // Test 5: Test de transaction
    let result4 = diesel::sql_query("BEGIN").execute(&mut conn);
    assert!(result4.is_ok(), "Début de transaction échoué");
    
    let result5 = diesel::sql_query("COMMIT").execute(&mut conn);
    assert!(result5.is_ok(), "Commit de transaction échoué");
    println!("✅ Test 5: Transaction testée");
    
    println!("🎉 Test complet PostgreSQL isolé - SUCCÈS !");
} 