/* 
 * ====================
 * ====================
 * =====          =====
 * =====  ROUTES  =====
 * =====          =====
 * ====================
 * ====================
 * 
 * 
 * 
 * Functions to be called when user request hits endpoints listed
 * in the main function.
 * 
 * 
*/
// why it gray?
use actix_web::{
    web, HttpResponse, HttpRequest,
    Responder, http::StatusCode, http::header,
    get, post, web::Redirect };
use actix_web::cookie::{ Cookie };
use askama::Template;
use serde::{ Deserialize, Serialize };


