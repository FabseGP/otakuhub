use crate::{
    auth::{auth::*, server::RegisterNewUser},
    components::nav_bar::NavBar,
    config::shared::*,
};
use leptos::prelude::*;
use leptos_meta::Title;

#[server(UserExists, "/api", "Url", "user_exists")]
pub async fn user_exists(user: String) -> Result<bool, ServerFnError> {
    use crate::config::types::AppState;
    use sqlx::query;
    let pbox: AppState = use_context().expect("No database pool provided in context");
    let exists = query!("SELECT id FROM users WHERE username = $1", user)
        .fetch_optional(&pbox.pool)
        .await?;
    Ok(exists.is_some())
}

#[must_use]
pub fn password_strength(passwd: &str) -> f32 {
    let mut special: f32 = 0.5;
    let mut num: f32 = 0.5;
    let mut plain: f32 = 0.5;
    for ch in passwd.chars() {
        if "!@#$%^&*()_+-=<>?,./\\[]}{:;\"'".contains(ch) {
            special = 1.2;
        } else if "0123456789".contains(ch) {
            num = 1.1;
        } else {
            plain = 1.3;
        }
    }
    let len = passwd.len() as f32;
    let password_length_minimum_f32 = PASSWORD_LENGTH_MINIMUM as f32;
    let strength = plain
        * num
        * special
        * len
        * len
        * if len < password_length_minimum_f32 {
            len / password_length_minimum_f32
        } else {
            1.0
        }
        / 4.0;
    if strength > 100. { 100. } else { strength }
}

