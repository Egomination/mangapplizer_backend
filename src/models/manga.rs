use crate::schema::mangas;
use diesel::PgConnection;

// Return type from the db
#[derive(
    Queryable,
    QueryableByName,
    Identifiable,
    Associations,
    Debug,
    Serialize,
    Deserialize,
    AsChangeset,
)]
#[table_name = "mangas"]
pub struct Manga {
    pub id:                uuid::Uuid,
    pub created_at:        Option<std::time::SystemTime>,
    pub updated_at:        Option<std::time::SystemTime>,
    pub deleted_at:        Option<std::time::SystemTime>,
    pub anilist_id:        i64,
    pub cover_image:       String,
    pub banner_image:      String,
    pub start_date:        String,
    pub end_date:          String,
    pub status:            String,
    pub description:       String,
    pub total_chapters:    Option<String>,
    pub volumes:           Option<String>,
    pub english_title:     String,
    pub romaji_title:      String,
    pub native_title:      String,
    pub cover_extra_large: String,
    pub cover_large:       String,
    pub cover_medium:      String,
    pub popularity:        i64,
}
// Used when new manga is going to be inserted into the database
#[derive(Insertable, Debug, Deserialize, AsChangeset)]
#[table_name = "mangas"]
pub struct NewManga<'a> {
    pub anilist_id:        i64,
    pub cover_image:       &'a str,
    pub banner_image:      &'a str,
    pub start_date:        &'a str,
    pub end_date:          &'a str,
    pub status:            &'a str,
    pub description:       &'a str,
    pub total_chapters:    Option<&'a str>,
    pub volumes:           Option<&'a str>,
    pub english_title:     &'a str,
    pub romaji_title:      &'a str,
    pub native_title:      &'a str,
    pub cover_extra_large: &'a str,
    pub cover_large:       &'a str,
    pub cover_medium:      &'a str,
    pub popularity:        i64,
}

#[derive(Serialize, Deserialize)]
pub struct MangaList(pub Vec<Manga>);

impl MangaList {
    pub fn list(connection: &PgConnection) -> Self {
        use crate::schema::mangas::dsl::*;
        // use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let result = mangas
            .load::<Manga>(connection)
            .expect("Error loading mangas");

        MangaList(result)
    }
}

impl<'a> NewManga<'a> {
    pub fn create(
        &self,
        connection: &PgConnection,
    ) -> Result<Manga, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(mangas::table)
            .values(self)
            .get_result(connection)
    }
}

// TODO: put resp into separate file or into more proper file
#[derive(Serialize)]
pub struct Resp {
    pub manga: Manga,
    pub staff: Vec<crate::models::staff::Staff>,
}

impl Manga {
    pub fn full(
        mid: String,
        connection: &PgConnection,
    ) -> Result<Resp, diesel::result::Error> {
        use crate::diesel::ExpressionMethods;
        use crate::diesel::QueryDsl;
        use crate::diesel::RunQueryDsl;
        use crate::models::{
            series::Series,
            staff::Staff,
        };
        use crate::schema::{
            series,
            staffs,
        };

        let uid = uuid::Uuid::parse_str(&mid).unwrap();
        let manga: Manga = mangas::table
            .filter(mangas::id.eq(uid))
            .get_result(connection)?;
        let series: Result<Vec<Series>, diesel::result::Error> = series::table
            .filter(series::manga_id.eq(uid))
            .get_results(connection);
        let staff_vec: Vec<Staff> = series
            .into_iter()
            .flatten()
            .flat_map(|k| {
                staffs::table
                    .filter(staffs::id.eq(k.staff_id))
                    .get_results(connection)
                    .unwrap()
            })
            .collect::<Vec<Staff>>();

        Ok(Resp {
            manga: manga,
            staff: staff_vec,
        })
    }
}
