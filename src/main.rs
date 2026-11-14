use axum::{extract::{Path, State}, http::StatusCode, response::{Html, IntoResponse}, routing::{delete, get, post, put}, Json, Router};
use chrono::NaiveDate;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::{Arc, Mutex}};
use tower_http::trace::TraceLayer;
use uuid::Uuid;

#[derive(Clone)] struct AppState { db: Arc<Mutex<Connection>> }
#[derive(Serialize, Deserialize, Clone)] struct Profile { name:String, occupation:String, company:String, year:u8, start_date:Option<String> }
#[derive(Serialize, Deserialize, Clone)] struct Entry { id:String, date:String, activity:String, hours:f64, category:String, notes:String }
#[derive(Serialize)] struct ApiError { error:String }
impl IntoResponse for ApiError { fn into_response(self)->axum::response::Response { (StatusCode::BAD_REQUEST, Json(self)).into_response() } }
fn init_db()->Connection {
 let db=Connection::open("azubilog.db").expect("Datenbank konnte nicht geöffnet werden");
 db.execute_batch("PRAGMA journal_mode=WAL;
 CREATE TABLE IF NOT EXISTS profile (id INTEGER PRIMARY KEY CHECK(id=1), data TEXT NOT NULL);
 CREATE TABLE IF NOT EXISTS entries (id TEXT PRIMARY KEY, date TEXT NOT NULL, activity TEXT NOT NULL, hours REAL NOT NULL, category TEXT NOT NULL, notes TEXT NOT NULL);
 CREATE INDEX IF NOT EXISTS idx_entries_date ON entries(date);").expect("Datenbank konnte nicht initialisiert werden"); db
}
async fn index()->Html<&'static str>{Html(include_str!("../static/index.html"))}
async fn get_profile(State(s):State<AppState>)->Result<Json<Option<Profile>>,ApiError>{
 let db=s.db.lock().unwrap(); let raw:Option<String>=db.query_row("SELECT data FROM profile WHERE id=1",[],|r|r.get(0)).optional().map_err(err)?;
 Ok(Json(raw.and_then(|v|serde_json::from_str(&v).ok())))
}
async fn save_profile(State(s):State<AppState>,Json(p):Json<Profile>)->Result<StatusCode,ApiError>{
 if p.name.trim().is_empty()||p.occupation.trim().is_empty()||p.company.trim().is_empty()||!(1..=4).contains(&p.year){return Err(ApiError{error:"Bitte Profil vollständig ausfüllen.".into()})}
 let raw=serde_json::to_string(&p).map_err(err)?; let db=s.db.lock().unwrap(); db.execute("INSERT INTO profile(id,data) VALUES(1,?1) ON CONFLICT(id) DO UPDATE SET data=excluded.data",params![raw]).map_err(err)?; Ok(StatusCode::NO_CONTENT)
}
async fn list_entries(State(s):State<AppState>)->Result<Json<Vec<Entry>>,ApiError>{
 let db=s.db.lock().unwrap(); let mut st=db.prepare("SELECT id,date,activity,hours,category,notes FROM entries ORDER BY date DESC").map_err(err)?;
 let rows=st.query_map([],|r|Ok(Entry{id:r.get(0)?,date:r.get(1)?,activity:r.get(2)?,hours:r.get(3)?,category:r.get(4)?,notes:r.get(5)?})).map_err(err)?;
 let mut out=Vec::new();for row in rows{out.push(row.map_err(err)?)}Ok(Json(out))
}
async fn create_entry(State(s):State<AppState>,Json(mut e):Json<Entry>)->Result<Json<Entry>,ApiError>{
 validate(&e)?;e.id=Uuid::new_v4().to_string();let db=s.db.lock().unwrap();db.execute("INSERT INTO entries(id,date,activity,hours,category,notes) VALUES(?1,?2,?3,?4,?5,?6)",params![e.id,e.date,e.activity,e.hours,e.category,e.notes]).map_err(err)?;Ok(Json(e))
}
async fn update_entry(Path(id):Path<String>,State(s):State<AppState>,Json(e):Json<Entry>)->Result<StatusCode,ApiError>{
 validate(&e)?;let db=s.db.lock().unwrap();let n=db.execute("UPDATE entries SET date=?2,activity=?3,hours=?4,category=?5,notes=?6 WHERE id=?1",params![id,e.date,e.activity,e.hours,e.category,e.notes]).map_err(err)?;if n==0{return Err(ApiError{error:"Eintrag nicht gefunden.".into()})}Ok(StatusCode::NO_CONTENT)
}
async fn delete_entry(Path(id):Path<String>,State(s):State<AppState>)->Result<StatusCode,ApiError>{let db=s.db.lock().unwrap();db.execute("DELETE FROM entries WHERE id=?1",params![id]).map_err(err)?;Ok(StatusCode::NO_CONTENT)}
fn validate(e:&Entry)->Result<(),ApiError>{let d=NaiveDate::parse_from_str(&e.date,"%Y-%m-%d").map_err(|_|ApiError{error:"Ungültiges Datum.".into()})?;let _=d;if e.activity.trim().is_empty()||!e.hours.is_finite()||e.hours<0.0||e.hours>24.0{return Err(ApiError{error:"Bitte Tätigkeit und gültige Stunden angeben.".into()})}Ok(())}
fn err<E:std::fmt::Display>(e:E)->ApiError{ApiError{error:e.to_string()}}
#[tokio::main] async fn main(){
 let state=AppState{db:Arc::new(Mutex::new(init_db()))};
 let app=Router::new().route("/",get(index)).route("/api/profile",get(get_profile).post(save_profile))
 .route("/api/entries",get(list_entries).post(create_entry)).route("/api/entries/:id",put(update_entry).delete(delete_entry))
 .with_state(state).layer(TraceLayer::new_for_http());
 let addr:SocketAddr="127.0.0.1:3000".parse().unwrap();
 println!("AzubiLog läuft unter http://{addr}");
 let listener=tokio::net::TcpListener::bind(addr).await.unwrap();axum::serve(listener,app).await.unwrap();
}
