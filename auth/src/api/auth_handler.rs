use axum::Error;
use axum::{ extract::State, Json };
use tracing::{ info, warn };
use crate::model::dto::request::{
    ActiveRequest,
    ForgetPasswordQueryParam,
    Login2faRequest,
    LoginRequest,
    RegisterRequest,
    SetPasswordRequest,
};
use crate::model::dto::response::{ ForgetPasswordResponse, MessageResponse, RegisterResponse, Res };
use crate::server::state::AppState;
use crate::service;
use anyhow::Result;
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>
) -> Result<Json<Res<RegisterResponse>>, Json<Res<String>>> {
    info!("Register a new user request: {req:?}");
    match service::auth_service::register(state, req).await {
        Ok(user_id) => {
            info!("Successfully register user: {user_id}");
            let resp = RegisterResponse { user_id: user_id };
            Ok(Json(Res::with_success(resp)))
        }
        Err(e) => {
            warn!("Unsuccessfully register user: {e:?}");
            Err(Json(Res::with_err(&e.to_string())))
        }
    }
}

// pub async fn active(
//     State(state): State<AppState>,
//     Json(req): Json<ActiveRequest>
// ) -> Result<Json<MessageResponse>> {
//     info!("Active user with token: {req:?}.");
//     todo!()
//     // match service::user::active(&state, req).await {
//     //     Ok(_) => {
//     //         info!("User successfully activated.");
//     //         Ok(Json(MessageResponse::new("User successfully activated.")))
//     //     }
//     //     Err(e) => {
//     //         info!("The user activation operation was not successful: {e:?}");
//     //         Err(e)
//     //     }
//     // }
// }

// pub async fn login(
//     State(state): State<AppState>,
//     Json(req): Json<LoginRequest>
// ) -> Result<Json<LoginResponse>> {
//     info!("Login user with request: {req:?}.");
//     todo!()
//     // match service::user::login(&state, req).await {
//     //     Ok(resp) => {
//     //         info!("Success login user_id: {resp:?}.");
//     //         Ok(Json(resp))
//     //     }
//     //     Err(e) => {
//     //         warn!("Unsuccessfully login user error: {e:?}.");
//     //         Err(e)
//     //     }
//     // }
// }

// pub async fn login2fa(
//     State(state): State<AppState>,
//     Json(req): Json<Login2faRequest>
// ) -> Result<Json<LoginResponse>> {
//     info!("Two factor login user with request: {req:?}.");
//     todo!()
//     // match service::user::login2fa(&state, req).await {
//     //     Ok(resp) => {
//     //         info!("Success login user_id: {resp:?}.");
//     //         Ok(Json(LoginResponse::Token(resp)))
//     //     }
//     //     Err(e) => {
//     //         warn!("Unsuccessfully login user error: {e:?}.");
//     //         Err(e)
//     //     }
//     // }
// }

// pub async fn logout(
//     State(state): State<AppState>,
//     user: UserClaims
// ) -> Result<Json<MessageResponse>> {
//     info!("Logout user_id: {}", user.uid);
//     todo!()
//     // match service::user::logout(&state, user.uid).await {
//     //     Ok(_) => {
//     //         info!("Success logout user user_id: {}", user.uid);
//     //         Ok(Json(MessageResponse::new("This user has successfully logged out.")))
//     //     }
//     //     Err(e) => {
//     //         warn!("unsuccessfully logout user: {e:?}");
//     //         Err(e)
//     //     }
//     // }
// }

// pub async fn forget_password(
//     State(state): State<AppState>,
//     Query(param): Query<ForgetPasswordQueryParam>
// ) -> Result<Json<ForgetPasswordResponse>> {
//     info!("Forget password user query parameter: {param:?}");
//     todo!()
//     // match service::user::forget_password(&state, param).await {
//     //     Ok(resp) => {
//     //         info!("Success forget password user response.");
//     //         Ok(Json(resp))
//     //     }
//     //     Err(e) => {
//     //         warn!("Unsuccessful forget password user: {e:?}.");
//     //         Err(e)
//     //     }
//     // }
// }

// pub async fn reset_password(
//     State(state): State<AppState>,
//     Json(req): Json<SetPasswordRequest>
// ) -> Result<Json<MessageResponse>> {
//     info!("Reset password user: {}.", req.user_id);
//     todo!()
//     // match service::user::reset_password(&state, req).await {
//     //     Ok(_) => {
//     //         info!("Success set new password.");
//     //         Ok(Json(MessageResponse::new("The password has been updated.")))
//     //     }
//     //     Err(e) => {
//     //         warn!("Unsuccessful set password user: {e:?}.");
//     //         Err(e)
//     //     }
//     // }
// }

// pub async fn get_profile(
//     State(state): State<AppState>,
//     user: UserClaims
// ) -> Result<Json<ProfileResponse>> {
//     info!("Get profile user id: {}.", user.uid);
//     todo!()
//     // match service::user::get_profile(&state, user.uid).await {
//     //     Ok(resp) => {
//     //         info!("Success get profile user: {}.", user.uid);
//     //         Ok(Json(resp))
//     //     }
//     //     Err(e) => {
//     //         warn!("Unsuccessfully get profile user: {e:?}.");
//     //         Err(e)
//     //     }
//     // }
// }

// pub async fn update_profile(
//     State(state): State<AppState>,
//     user: UserClaims,
//     Json(req): Json<UpdateProfileRequest>
// ) -> Result<Json<MessageResponse>> {
//     info!("Update profile user_id: {}.", user.uid);
//     todo!()
//     // match service::user::update_profile(&state, user.uid, req).await {
//     //     Ok(_) => {
//     //         info!("Success update profile user user_id: {}.", user.uid);
//     //         Ok(Json(MessageResponse::new("User profile updated.")))
//     //     }
//     //     Err(e) => {
//     //         info!("Unsuccessful update profile user: {e:?}");
//     //         Err(e)
//     //     }
//     // }
// }
