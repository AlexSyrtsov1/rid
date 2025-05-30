use std::time;
use std::collections::HashMap;
use std::env;

use actix_web::web::Query;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::cookie::Key;
use actix_session::SessionMiddleware;
use actix_session::storage::SessionStore;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json::{Value, Error};
use sqlx::mysql::MySqlPool;



#[derive(Deserialize)]
pub struct InputData
{
    name: String,
    email: String,
}

#[derive(Serialize, Debug, sqlx::FromRow)]
struct Bestrid
{
    name: Option<String>,
    sub_area: Option<String>
}

#[derive(Serialize, Debug, sqlx::FromRow, Clone)]
struct SubjectAreaFilter
{
    sub_area: Option<String>,
    sub_area_count: i64
}

#[derive(Serialize, Debug, sqlx::FromRow, Clone)]
struct YearFilter
{
    year: Option<String>,
    year_count: i64
}

#[derive(Serialize, Debug, sqlx::FromRow, Clone)]
struct FacultyFilter
{
    faculty: Option<String>,
    faculty_count: i64
}

#[derive(Serialize, Debug, sqlx::FromRow, Clone)]
struct Rid
{
    name: String,
    describtion: String,
    number: i32,
    faculty: String,
    rid_type: String,
    year: String,
    sub_area: String,
    link: String,
    authors: Option<String>
}

