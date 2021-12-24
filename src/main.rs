use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Transaction};
use uuid::Uuid;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "my_enum", rename_all = "snake_case")]
enum MyEnum {
    State1,
    State2,
    State3,
}

// This wackiness is needed to work around an issue with a plain Vec<EnumType>
// - see https://github.com/launchbadge/sqlx/pull/1170#issuecomment-817738085.
#[derive(Debug, sqlx::Decode, sqlx::Encode)]
struct MyEnums(Vec<MyEnum>);

impl sqlx::Type<sqlx::Postgres> for MyEnums {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_my_enum")
    }
}

// You might think that if you just derive sqlx::Type, it'll work without
// runtime hacks. Unfortunately that isn't true.
#[derive(Debug, sqlx::Decode)]
struct MyText(String);

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
struct Table1 {
    table1_id: Uuid,
    text: String,
    text_null: Option<String>,
    citext: String,
    citext_null: Option<String>,
    mytext: MyText,
    mytext_null: Option<MyText>,
    myenum: MyEnum,
    myenum_null: Option<MyEnum>,
}

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
struct MyEnumArray {
    myenums: MyEnums,
}

#[tokio::main]
async fn main() -> Result<()> {
    let db_uri =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set to run experiments");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_uri)
        .await
        .expect("Could not connect to postgres");
    insert_text_col(&pool).await?;
    insert_text_null_col(&pool).await?;
    insert_citext_col(&pool).await?;
    insert_citext_null_col(&pool).await?;
    insert_citext_col_with_non_string(&pool).await?;
    insert_mytext_col(&pool).await?;
    insert_mytext_null_col(&pool).await?;
    select_myenum_array(&pool).await?;

    let mut tx = pool.begin().await?;
    insert_in_tx1(&mut tx).await?;
    tx.commit().await?;

    Ok(())
}

async fn insert_text_col(pool: &Pool<Postgres>) -> Result<()> {
    let row = sqlx::query_as!(
        Table1,
        r#"
            INSERT INTO table1 (text) VALUES ($1)
            RETURNING table1_id,
                      text,
                      text_null,
                      citext AS "citext!: String",
                      citext_null AS "citext_null: String",
                      mytext AS "mytext!: MyText",
                      mytext_null AS "mytext_null: MyText",
                      myenum AS "myenum!: _",
                      myenum_null AS "myenum_null: _"
        "#,
        "insert_text_col",
    )
    .fetch_one(pool)
    .await?;
    println!("Row = {:?}", row);
    Ok(())
}

async fn insert_text_null_col(pool: &Pool<Postgres>) -> Result<()> {
    let row = sqlx::query_as!(
        Table1,
        r#"
            INSERT INTO table1 (text_null) VALUES ($1)
            RETURNING table1_id,
                      text,
                      text_null,
                      citext AS "citext!: String",
                      citext_null AS "citext_null: String",
                      mytext AS "mytext!: MyText",
                      mytext_null AS "mytext_null: MyText",
                      myenum AS "myenum!: _",
                      myenum_null AS "myenum_null: _"
        "#,
        "insert_text_null_col",
    )
    .fetch_one(pool)
    .await?;
    println!("Row = {:?}", row);
    Ok(())
}

async fn insert_citext_col(pool: &Pool<Postgres>) -> Result<()> {
    let row = sqlx::query_as!(
        Table1,
        r#"
            INSERT INTO table1 (citext) VALUES ($1)
            RETURNING table1_id,
                      text,
                      text_null,
                      citext AS "citext!: String",
                      citext_null AS "citext_null: String",
                      mytext AS "mytext!: MyText",
                      mytext_null AS "mytext_null: MyText",
                      myenum AS "myenum!: _",
                      myenum_null AS "myenum_null: _"
        "#,
        // The "as _" part effectively te
        "insert_citext" as _,
    )
    .fetch_one(pool)
    .await?;
    println!("Row = {:?}", row);
    Ok(())
}

