use chrono::Utc;
use sqlx::PgPool;

use crate::{
    dto::{LoginInput, RegisterInput},
    error::{Error, Result},
    model::{CreateUserData, UpdateUserData, User},
    utils::encryption,
};

pub struct AuthService;

impl AuthService {
    /*pub async fn sign_in(input: LoginInput, pool: &PgPool) -> Result<User> {
        let user = User::find_by_email(&input.email, &pool).await?;
        if encryption::verify_password(input.password, user.password.to_owned()).await? {
            Ok(user)
        } else {
            Err(Error::WrongPassword)
        }
    }

    pub async fn sign_up(input: RegisterInput, pool: &PgPool) -> Result<User> {
        if User::find_by_email(&input.email, &pool).await.is_ok() {
            return Err(Error::DuplicateUserEmail);
        }
        if User::find_by_name(&input.name, &pool).await.is_ok() {
            return Err(Error::DuplicateUserName);
        }

        let data = CreateUserData {
            name: input.name,
            email: input.email,
            password: encryption::hash_password(input.password).await?,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        Ok(User::create(data, &pool).await?)
    }*/
}

pub struct UserService;

impl UserService {
    pub async fn create(pool: &PgPool, data: CreateUserData) -> Result<User> {
        Ok(User::create(data, &pool).await?)
    }

    pub async fn find_by_id(pool: &PgPool, id: uuid::Uuid) -> Result<User> {
        Ok(User::find_by_id(id, &pool).await?)
    }

    pub async fn update(pool: &PgPool, id: uuid::Uuid, data: UpdateUserData) -> Result<User> {
        Ok(User::update(id, data, &pool).await?)
    }
}
