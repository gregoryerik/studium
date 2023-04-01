use std::sync::Arc;
use actix_web::{web::{Data, self}, FromRequest};


use crate::{business::collections::Group};

const DATABASE_URL: &str = "mysql://7njcpzhr946xitddftib:pscale_pw_FlYPDt79ZkWI7yof9pWQqxR8t31hQT3wPcOqerZM2pi@aws.connect.psdb.cloud/studium";


