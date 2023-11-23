use actix_web::{delete, web};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use web::Json;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course{
    pub teacher_id: usize,
    pub id: Option<usize>,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}

impl From<Json<Course>> for Course{
    fn from(course: web::Json<Course>) -> Self{
        Course{
            teacher_id: course.teacher_id,
            id: course.id,
            name: course.name.clone(),
            time: course.time,
        }
    }
}