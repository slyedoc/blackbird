use crate::prelude::*;
use leptos::prelude::*;

#[component]
pub fn LoginPage(action: ServerAction<Login>) -> impl IntoView {
    view! {
      <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
        <div class="sm:mx-auto sm:w-full sm:max-w-sm">
          <img
            class="mx-auto h-10 w-auto"
            src="https://tailwindcss.com/plus-assets/img/logos/mark.svg?color=indigo&shade=600"
            alt="Your Company"
          />
          <h2 class="mt-10 text-center text-2xl/9 font-bold tracking-tight">
            Sign in to your account
          </h2>
        </div>

        <ActionForm action=action>

          <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
            <div class="space-y-6">
              <div>
                <label for="email" class="block text-sm/6 font-medium text">
                  Email address
                </label>
                <div class="mt-2">
                  <input type="text" name="username"  required class="block w-full bg-gray-500"
                  />
                </div>
              </div>

              <div>
                <div class="flex items-center justify-between">
                  <label for="password" class="block text-sm/6 font-medium text">
                    Password
                  </label>
                  <div class="text-sm">
                    <a
                      href="/todos"
                      class="font-semibold text-indigo-600 dark:text-indigo-400 hover:text-indigo-500 dark:hover:text-indigo-300"
                    >
                      Forgot password?
                    </a>
                  </div>
                </div>
                <div class="mt-2">
                  <input
                    type="password"
                    name="password"
                    id="password"
                    autocomplete="current-password"
                    required
                    class="block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6"
                  />
                </div>
              </div>
              <div>
                <label>
                  <input type="checkbox" name="remember" class="auth-input" />
                  "Remember me?"
                </label>
              </div>
              <div>
                <button
                  type="submit"
                  class="flex w-full justify-center rounded-md btn"
                >
                  Sign in
                </button>
              </div>
            </div>
          </div>
        </ActionForm>

        // social login
        <div class="mt-10">
          <div class="relative">
            <div class="absolute inset-0 flex items-center" aria-hidden="true">
              <div class="w-full border-t border-gray-200"></div>
            </div>
            <div class="relative flex justify-center text-sm/6 font-medium">
              <span class="bg-white px-6 text-gray-900">Or continue with</span>
            </div>
          </div>

          <div class="mt-6 grid grid-cols-2 gap-4">
            <a
              href="#"
              class="flex w-full items-center justify-center gap-3 rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 shadow-xs ring-gray-300 ring-inset hover:bg-gray-50 focus-visible:ring-transparent"
            >
            <Icon icon=i::AiGoogleOutlined class="h-5 w-5" />
              <span class="text-sm/6 font-semibold">Google</span>
            </a>

            <a
              href="/todos"
              class="flex w-full items-center justify-center gap-3 rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 shadow-xs ring-gray-300 ring-inset hover:bg-gray-50 focus-visible:ring-transparent"
            >
              <Icon icon=i::AiGithubOutlined class="h-5 w-5" />
              <span class="text-sm/6 font-semibold">GitHub</span>
            </a>
          </div>
        </div>
      </div>
    }
}