async fn insert_citext_null_col(pool: &Pool<Postgres>) -> Result<()> {
    let row = sqlx::query_as!(
        Table1,
        r#"
            INSERT INTO table1 (citext_null) VALUES ($1::TEXT::citext)
            RETURNING table1_id,
                      text,
                      text_null,
                      citext AS "citext!: String",
                      citext_null AS "citext_null: String",
                      mytext AS "mytext!: MyText",
                      mytext_null AS "mytext_null: MyText",
                      myenum AS "myenum!: _",
                      myenum_null AS "myenum_null: _"
        "#,
        "insert_citext_null_col",
    )
    .fetch_one(pool)
    .await?;
    println!("Row = {:?}", row);
    Ok(())
}

async fn insert_citext_col_with_non_string(pool: &Pool<Postgres>) -> Result<()> {
    let row = sqlx::query_as!(
        Table1,
        r#"
            INSERT INTO table1 (citext, citext_null) VALUES ($1, $2)
            RETURNING table1_id,
                      text,
                      text_null,
                      citext AS "citext!: String",
                      citext_null AS "citext_null: String",
                      mytext AS "mytext!: MyText",
                      mytext_null AS "mytext_null: MyText",
                      myenum AS "myenum!: _",
                      myenum_null AS "myenum_null: _"
        "#,
        42 as _,
        vec!["hello", "world"] as _,
    )
    .fetch_one(pool)
    .await?;
    println!("Row = {:?}", row);
    Ok(())
}

async fn insert_mytext_col(pool: &Pool<Postgres>) -> Result<()> {
    let row = sqlx::query_as!(
        Table1,
        r#"
            INSERT INTO table1 (mytext) VALUES ($1::TEXT::mytext)
            RETURNING table1_id,
                      text,
                      text_null,
                      citext AS "citext!: String",
                      citext_null AS "citext_null: String",
                      mytext AS "mytext!: MyText",
                      mytext_null AS "mytext_null: MyText",
                      myenum AS "myenum!: _",
                      myenum_null AS "myenum_null: _"
        "#,
        "insert_mytext_col",
    )
    .fetch_one(pool)
    .await?;
    println!("Row = {:?}", row);
    Ok(())
}

async fn insert_mytext_null_col(pool: &Pool<Postgres>) -> Result<()> {
    let row = sqlx::query_as!(
        Table1,
        r#"
            INSERT INTO table1 (mytext_null) VALUES ($1::TEXT::mytext)
            RETURNING table1_id,
                      text,
                      text_null,
                      citext AS "citext!: String",
                      citext_null AS "citext_null: String",
                      mytext AS "mytext!: MyText",
                      mytext_null AS "mytext_null: MyText",
                      myenum AS "myenum!: _",
                      myenum_null AS "myenum_null: _"
        "#,
        "insert_mytext_null_col",
    )
    .fetch_one(pool)
    .await?;
    println!("Row = {:?}", row);
    Ok(())
}

async fn select_myenum_array(pool: &Pool<Postgres>) -> Result<()> {
    let row = sqlx::query_as!(
        MyEnumArray,
        r#"
            SELECT ARRAY(
                SELECT myenum
                  FROM table1
            ) AS "myenums!: _"
        "#
    )
    .fetch_one(pool)
    .await?;
    println!("Row = {:?}", row);
    Ok(())
}

async fn insert_in_tx1(tx: &mut Transaction<'_, Postgres>) -> Result<()> {
    sqlx::query!("INSERT INTO table1 (text) VALUES ($1)", "insert_in_tx",)
        // If we just use execute(tx) then the call to insert_in_tx2(tx) will
        // cause the compiler to emit a "value borrowed here after move"
        // error.
        .execute(&mut *tx)
        .await?;
    insert_in_tx2(tx).await?;
    Ok(())
}

async fn insert_in_tx2(tx: &mut Transaction<'_, Postgres>) -> Result<()> {
    sqlx::query!("INSERT INTO table1 (text) VALUES ($1)", "insert_in_tx",)
        .execute(tx)
        .await?;
    Ok(())
}