pub async fn index(name: web::Path<String>/*, search_request: web::Json<HashMap<String, serde_json::Value>>*/) -> impl Responder
{
    let file_path = format!("C:/Users/syrtsov_ayu/projects/site/server/appearence/{}/{}.html", &name, &name);
    let body = std::fs::read_to_string(file_path).unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

// pub async fn index_find(name: web::Path<String>/*, search_request: web::Json<HashMap<String, serde_json::Value>>*/) -> impl Responder
// {
//     let file_path = format!("C:/Users/syrtsov_ayu/projects/site/server/appearence/{}/{}.html", &name, &name);
//     let body = std::fs::read_to_string(file_path).unwrap();
//     HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(body)
// }

pub async fn main_page() -> impl Responder
{
    let body = std::fs::read_to_string("C:/Users/syrtsov_ayu/projects/site/server/appearence/main/main.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

pub async fn styles(name: web::Path<String>) -> impl Responder
{
    let file_path = format!("C:/Users/syrtsov_ayu/projects/site/server/appearence/{}/content.css", &name);
    let body = std::fs::read_to_string(file_path).unwrap();
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(body)
}

pub async fn scripts(name: web::Path<String>) -> impl Responder
{
    let file_path = format!("C:/Users/syrtsov_ayu/projects/site/server/appearence/{}/{}.js", &name, &name);
    let body = std::fs::read_to_string(file_path).unwrap();
    HttpResponse::Ok()
        .content_type("application/javascript; charset=utf-8")
        .body(body)
}

pub async fn png(file: web::Path<String>) -> impl Responder
{
    let image = format!("C:/Users/syrtsov_ayu/projects/site/server/appearence/images/{}.png", &file);
    let body = std::fs::read(image).unwrap();
    HttpResponse::Ok()
    .content_type("image/png")
    .body(body)
}

pub async fn svg(file: web::Path<String>) -> impl Responder
{
    let image = format!("C:/Users/syrtsov_ayu/projects/site/server/appearence/images/{}.svg", &file);
    let body = std::fs::read(image).unwrap();
    HttpResponse::Ok()
    .content_type("image/svg+xml")
    .body(body)
}

pub async fn fonts(file: web::Path<String>) -> impl Responder
{
    let image = format!("C:/Users/syrtsov_ayu/projects/site/server/appearence/fonts/{}.ttf", &file);
    let body = std::fs::read(image).unwrap();
    HttpResponse::Ok()
    .content_type("font/ttf")
    .body(body)
}

pub async fn best(pool: web::Data<MySqlPool>) -> impl Responder
{
    let rows = sqlx::query_as!(Bestrid, "select bestrid.name as name, SubjectArea.name as sub_area from bestrid left join SubjectArea on (bestrid.idSubjectArea = SubjectArea.id)")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    let mut body: String = String::default();

    for row in rows
    {
        body.push_str(&format!(
            r#"
            <div class="display-frames" style="display: none;">
                <p id="categories-field" style="font-size: 1.3rem; margin: 1rem 0;">
                    {1}
                </p>
                <h3 id="tech-name">
                    <a href="" id="tech-name-link">
                        {0}
                    </a>
                </h3>
            </div>
            "#, row.name.unwrap().to_string(), row.sub_area.unwrap().to_string()
        ))
    }

     HttpResponse::Ok()
    .content_type("text/html, charset=utf-8")
    .body(body)
}

pub async fn counters(pool: web::Data<MySqlPool>) -> impl Responder
{
    let sub_area_count_map = sqlx::query_as!(SubjectAreaFilter, r#"
        select
            subjectarea.name as sub_area,
            COUNT(subjectarea.name) AS sub_area_count
        from rid

        left join SubjectArea on (RID.idSubjectArea = SubjectArea.id)
        GROUP BY
            subjectarea.name
        ORDER by
         	sub_area_count desc
        "#)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    let year_count_map = sqlx::query_as!(YearFilter, r#"
        select
            cast(year.year as char) as year,
            COUNT(year.year) AS year_count
        from rid

        left join Year on (RID.idYear = Year.id)
        GROUP BY
            year.year
        order by
            year_count desc
        "#)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    let faculty_count_map = sqlx::query_as!(FacultyFilter, r#"
        select
            Faculty.name as faculty,
            COUNT(Faculty.name) AS faculty_count
        from rid

        left join Faculty on (RID.idFaculty = Faculty.id)
        GROUP BY
            Faculty.name
        order by
            faculty_count desc
        "#)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    let mut body: String = String::from(r#"
        <div class="category-type">
            <button type="button" class="category-header" onclick="hideCategoryList('category-1')">
                Предметная область<img style="block-size: 1rem; transform: rotate(0);" id="category-1" src="https://lib.obs.ru-moscow-1.hc.sbercloud.ru:443/PATENTS/expand.svg">
            </button>

            <div id="category-list-1" class="category-list">
    "#);

    for (i, item) in sub_area_count_map.iter().enumerate()
    {
        body.push_str(&format!(
            r#"
                <div class="category">
                        <div>
                            <input type="checkbox" id="sub-{0}" name="check" onclick="addQuery('s','{1}')"/>
                            <label for="sub-{0}">{1}</label>
                        </div>
                    <div class="category-count">{2}</div>
                </div>
            "#,
            i,
            item.clone().sub_area.unwrap().to_string(),
            item.sub_area_count
        ));
    }

    body.push_str(r#"
            </div>
        </div>

        <div class="category-type">
            <button type="button" class="category-header" onclick="hideCategoryList('category-2')">
                Год<img style="block-size: 1rem; transform: rotate(180deg);" id="category-2" src="https://lib.obs.ru-moscow-1.hc.sbercloud.ru:443/PATENTS/expand.svg">
            </button>

            <div id="category-list-2" class="category-list" style="display:none;">
    "#);

    for (i, item) in year_count_map.iter().enumerate()
    {
        body.push_str(&format!(
            r#"
                <div class="category">
                        <div>
                            <input type="checkbox" id="year-{0}" name="check" onclick="addQuery('y','{1}')"/>
                            <label for="year-{0}">{1}</label>
                        </div>
                    <div class="category-count">{2}</div>
                </div>
            "#,
            i,
            item.clone().year.unwrap().to_string(),
            item.year_count
        ));
    }

    body.push_str(r#"
            </div>
        </div>

        <div class="category-type" style="margin-bottom: 10rem;">
            <button type="button" class="category-header" aria-expanded="true" onclick="hideCategoryList('category-3')">
                Факультеты<img style="block-size: 1rem; transform: rotate(180deg);" id="category-3" src="https://lib.obs.ru-moscow-1.hc.sbercloud.ru:443/PATENTS/expand.svg">
            </button>

            <div id="category-list-3" class="category-list" style="display: none;">
    "#);

    for (i, item) in faculty_count_map.iter().enumerate()
    {
        body.push_str(&format!(
            r#"
                <div class="category">
                        <div>
                            <input type="checkbox" id="dep-{0}" name="check" onclick="addQuery('d','{1}')"/>
                            <label for="dep-{0}">{1}</label>
                        </div>
                    <div class="category-count">{2}</div>
                </div>
            "#,
            i,
            item.clone().faculty.unwrap().to_string(),
            item.faculty_count
        ));
    }

    body.push_str(r#"
            </div>
        </div>
    "#);
    
    HttpResponse::Ok()
        .content_type("text/html, charset=utf-8")
        .body(body)
}

pub async fn find(search_request: web::Json<HashMap<String, serde_json::Value>>, pool: web::Data<MySqlPool>) -> impl Responder
{
    let mut basepart: String = String::from(r#"
        select
            rid.name as name,
            rid.description as describtion,
            rid.numPotent as number,
            Faculty.name as faculty,
            Type.name as rid_type,
            cast(Year.year as char) as year,
            SubjectArea.name as sub_area,
            rid.link as link,
            group_concat(fio.surname, ' ', fio.name, '.', fio.lastname, '.', ' (', authorcountry.name, ')' SEPARATOR ', ') as authors
        from rid

        left join Faculty on (RID.idFaculty = Faculty.id)
        left join Type on (RID.idType = Type.id)
        left join Year on (RID.idYear = Year.id)
        left join SubjectArea on (RID.idSubjectArea = SubjectArea.id)
        left join AUTHORxRID on (RID.id = AUTHORxRID.idRID)
        left join fio on (AUTHORxRID.idAuthor = FIO.id)
        left join ConnectionAuthorCountry on (FIO.id = ConnectionAuthorCountry.idFIO)
        left join AuthorCountry on (ConnectionAuthorCountry.idCountry = AuthorCountry.id)
    "#);

    let mut strpart: String = String::default();
    let mut ypart: String = String::default();
    let mut spart: String = String::default();
    let mut dpart: String = String::default();

    for (key, val) in search_request.into_inner()
    {
        match key.as_str() {
            "str" => 
            {
                strpart.push_str(&format!(" (lower(RID.name) like lower('%{0}%')) or (lower(RID.description) like lower('%{0}%'))", val.as_str().unwrap().to_string()));
                
            },

            "y" =>
            {
                ypart.push_str(&format!(" Year.year = {}", val.as_array().unwrap()[0].to_string()));

                if val.as_array().unwrap().len() == 1
                {
                    continue;
                }

                for year in &val.as_array().unwrap()[1..]
                {
                    ypart.push_str(&format!(" or Year.year = {}", year.to_string()));
                }
            },

            "s" =>
            {
                spart.push_str(&format!(" SubjectArea.name = {}", val.as_array().unwrap()[0].to_string()));

                if val.as_array().unwrap().len() == 1
                {
                    continue;
                }

                for sub_area in &val.as_array().unwrap()[1..]
                {
                    spart.push_str(&format!(" or SubjectArea.name = {}", sub_area.to_string()));
                }
            },

            "d" =>
            {
                dpart.push_str(&format!(" Faculty.name = {}", val.as_array().unwrap()[0].to_string()));

                if val.as_array().unwrap().len() == 1
                {
                    continue;
                }

                for faculty in &val.as_array().unwrap()[1..]
                {
                    dpart.push_str(&format!(" or Faculty.name = {}", faculty.to_string()));
                }
            },

            _ => continue
        }
    }

    let mut is_first: bool = true;

    if !dpart.trim().is_empty()
    {
        is_first = if is_first {false} else {
            strpart.push_str(" and");
            false
        };
        strpart.push_str(&dpart);
    }

    if !ypart.trim().is_empty()
    {
        is_first = if is_first {false} else {
            strpart.push_str(" and");
            false
        };
        strpart.push_str(&ypart);
    }

    if !spart.trim().is_empty()
    {
        is_first = if is_first {false} else {
            strpart.push_str(" and");
            false
        };
        strpart.push_str(&spart);
    }

    if !strpart.trim().is_empty()
    {
        basepart.push_str(" where");
        basepart.push_str(&strpart);
    }

    basepart.push_str(" GROUP BY rid.name, year");

    let rows:Result<Vec<Rid>, sqlx::Error>  = sqlx::query_as(&basepart)
        .fetch_all(pool.get_ref())
        .await;

    match rows
    {
        Ok(rows) => {
            if rows.is_empty()
            {
                return HttpResponse::Ok()
                    .content_type("text/html, charset=utf-8")
                    .body("<h2> По запросу нет подходящих результатов </h2>");
            }

            let mut body: String = String::default();

            for (i, row) in rows.iter().enumerate()
            {
                body.push_str(&format!(
                    r#"
                        <div class="card-container">
                            <div class="card-content">
                                <h1 style="font-size: 1.1rem; margin: 0 0 .5rem 0;">
                                    {3}
                                </h1>
                                <h2 id="tech-name" style="font-size: 1.3rem; margin: 0 0 1rem 0;">
                                    <a class="tech-name-title" href="{8}">
                                        {1}
                                    </a>
                                </h2>
                                <p style="font-size: 1.1rem; margin: 0 0 .3rem 0;">
                                    {9}
                                </p>
                                <p style="font-size: .9rem; margin: 0 0 1rem 0; color: #5f5f5f;">
                                    {6}
                                </p>
                                <div class="category-tags">
                                    <a href="" class="category-tag">{5}</a>
                                    <a href="" class="category-tag">{7}</a>
                                    <a href="" class="category-tag">{4}</a>
                                </div>
                                
                                <button type="button" class="description-toggle" aria-expanded="true" onclick="showDescribtion('patent-{0}')" style="margin-top: 1rem;">
                                    Описание<img style="block-size: 1rem; transform: rotate(180deg);" id="patent-{0}" src="https://lib.obs.ru-moscow-1.hc.sbercloud.ru:443/PATENTS/expand.svg">
                                </button>

                                <p id="patent-{0}-desc" class="category-list" style="display: none;">
                                    {2}
                                </p>
                            </div>
                        </div>
                    "#,
                    i,
                    row.name, // 1
                    row.describtion, // 2
                    row.number, // 3
                    row.faculty, // 4
                    row.rid_type, // 5
                    row.year, // 6
                    row.sub_area, // 7
                    row.link, // 8
                    row.clone().authors.unwrap_or("Авторы не указаны".to_string()).to_string() // 9
                ));
            }

            return HttpResponse::Ok()
                .content_type("text/html, charset=utf-8")
                .body(body);
        },
        Err(error) => {
            return HttpResponse::ServiceUnavailable()
                .content_type("text/html, charset=utf-8")
                .body(error.to_string());
        }
    }


     // q.into_inner()["y"].as_array().unwrap()[0].to_string()
}

pub async fn update(name: web::Path<String>) -> impl Responder
{
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Actix Web Example</title>
            
        </head>
        <body>
            <h1 id="myElement">{}</h1>
            
        </body>
        </html>
    "#, env::current_dir().unwrap().into_os_string().into_string().unwrap()))
}

pub async fn poster(name: web::Path<String>, params: web::Json<InputData>) -> impl Responder
{
    let res = format!(
        r#"<div id="shit">our input is name: {}, email: {}, and I have got it in {:?}</div>
        <img src="images/i.png"/>"#,
        params.name,
        params.email,
        time::Instant::now()
    );

    HttpResponse::Ok()
        .content_type("text/html") // Correcting "text/palin" to "text/plain"
        .body(res)
}

pub async fn not_found() -> impl Responder
{
    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body("<h1>Error 404</h1>")
}