use crate::{
    auth::{auth::get_user, server::LoginUser},
    components::nav_bar::NavBar,
    config::shared::*,
};
use leptos::prelude::*;
use leptos_router::{NavigateOptions, hooks::use_navigate};

#[component]
pub fn LoginPage() -> impl IntoView {
    let login: ServerAction<LoginUser> = ServerAction::new();
    let show_pass = RwSignal::new(false);
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let login_error = RwSignal::new(Option::<String>::None);

    let pass_type = Signal::derive(move || if show_pass.get() { "text" } else { "password" });

    let is_valid_form = Signal::derive(move || {
        let username_value = username.get();
        let valid_username = !username_value.is_empty()
            && (USERNAME_LENGTH_MINIMUM..=USERNAME_LENGTH_MAXIMUM).contains(&username_value.len());
        let password_value = password.get();
        let valid_password = !password_value.is_empty()
            && (PASSWORD_LENGTH_MINIMUM..=PASSWORD_LENGTH_MAXIMUM).contains(&password_value.len());

        valid_username && valid_password
    });

    let logged_in_user = Resource::new(
        move || login.version().get(),
        move |_user| async move {
            if let Ok(Some(_)) = get_user().await {
                login_error.set(None);
                true
            } else {
                false
            }
        },
    );

    Effect::new(move |_| {
        login.pending().track();
        if login.pending().get() {
            login_error.set(None);
        } else {
            let value = login.value().get();
            match value {
                Some(Ok(None)) => {
                    login_error.set(Some("Invalid username or password".to_string()));
                }
                Some(Err(e)) => {
                    login_error.set(Some(format!("Login failed: {e}")));
                }
                _ => {
                    login_error.set(None);
                }
            }
        }
    });

    Effect::new(move || {
        if logged_in_user.get() == Some(true) {
            let nav = use_navigate();
            nav("/", NavigateOptions::default());
        }
    });

    view! {
        <leptos_meta::Title text="Log in"></leptos_meta::Title>
        <NavBar />
        <div class="bg-base-200 min-h-screen flex items-center justify-center p-4">
            <div class="card lg:card-side bg-base-100 shadow-xl max-w-4xl w-full">
                <figure class="lg:w-1/2 hidden lg:block">
                    <img
                        src="/images/login.webp"
                        alt="Login illustration"
                        class="object-cover w-full h-full"
                    />
                </figure>
                <div class="card-body lg:w-1/2">
                    <div class="space-y-6">
                        <ActionForm action=login on:submit=move |ev| ev.prevent_default()>
                            <h2 class="card-title text-2xl font-bold mb-6 text-center">Sign in to your account</h2>
                            {move || login_error.get().map(|error| view! {
                                <div role="alert" class="alert alert-error">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                                    <span>{error}</span>
                                </div>
                            })}
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
                                        placeholder="Enter username"
                                        required
                                        on:input=move |ev| {
                                            username.set(event_target_value(&ev));
                                        }
                                        class="grow"
                                    />
                                </label>
                                {move || {
                                    let username_value = username.get();
                                    if !username_value.is_empty() {
                                        if username_value.len() < USERNAME_LENGTH_MINIMUM {
                                            Some(view! {
                                                <label class="label">
                                                    <span class="label-text-alt text-error">{format!("Username must be at least {USERNAME_LENGTH_MINIMUM} characters")}</span>
                                                </label>
                                            })
                                        } else if username_value.len() > USERNAME_LENGTH_MAXIMUM {
                                            Some(view! {
                                                <label class="label">
                                                    <span class="label-text-alt text-error">{format!("Username must be at most {USERNAME_LENGTH_MAXIMUM} characters")}</span>
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
                                <div class="label">
                                    <span class="label-text">Password</span>
                                    <a href="#" class="label-text-alt link link-hover">Forgot password?</a>
                                </div>
                                <label class="input input-bordered flex items-center gap-2">
                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="w-4 h-4 opacity-70">
                                        <path fill-rule="evenodd" d="M14 6a4 4 0 0 1-4.899 3.899l-1.955 1.955a.5.5 0 0 1-.353.146H5v1.5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1-.5-.5v-2.293a.5.5 0 0 1 .146-.353l3.955-3.955A4 4 0 1 1 14 6Zm-4-2a.75.75 0 0 0 0 1.5.5.5 0 0 1 .5.5.75.75 0 0 0 1.5 0 2 2 0 0 0-2-2Z" clip-rule="evenodd" />
                                    </svg>
                                    <input
                                        id="password"
                                        name="password"
                                        type=move || pass_type.get()
                                        placeholder="Enter password"
                                        required
                                        on:input=move |ev| {
                                            password.set(event_target_value(&ev));
                                        }
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
                            <div class="form-control mt-6">
                                <input
                                    type="submit"
                                    value="Sign in"
                                    class="btn btn-primary"
                                    disabled={move || !is_valid_form()}
                                />
                            </div>
                        </ActionForm>
                    </div>
                    <div class="divider">OR</div>
                    <div class="text-center">
                        <p>Not a member?</p>
                        <a href="/signup" class="link link-primary">Register for an account</a>
                    </div>
                </div>
            </div>
        </div>
    }
}
