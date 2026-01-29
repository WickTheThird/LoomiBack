use askama::Template;
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};
use tower_cookies::{Cookie, Cookies};

use crate::app::AppState;
use crate::auth::model::AccountStatus;
use super::ui::{
    AUTH_COOKIE_NAME, LoginTemplate, DashboardTemplate, UsersTemplate,
    LoginForm, PaginationQuery,
};

pub async fn login_page(cookies: Cookies) -> impl IntoResponse {
    if cookies.get(AUTH_COOKIE_NAME).is_some() {
        return Redirect::to("/admin/dashboard").into_response();
    }
    Html(LoginTemplate { error: None }.render().unwrap_or_default()).into_response()
}

pub async fn login_submit(
    State(state): State<AppState>,
    cookies: Cookies,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    let user = match state.storage.get_user_by_email(&form.email).await {
        Ok(Some(user)) => user,
        _ => {
            return Html(
                LoginTemplate { error: Some("Invalid email or password".to_string()) }
                    .render()
                    .unwrap_or_default(),
            )
            .into_response();
        }
    };

    let password_valid = bcrypt::verify(&form.password, &user.password_hash).unwrap_or(false);
    if !password_valid {
        return Html(
            LoginTemplate { error: Some("Invalid email or password".to_string()) }
                .render()
                .unwrap_or_default(),
        )
        .into_response();
    }

    let admin = match state.storage.get_admin_by_user_id(user.id).await {
        Ok(Some(admin)) => admin,
        _ => {
            return Html(
                LoginTemplate { error: Some("You are not authorized to access the admin panel".to_string()) }
                    .render()
                    .unwrap_or_default(),
            )
            .into_response();
        }
    };

    let account = match state.storage.get_account_by_user_id(user.id).await {
        Ok(Some(account)) => account,
        _ => {
            return Html(
                LoginTemplate { error: Some("Account not found".to_string()) }
                    .render()
                    .unwrap_or_default(),
            )
            .into_response();
        }
    };

    if account.account_status != AccountStatus::Active {
        return Html(
            LoginTemplate { error: Some("Your account is not active".to_string()) }
                .render()
                .unwrap_or_default(),
        )
        .into_response();
    }

    let token_pair = match state.token_service.generate_admin_tokens(&user, &account, &admin) {
        Ok(tokens) => tokens,
        Err(_) => {
            return Html(
                LoginTemplate { error: Some("Failed to generate authentication token".to_string()) }
                    .render()
                    .unwrap_or_default(),
            )
            .into_response();
        }
    };

    let mut cookie = Cookie::new(AUTH_COOKIE_NAME, token_pair.access_token);
    cookie.set_path("/admin");
    cookie.set_http_only(true);
    cookie.set_secure(false);
    cookies.add(cookie);

    let _ = state.storage.update_user_last_login(user.id).await;

    Redirect::to("/admin/dashboard").into_response()
}

pub async fn logout(
    State(state): State<AppState>,
    cookies: Cookies,
) -> impl IntoResponse {
    if let Some(cookie) = cookies.get(AUTH_COOKIE_NAME) {
        if let Ok(claims) = state.token_service.verify_access_token(cookie.value()) {
            state.validation.blacklist_jti(claims.jti);
        }
    }

    let mut cookie = Cookie::new(AUTH_COOKIE_NAME, "");
    cookie.set_path("/admin");
    cookies.remove(cookie);

    Redirect::to("/admin/login").into_response()
}

pub async fn dashboard(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Response {
    let claims = match verify_admin_cookie(&state, &cookies).await {
        Some(claims) => claims,
        None => return Redirect::to("/admin/login").into_response(),
    };

    let system_healthy = state.storage.health_check().await;

    let template = DashboardTemplate {
        user_email: claims.email,
        admin_role: claims.admin_role.map(|r| format!("{:?}", r)).unwrap_or_else(|| "Admin".to_string()),
        account_level: format!("{:?}", claims.account_level),
        total_users: 0,
        active_users: 0,
        total_admins: 1,
        system_healthy,
    };

    Html(template.render().unwrap_or_default()).into_response()
}

pub async fn users_list(
    State(state): State<AppState>,
    cookies: Cookies,
    Query(query): Query<PaginationQuery>,
) -> Response {
    let claims = match verify_admin_cookie(&state, &cookies).await {
        Some(claims) => claims,
        None => return Redirect::to("/admin/login").into_response(),
    };

    let _page = query.page.unwrap_or(1);

    let template = UsersTemplate {
        user_email: claims.email,
        users: vec![],
        message: None,
        current_page: 1,
        total_pages: 1,
    };

    Html(template.render().unwrap_or_default()).into_response()
}

async fn verify_admin_cookie(
    state: &AppState,
    cookies: &Cookies,
) -> Option<crate::auth::model::Claims> {
    let cookie = cookies.get(AUTH_COOKIE_NAME)?;
    let token = cookie.value();
    let claims = state.token_service.verify_access_token(token).ok()?;

    if state.validation.is_jti_blacklisted(&claims.jti) {
        return None;
    }

    if !claims.is_admin {
        return None;
    }

    Some(claims)
}
