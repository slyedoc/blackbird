use crate::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SignupPage(action: ServerAction<Signup>) -> impl IntoView {
    view! {
      <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
        <div class="sm:mx-auto sm:w-full sm:max-w-sm">
          <img
            class="mx-auto h-10 w-auto"
            src="https://tailwindcss.com/plus-assets/img/logos/mark.svg?color=indigo&shade=600"
            alt="Blackbird Logo"
          />
          <h2 class="mt-10 text-center text-2xl/9 font-bold tracking-tight">
            Sign up for an account
          </h2>
        </div>

        <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
          <ActionForm action=action>
            <div class="space-y-6">
              <div>
                <div>
                  <label for="email" class="block text-sm/6 font-medium text">
                    "Email"
                  </label>
                  <div class="mt-2">
                    <input
                      type="text"
                      maxlength="32"
                      name="email"
                      required
                      class="block w-full input-primary"
                    />
                  </div>
                </div>
                <div>
                  <label for="email" class="block text-sm/6 font-medium text">
                    "Username"
                  </label>
                  <div class="mt-2">
                    <input
                      type="text"
                      maxlength="32"
                      name="username"
                      required
                      class="block w-full input-primary"
                    />
                  </div>
                </div>

                <div>
                  <label for="password" class="block text-sm/6 font-medium text">
                    "Password"
                  </label>
                  <div class="mt-2">
                    <input type="password" name="password" class="block w-full input-primary" />
                  </div>
                </div>

                <div>
                  <label for="password_confirmation" class="block text-sm/6 font-medium text">
                    "Confirm Password"
                  </label>
                  <div class="mt-2">
                    <input
                      type="password"
                      name="password_confirmation"
                      class="block w-full input-primary"
                    />
                  </div>
                </div>

              </div>

              <div>
                <label>
                  <input type="checkbox" name="remember" class="mr-3" />
                  "Remember me?"
                </label>
              </div>
              <div>
                <button type="submit" class="flex w-full btn-primary">
                  "Sign Up"
                </button>
              </div>
            </div>
          </ActionForm>
          <p class="mt-10 text-center text-sm/6 text-gray-500 dark:text-gray-400">
            Already have an account?
            <a href="/login" class="font-semibold text-indigo-600 hover:text-indigo-500 dark:text-indigo-400 dark:hover:text-indigo-300">
              Login
            </a>
          </p>
        </div>
      </div>
    }
}
