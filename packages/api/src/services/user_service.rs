use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManger};

use crate::models::{user, NewUser};