#[component]
pub fn SignupPage() -> impl IntoView {
    use leptos::{either::Either, task::spawn_local};
    use leptos_router::{NavigateOptions, hooks::use_navigate};

    let register: ServerAction<RegisterNewUser> = ServerAction::new();
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let password_confirm = RwSignal::new(String::new());
    let show_pass = RwSignal::new(false);
    let pass_type = move || show_pass.with(|show| if *show { "text" } else { "password" });

    Effect::new(move |_| {
        if register.version().get() > 0 {
            let nav = use_navigate();
            spawn_local(async move {
                if let Ok(Some(_)) = get_user().await {
                    nav("/", NavigateOptions::default());
                }
            });
        }
    });

    let name_taken = Resource::new(
        move || (username.get(), register.version().get()),
        move |(name, _)| async move {
            if name.trim().is_empty() {
                Ok(false)
            } else {
                user_exists(name).await
            }
        },
    );

    let valid_username_ui = move || {
        view! {
            <Transition fallback=|| view! { "..." }>
                {move || Suspend::new(async move {
                    if name_taken.await == Ok(true) {
                        Either::Left(view! { <span class="text-red-500 text-sm">Sorry, that username is taken</span> })
                    } else {
                        let username_len = username.get().len();
                        let message = if username_len > USERNAME_LENGTH_MAXIMUM {
                            (format!("Username must be at most {USERNAME_LENGTH_MAXIMUM} characters"), "text-red-500 text-sm")
                        } else if username_len == 0 {
                            ("Please enter a username".to_string(), "text-red-500 text-sm")
                        } else if username_len < USERNAME_LENGTH_MINIMUM {
                            (format!("Username must be at least {USERNAME_LENGTH_MINIMUM} characters"), "text-red-500 text-sm")
                        } else {
                            ("Username available".to_string(), "text-green-500 text-sm")
                        };
                        Either::Right(view! { <span class={message.1}>{message.0}</span> })
                    }
                })}
            </Transition>
        }
    };

    let pass_strength = move || password.with(move |pw| password_strength(pw));

    let is_valid_form = RwSignal::new(false);

    Effect::new(move |_| {
        let password_value = password.get();
        let password_confirm_value = password_confirm.get();
        let password_valid = password_value == password_confirm_value
            && !password_value.is_empty()
            && (PASSWORD_LENGTH_MINIMUM..=PASSWORD_LENGTH_MAXIMUM).contains(&password_value.len());

        let username_value = username.get();
        let username_valid = (USERNAME_LENGTH_MINIMUM..=USERNAME_LENGTH_MAXIMUM)
            .contains(&username_value.len())
            && matches!(name_taken.get(), Some(Ok(false)));

        is_valid_form.set(password_valid && username_valid);
    });

    view! {
        <Title text="Register"></Title>
        <NavBar />
        <div class="bg-base-200 min-h-screen flex items-center justify-center p-4">
            <div class="card lg:card-side bg-base-100 shadow-xl max-w-4xl w-full">
                <figure class="lg:w-1/2 hidden lg:block">
                    <img
                        src="/images/signup.webp"
                        alt="Signup illustration"
                        class="object-cover w-full h-full"
                    />
                </figure>
                <div class="card-body lg:w-1/2">
                    <div class="space-y-6">
                        <ActionForm action=register on:submit=move |ev| ev.prevent_default()>
                            <h2 class="card-title text-2xl font-bold mb-6 text-center">Create your account</h2>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">Username</span>
                                </label>
                                <label class="input input-bordered flex items-center gap-2">
                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="w-4 h-4 opacity-70">
                                        <path d="M8 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6ZM6.5 9c-1.348 0-2.667.68-3.445 1.767A4.527 4.527 0 0 0 2.5 13.5 1.5 1.5 0 0 0 4 15h8a1.5 1.5 0 0 0 1.5-1.5c0-1.246-.402-2.401-1.055-3.233C11.667 9.68 10.348 9 9 9H6.5Z"/>
                                    </svg>
                                    <input
                                        id="username"
                                        name="username"
                                        type="text"
                                        placeholder="Choose a username"
                                        required
                                        on:input=move |ev| username.set(event_target_value(&ev))
                                        class="grow"
                                    />
                                </label>
                                <label class="label">
                                    {valid_username_ui}
                                </label>
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">Password</span>
                                </label>
                                <label class="input input-bordered flex items-center gap-2">
                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="w-4 h-4 opacity-70">
                                        <path fill-rule="evenodd" d="M14 6a4 4 0 0 1-4.899 3.899l-1.955 1.955a.5.5 0 0 1-.353.146H5v1.5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1-.5-.5v-2.293a.5.5 0 0 1 .146-.353l3.955-3.955A4 4 0 1 1 14 6Zm-4-2a.75.75 0 0 0 0 1.5.5.5 0 0 1 .5.5.75.75 0 0 0 1.5 0 2 2 0 0 0-2-2Z" clip-rule="evenodd" />
                                    </svg>
                                    <input
                                        id="password"
                                        name="password"
                                        type=pass_type
                                        placeholder="Create a password"
                                        required
                                        on:input=move |ev| { password.set(event_target_value(&ev)) }
                                        class="grow"
                                    />
                                    <button
                                        type="button"
                                        class="btn btn-ghost btn-xs"
                                        on:click=move |ev| {
                                            ev.prevent_default();
                                            show_pass.update(|s| *s = !*s);
                                        }
                                    >
                                        {move || if show_pass.get() { "Hide" } else { "Show" }}
                                    </button>
                                </label>
                                <div class="w-full h-1 mt-2 flex items-start">
                                    <div
                                        class="rounded h-1 bg-green-300"
                                        style=move || format!("width: {}%", pass_strength())
                                    ></div>
                                </div>
                                {move || {
                                    let password_value = password.get();
                                    if !password_value.is_empty() {
                                        if password_value.len() < PASSWORD_LENGTH_MINIMUM {
                                            Some(view! {
                                                <label class="label">
                                                    <span class="label-text-alt text-error">{format!("Password must be at least {PASSWORD_LENGTH_MINIMUM} characters")}</span>
                                                </label>
                                            })
                                        } else if password_value.len() > PASSWORD_LENGTH_MAXIMUM {
                                            Some(view! {
                                                <label class="label">
                                                    <span class="label-text-alt text-error">{format!("Passowrd must be at most {PASSWORD_LENGTH_MAXIMUM} characters")}</span>
                                                </label>
                                            })
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }}
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">Confirm password</span>
                                </label>
                                <label class="input input-bordered flex items-center gap-2">
                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="w-4 h-4 opacity-70">
                                        <path fill-rule="evenodd" d="M14 6a4 4 0 0 1-4.899 3.899l-1.955 1.955a.5.5 0 0 1-.353.146H5v1.5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1-.5-.5v-2.293a.5.5 0 0 0 .146-.353l3.955-3.955A4 4 0 1 1 14 6Zm-4-2a.75.75 0 0 0 0 1.5.5.5 0 0 1 .5.5.75.75 0 0 0 1.5 0 2 2 0 0 0-2-2Z" clip-rule="evenodd" />
                                    </svg>
                                    <input
                                        id="password_confirm"
                                        name="password_confirm"
                                        type=pass_type
                                        placeholder="Confirm password"
                                        required
                                        on:input=move |ev| { password_confirm.set(event_target_value(&ev)) }
                                        disabled=show_pass
                                        class="grow"
                                    />
                                </label>
                            </div>

                            <div class="form-control mt-6">
                                <input
                                    type="submit"
                                    value="Create account"
                                    class="btn btn-primary"
                                    disabled={move || !is_valid_form()}
                                />
                            </div>
                        </ActionForm>
                    </div>

                    <div class="divider">OR</div>
                    <div class="text-center">
                        <p>Already have an account?</p>
                        <a href="/login" class="link link-primary">Sign in</a>
                    </div>
                </div>
            </div>
        </div>
    }
}
